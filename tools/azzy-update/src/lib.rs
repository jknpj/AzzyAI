//! The "Update AI from GitHub" flow shared by the native GUIs and the local web GUI host.
//!
//! Downloads the main-branch zip, keeps the files the player edits, replaces the rest, backs up
//! what it replaces and stages a newer executable as `.exe.new` (a running exe cannot be
//! overwritten on Windows).

use std::collections::BTreeSet;
use std::io::Read;

use std::path::{Path, PathBuf};

/// Downloads an update archive. The caller decides the URL (the shipped one is the `main` branch
/// zip of the repository).
pub fn download(url: &str) -> Result<Vec<u8>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("azzy-update")
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;
    let response = client.get(url).send().map_err(|e| format!("{url}: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("{url}: HTTP {}", response.status().as_u16()));
    }
    response
        .bytes()
        .map(|bytes| bytes.to_vec())
        .map_err(|e| format!("{url}: {e}"))
}

#[cfg(test)]
mod download_tests {
    #[test]
    fn rejects_a_local_url_that_is_not_http() {
        // The network path itself is covered by the host/binaries by hand; here we only pin that a
        // bad URL comes back as an error instead of panicking.
        assert!(super::download("not a url").is_err());
    }
}

/// Files the player edits: never overwritten, only created when missing.
const USER_FILES: [&str; 11] = [
    "H_Config.lua", "M_Config.lua", "H_Tactics.lua", "M_Tactics.lua", "H_PVP_Tact.lua",
    "M_PVP_Tact.lua", "H_Extra.lua", "M_Extra.lua", "H_Avoid.lua", "A_Friends.lua", "Mob_ID.lua",
];

#[derive(Debug)]
pub struct Report {
    pub updated: usize,
    pub added: usize,
    pub kept: usize,
    pub staged: Vec<String>,
    pub backup: Option<String>,
}

impl Report {
    pub fn to_json(&self) -> String {
        let staged: Vec<String> = self.staged.iter().map(|s| format!("\"{}\"", json_escape(s))).collect();
        format!(
            "{{\"updated\":{},\"added\":{},\"kept\":{},\"staged\":[{}],\"backup\":{}}}",
            self.updated,
            self.added,
            self.kept,
            staged.join(","),
            match &self.backup {
                Some(dir) => format!("\"{}\"", json_escape(dir)),
                None => "null".to_string(),
            }
        )
    }
}

fn json_escape(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

pub fn is_user_file(relative: &str) -> bool {
    let name = relative.rsplit('/').next().unwrap_or(relative);
    USER_FILES.contains(&name)
}

/// Applies the zip at `archive` (bytes) over `user_ai`, which must exist.
/// `overwrite_presets` decides Types/*.lua: the Windows app keeps them unless asked, because they
/// hold the per-homunculus tuning the player may have edited.
pub fn apply_zip(archive: &[u8], user_ai: &Path, overwrite_presets: bool) -> Result<Report, String> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(archive)).map_err(|e| format!("zip: {e}"))?;
    let mut report = Report { updated: 0, added: 0, kept: 0, staged: Vec::new(), backup: None };
    let mut backup_dir: Option<PathBuf> = None;
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).map_err(|e| format!("zip entry: {e}"))?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().to_string();
        // GitHub's archive has one root folder: <repo>-<branch>/USER_AI/...
        let Some(relative) = name.split_once("/USER_AI/").map(|(_, rest)| rest.to_string()) else {
            continue;
        };
        if relative.is_empty() || relative.contains("..") || relative.starts_with('/') {
            continue;
        }
        seen.insert(relative.clone());
        let target = user_ai.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        let preset = relative.starts_with("Types/") && !overwrite_presets;
        if is_user_file(&relative) || preset {
            if target.is_file() {
                report.kept += 1;
                continue;
            }
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
            }
            copy_entry(&mut entry, &target)?;
            report.added += 1;
            continue;
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        let mut data = Vec::new();
        entry.read_to_end(&mut data).map_err(|e| format!("{}: {e}", relative))?;
        if relative.to_lowercase().ends_with(".exe") && target.is_file() {
            let staged = PathBuf::from(format!("{}.new", target.display()));
            std::fs::write(&staged, &data).map_err(|e| format!("{}: {e}", staged.display()))?;
            report.staged.push(relative);
            continue;
        }
        let existing = std::fs::read(&target).ok();
        if existing.as_deref() == Some(data.as_slice()) {
            report.kept += 1;
            continue;
        }
        if existing.is_some() {
            if backup_dir.is_none() {
                let stamp = timestamp();
                let dir = user_ai.join("backup").join(format!("update-{stamp}"));
                std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
                backup_dir = Some(dir.clone());
                report.backup = Some(dir.display().to_string());
            }
            if let Some(dir) = &backup_dir {
                let backup = dir.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
                if let Some(parent) = backup.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::copy(&target, &backup);
            }
            report.updated += 1;
        } else {
            report.added += 1;
        }
        std::fs::write(&target, &data).map_err(|e| format!("{}: {e}", target.display()))?;
    }
    if seen.is_empty() {
        return Err("the archive has no USER_AI folder (wrong URL or branch?)".into());
    }
    Ok(report)
}

