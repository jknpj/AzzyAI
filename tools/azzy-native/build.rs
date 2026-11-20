// Embeds the Windows icon and version information into the executable, so Explorer and the file
// properties show the AzzyAI icon instead of a blank document. The window icon at runtime comes
// from data/azzyai-48.rgba; this is the resource Windows reads from the .exe itself.
fn main() {
    println!("cargo:rerun-if-changed=data/azzyai.ico");
    println!("cargo:rerun-if-changed=../../USER_AI/Const_.lua");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    #[cfg(windows)]
    {
        let version = cversion().unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());
        let mut res = winres::WindowsResource::new();
        res.set_icon("data/azzyai.ico");
        res.set("ProductName", "AzzyAI");
        res.set("FileDescription", "AzzyAI Configuration");
        res.set("LegalCopyright", "GPL-3.0-or-later");
        res.set("FileVersion", &version);
        res.set("ProductVersion", &version);
        res.set_language(0x0409);
        if let Err(error) = res.compile() {
            println!("cargo:warning=could not embed the Windows icon: {error}");
        }
    }
}

#[cfg(windows)]
fn cversion() -> Option<String> {
    let text = std::fs::read_to_string("../../USER_AI/Const_.lua").ok()?;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("CVersion=") {
            let rest = rest.trim().trim_start_matches('"');
            let end = rest.find('"').unwrap_or(rest.len());
            let version = rest[..end].trim();
            if !version.is_empty() {
                return Some(version.to_string());
            }
        }
    }
    None
}
