use std::error::Error;

pub async fn set_from_path(path: &str) -> Result<(), Box<dyn Error>> {
    wallpaper::set_from_path(path)?;
    Ok(())
}

pub async fn show_desktop() -> Result<(), Box<dyn Error>> {
    Ok(())
}