fn copy_entry<R: std::io::Read>(entry: &mut zip::read::ZipFile<R>, target: &Path) -> Result<(), String> {
    let mut file = std::fs::File::create(target).map_err(|e| format!("{}: {e}", target.display()))?;
    std::io::copy(entry, &mut file).map_err(|e| format!("{}: {e}", target.display()))?;
    Ok(())
}

fn timestamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn sample_zip() -> Vec<u8> {
        let mut buffer = std::io::Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut buffer);
            let options = zip::write::SimpleFileOptions::default();
            zip.start_file("AzzyAI-main/USER_AI/MobInfo.lua", options).unwrap();
            zip.write_all(b"-- new mob info\n").unwrap();
            zip.start_file("AzzyAI-main/USER_AI/H_Config.lua", options).unwrap();
            zip.write_all(b"LavaSlideMode = 9\n").unwrap(); // must be kept, the player edits it
            zip.start_file("AzzyAI-main/USER_AI/AzzyAIConfig.exe", options).unwrap();
            zip.write_all(b"fake exe").unwrap();
            zip.start_file("AzzyAI-main/USER_AI/Types/Dieter.lua", options).unwrap();
            zip.write_all(b"AutoMobCount = 1\n").unwrap();
            zip.start_file("AzzyAI-main/README.md", options).unwrap();
            zip.write_all(b"ignored\n").unwrap();
            zip.finish().unwrap();
        }
        buffer.into_inner()
    }

    #[test]
    fn keeps_user_files_and_stages_the_exe() {
        let dir = std::env::temp_dir().join(format!("azzy-gui-update-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Types")).unwrap();
        std::fs::write(dir.join("H_Config.lua"), "LavaSlideMode = 1\n").unwrap();
        std::fs::write(dir.join("MobInfo.lua"), "-- old mob info\n").unwrap();
        std::fs::write(dir.join("Types").join("Dieter.lua"), "AutoMobCount = 2\n").unwrap();
        std::fs::write(dir.join("AzzyAIConfig.exe"), "old exe").unwrap();

        let report = apply_zip(&sample_zip(), &dir, false).unwrap();

        assert_eq!(report.kept, 2, "H_Config.lua and Types/Dieter.lua are kept by default");
        assert_eq!(report.updated, 1, "MobInfo.lua is a shipped file");
        assert_eq!(report.added, 0);
        assert_eq!(report.staged, vec!["AzzyAIConfig.exe".to_string()]);
        assert_eq!(std::fs::read_to_string(dir.join("H_Config.lua")).unwrap(), "LavaSlideMode = 1\n");
        assert_eq!(std::fs::read_to_string(dir.join("MobInfo.lua")).unwrap(), "-- new mob info\n");
        assert_eq!(
            std::fs::read_to_string(dir.join("Types").join("Dieter.lua")).unwrap(),
            "AutoMobCount = 2\n",
            "the preset is kept unless --overwrite-presets"
        );
        assert_eq!(std::fs::read_to_string(dir.join("AzzyAIConfig.exe.new")).unwrap(), "fake exe");
        assert!(report.backup.is_some(), "the replaced file must be backed up");
        let backup = report.backup.unwrap();
        assert!(std::path::Path::new(&backup).join("MobInfo.lua").is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn presets_are_replaced_when_asked() {
        let dir = std::env::temp_dir().join(format!("azzy-gui-presets-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Types")).unwrap();
        std::fs::write(dir.join("Types").join("Dieter.lua"), "AutoMobCount = 2\n").unwrap();
        let report = apply_zip(&sample_zip(), &dir, true).unwrap();
        assert_eq!(report.updated, 1, "Types/Dieter.lua is replaced");
        assert_eq!(report.added, 3, "MobInfo.lua, H_Config.lua and the exe did not exist here");
        assert_eq!(std::fs::read_to_string(dir.join("Types").join("Dieter.lua")).unwrap(), "AutoMobCount = 1\n");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn archive_without_user_ai_is_rejected() {
        let mut buffer = std::io::Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut buffer);
            zip.start_file("AzzyAI-main/README.md", zip::write::SimpleFileOptions::default()).unwrap();
            zip.write_all(b"x").unwrap();
            zip.finish().unwrap();
        }
        let err = apply_zip(&buffer.into_inner(), std::path::Path::new("."), false).unwrap_err();
        assert!(err.contains("no USER_AI folder"), "{err}");
    }
}
