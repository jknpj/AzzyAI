//! The "Update AI from GitHub" flow shared by the native GUIs and the local web GUI host.
//!
//! Downloads the main-branch zip, keeps the files the player edits, replaces the rest, backs up
//! what it replaces and stages a newer executable as `.exe.new` (a running exe cannot be
//! overwritten on Windows).

use std::collections::BTreeSet;
use std::io::Read;

use std::path::{Path, PathBuf};

/// Loads an update archive. The caller decides the source (the shipped one is the `main` branch
/// zip of the repository). A plain path to a local zip also works, which is handy for testing a
/// packaged build without publishing it.
pub fn download(url: &str) -> Result<Vec<u8>, String> {
    let trimmed = url.trim();
    if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
        let path = Path::new(trimmed);
        return std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()));
    }
    let url = trimmed;
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
    fn a_local_zip_path_is_read_directly() {
        let path = std::env::temp_dir().join(format!("azzy-update-local-{}.zip", std::process::id()));
        std::fs::write(&path, b"PK fake").unwrap();
        assert_eq!(super::download(path.to_str().unwrap()).unwrap(), b"PK fake");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_missing_path_or_bad_url_is_an_error() {
        // The network path itself is covered by the host/binaries by hand; here we only pin that a
        // bad source comes back as an error instead of panicking.
        assert!(super::download("not a url").is_err());
        assert!(super::download("/nonexistent/azzy-update.zip").is_err());
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

/// Tag prefix of this product's releases (`main` keeps the plain `vX.Y.Z`).
pub const RELEASE_PREFIX: &str = "pre-rev";

/// The tag of the latest release of this product. Only tags carrying RELEASE_PREFIX are
/// considered, so the Renewal releases on the same repository are never offered here; `None`
/// when there is none.
pub fn latest_release_tag(repo: &str) -> Result<Option<String>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("azzy-native")
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|e| e.to_string())?;
    let pick = |body: &str| -> Option<String> {
        let value: serde_json::Value = serde_json::from_str(body).ok()?;
        value
            .as_array()?
            .iter()
            .filter_map(|e| {
                e.get("tag_name")
                    .or_else(|| e.get("name"))
                    .and_then(|v| v.as_str())
            })
            .find(|tag| tag.starts_with(RELEASE_PREFIX))
            .map(str::to_string)
    };
    let releases = format!("https://api.github.com/repos/{repo}/releases?per_page=50");
    let response = client
        .get(&releases)
        .send()
        .map_err(|e| format!("{releases}: {e}"))?;
    if response.status().is_success() {
        let body = response.text().map_err(|e| format!("{releases}: {e}"))?;
        if let Some(tag) = pick(&body) {
            return Ok(Some(tag));
        }
    }
    let tags = format!("https://api.github.com/repos/{repo}/tags?per_page=100");
    let response = client.get(&tags).send().map_err(|e| format!("{tags}: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("{tags}: HTTP {}", response.status().as_u16()));
    }
    let body = response.text().map_err(|e| format!("{tags}: {e}"))?;
    Ok(pick(&body))
}

/// True when `latest` (a tag such as `v2.0.2`) sorts after `installed` (`2.0.1`). Anything that is
/// not `MAJOR.MINOR.PATCH` numbers falls back to a plain string comparison of the numeric parts.
pub fn newer_version(latest: &str, installed: &str) -> bool {
    fn parts(text: &str) -> Vec<u64> {
        text.trim()
            .trim_start_matches(RELEASE_PREFIX)
            .trim_start_matches(['v', 'V'])
            .split(['.', '-', '+'])
            .take(3)
            .map(|part| part.parse::<u64>().unwrap_or(0))
            .collect()
    }
    let latest = parts(latest);
    let installed = parts(installed);
    if latest.is_empty() || installed.is_empty() {
        return false;
    }
    for index in 0..3 {
        let a = latest.get(index).copied().unwrap_or(0);
        let b = installed.get(index).copied().unwrap_or(0);
        if a != b {
            return a > b;
        }
    }
    false
}

/// A binary from the archive by file name, wherever the archive keeps it: `tools/azzy-native/`
/// while the native window is still developed next to the C# one, `USER_AI/` once it replaces
/// `AzzyAIConfig.exe` and ships beside the AI files. `None` when the archive does not ship it.
pub fn find_binary(archive: &[u8], file_name: &str) -> Result<Option<Vec<u8>>, String> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(archive)).map_err(|e| format!("zip: {e}"))?;
    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).map_err(|e| format!("zip entry: {e}"))?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().rsplit('/').next().unwrap_or(entry.name());
        if !name.eq_ignore_ascii_case(file_name) {
            continue;
        }
        let mut data = Vec::new();
        entry
            .read_to_end(&mut data)
            .map_err(|e| format!("{}: {e}", entry.name()))?;
        return Ok(Some(data));
    }
    Ok(None)
}

