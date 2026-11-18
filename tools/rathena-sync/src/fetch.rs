// fetch.rs: HTTP for rathena-sync. The GitHub raw/contents/commits endpoints, a proxy honored
// through HTTPS_PROXY (reqwest reads it), and the output paths of this repository.
use serde_json::Value as Json;
use std::path::PathBuf;
use std::time::Duration;

pub struct Client {
    inner: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Result<Self, String> {
        let inner = reqwest::blocking::Client::builder()
            .user_agent("AzzyAI-harness")
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| format!("HTTP client: {e}"))?;
        Ok(Self { inner })
    }

    fn get(&self, url: &str) -> Result<String, String> {
        let resp = self.inner.get(url).send().map_err(|e| format!("{url}: {e}"))?;
        let status = resp.status();
        let body = resp.text().map_err(|e| format!("{url}: {e}"))?;
        if !status.is_success() {
            return Err(format!("{url}: HTTP {}", status.as_u16()));
        }
        Ok(body)
    }

    pub fn raw(&self, sha: &str, path: &str) -> Result<String, String> {
        self.get(&format!("https://raw.githubusercontent.com/rathena/rathena/{sha}/{path}"))
    }

    pub fn contents(&self, sha: &str, path: &str) -> Result<Vec<Json>, String> {
        let text = self.get(&format!(
            "https://api.github.com/repos/rathena/rathena/contents/{path}?ref={sha}"
        ))?;
        serde_json::from_str(&text).map_err(|e| format!("contents {path}: {e}"))
    }

    pub fn resolve_ref(&self, reference: &str) -> Result<String, String> {
        let text = self.get(&format!(
            "https://api.github.com/repos/rathena/rathena/commits/{reference}"
        ))?;
        let json: Json = serde_json::from_str(&text).map_err(|e| format!("resolve {reference}: {e}"))?;
        json.get("sha")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| format!("resolve {reference}: no sha in the answer"))
    }
}

pub fn today() -> String {
    // UTC date, like datetime.date.today() in the repository's CI
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86400;
    civil_from_days(days as i64)
}

fn civil_from_days(z: i64) -> String {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

pub fn repo_root() -> PathBuf {
    // the binary lives in tools/rathena-sync/target[/debug], the outputs in the repo root
    let mut dir = std::env::current_exe().map_err(|_| ()).unwrap().canonicalize().unwrap();
    for _ in 0..5 {
        dir.pop();
        if dir.join("tools").join("rathena-sync").is_dir() || dir.join("USER_AI").is_dir() {
            return dir;
        }
    }
    PathBuf::from(".")
}

pub fn fixture_path() -> PathBuf {
    repo_root().join("tools/harness/fixtures/rathena.lua")
}

pub fn mobinfo_path() -> PathBuf {
    repo_root().join("USER_AI/MobInfo.lua")
}

pub fn webgui_path() -> PathBuf {
    repo_root().join("webgui")
}
