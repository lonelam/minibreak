use std::error::Error;
use std::process::Command;

pub async fn set_from_path(path: &str) -> Result<(), Box<dyn Error>> {
    wallpaper::set_from_path(path)?;
    Ok(())
}

pub async fn show_desktop() -> Result<(), Box<dyn Error>> {
    let script =
        r#"tell application "System Events" to set visible of every application process to false"#;
    Command::new("osascript").arg("-e").arg(script).output()?;

    Ok(())
}