/// Writes `bytes` next to `current` as `<name>.new` when they differ, so a running executable can
/// be replaced on the next launch. Returns the staged path, or `None` when it is already current.
pub fn stage_binary(bytes: &[u8], current: &Path) -> Result<Option<PathBuf>, String> {
    if let Ok(existing) = std::fs::read(current) {
        if existing == bytes {
            return Ok(None);
        }
    }
    let target = PathBuf::from(format!("{}.new", current.display()));
    std::fs::write(&target, bytes).map_err(|e| format!("{}: {e}", target.display()))?;
    Ok(Some(target))
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
            zip.start_file("AzzyAI-main/USER_AI/Types/Amistr.lua", options).unwrap();
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
        std::fs::write(dir.join("Types").join("Amistr.lua"), "AutoMobCount = 2\n").unwrap();
        std::fs::write(dir.join("AzzyAIConfig.exe"), "old exe").unwrap();

        let report = apply_zip(&sample_zip(), &dir, false).unwrap();

        assert_eq!(report.kept, 2, "H_Config.lua and Types/Amistr.lua are kept by default");
        assert_eq!(report.updated, 1, "MobInfo.lua is a shipped file");
        assert_eq!(report.added, 0);
        assert_eq!(report.staged, vec!["AzzyAIConfig.exe".to_string()]);
        assert_eq!(std::fs::read_to_string(dir.join("H_Config.lua")).unwrap(), "LavaSlideMode = 1\n");
        assert_eq!(std::fs::read_to_string(dir.join("MobInfo.lua")).unwrap(), "-- new mob info\n");
        assert_eq!(
            std::fs::read_to_string(dir.join("Types").join("Amistr.lua")).unwrap(),
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
        std::fs::write(dir.join("Types").join("Amistr.lua"), "AutoMobCount = 2\n").unwrap();
        let report = apply_zip(&sample_zip(), &dir, true).unwrap();
        assert_eq!(report.updated, 1, "Types/Amistr.lua is replaced");
        assert_eq!(report.added, 3, "MobInfo.lua, H_Config.lua and the exe did not exist here");
        assert_eq!(std::fs::read_to_string(dir.join("Types").join("Amistr.lua")).unwrap(), "AutoMobCount = 1\n");
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn sample_zip_with_binary(path: &str) -> Vec<u8> {
        let mut buffer = std::io::Cursor::new(Vec::new());
        {
            let mut zip = zip::ZipWriter::new(&mut buffer);
            let options = zip::write::SimpleFileOptions::default();
            zip.start_file("AzzyAI-main/USER_AI/MobInfo.lua", options).unwrap();
            zip.write_all(b"-- new mob info\n").unwrap();
            zip.start_file(path, options).unwrap();
            zip.write_all(b"new native exe").unwrap();
            zip.finish().unwrap();
        }
        buffer.into_inner()
    }

    #[test]
    fn finds_the_binary_by_name_in_either_layout() {
        // development layout, next to the C# one
        let archive = sample_zip_with_binary("AzzyAI-main/tools/azzy-native/azzy-native.exe");
        let bytes = find_binary(&archive, "azzy-native.exe")
            .unwrap()
            .expect("binary in the archive");
        assert_eq!(bytes, b"new native exe");
        assert!(find_binary(&archive, "azzy-gui.exe").unwrap().is_none());

        // shipped layout: the native window replaces the C# GUI inside USER_AI
        let archive = sample_zip_with_binary("AzzyAI-main/USER_AI/azzy-native.exe");
        let bytes = find_binary(&archive, "AZZY-NATIVE.EXE")
            .unwrap()
            .expect("the lookup is case-insensitive, like Windows");
        assert_eq!(bytes, b"new native exe");
    }

    #[test]
    fn stages_a_binary_next_to_the_running_one() {
        let archive = sample_zip_with_binary("AzzyAI-main/USER_AI/azzy-native.exe");
        let bytes = find_binary(&archive, "azzy-native.exe").unwrap().unwrap();

        let dir = std::env::temp_dir().join(format!("azzy-update-exe-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let exe = dir.join("azzy-native.exe");
        std::fs::write(&exe, b"old").unwrap();
        let staged = stage_binary(&bytes, &exe).unwrap().expect("must stage");
        assert_eq!(std::fs::read(&staged).unwrap(), b"new native exe");
        assert!(stage_binary(&bytes, &staged).unwrap().is_none(), "same content is not staged");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compares_release_tags() {
        assert!(newer_version("v2.0.2", "2.0.1"));
        assert!(newer_version("v2.1", "2.0.9"));
        assert!(newer_version("3.0.0", "2.9.9"));
        assert!(!newer_version("v2.0.1", "2.0.1"));
        assert!(!newer_version("v2.0.0", "2.0.1"));
        assert!(!newer_version("", "2.0.1"));
        assert!(newer_version("v2.0.10", "2.0.9"));
    }

    #[test]
    #[ignore = "network"]
    fn reads_the_latest_release_tag_from_github() {
        let tag = latest_release_tag("jknpj/AzzyAI").unwrap();
        assert!(tag.is_some(), "the repository has releases");
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
