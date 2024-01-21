use paper_source::bing::WallpaperResolution;

mod paper_source;
mod wallpaper_fast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallpaper_url =
        paper_source::bing::get_latest_bing_wallpaper_url(WallpaperResolution::W1920).await?;
    println!("The latest BING wallpaper url is: {}", wallpaper_url);
    let wallpaper_path =
        paper_source::download::download_url_to_wallpaper_dir(wallpaper_url).await?;
    wallpaper_fast::set_from_path(&wallpaper_path).await?;

    wallpaper_fast::show_desktop().await?;

    Ok(())
}
