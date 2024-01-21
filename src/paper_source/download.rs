use std::io;
use std::path::Path;
use std::{error::Error, fs::File, io::Write, path::PathBuf};
use url::Url;

async fn extract_file_name_from_url(url: &str) -> Result<String, &'static str> {
    // Parse the URL
    let parsed_url = Url::parse(url).map_err(|_| "Invalid URL")?;

    // Try to extract the file name from the query parameter 'id'
    if let Some(query_pairs) = parsed_url.query_pairs().find(|(key, _)| key == "id") {
        let (_, file_name) = query_pairs;
        return Ok(file_name.to_string());
    }

    // Fallback: Use the last segment of the path
    let path = parsed_url.path();
    let file_name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("No file name found in URL")?;

    Ok(file_name.to_string())
}

pub async fn download_url_to_wallpaper_dir(url: String) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let server_file_name: String = response.headers().get("Content-Disposition").map_or(
            String::from(format!(
                "wallpaper-{}.jpg",
                extract_file_name_from_url(&url).await?
            )),
            |value: &reqwest::header::HeaderValue| -> String {
                value.to_str().unwrap().to_string()
            },
        );
        let mut target_path = PathBuf::from(dirs::picture_dir().unwrap());
        target_path.push(server_file_name);

        let mut file = File::create(&target_path)?;
        let content = response.bytes().await?;
        file.write_all(&content)?;
        Ok(target_path.display().to_string())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Failed to download the file",
        )))
    }
}
