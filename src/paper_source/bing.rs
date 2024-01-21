use std::error::Error;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum WallpaperResolution {
    W3840 = 3840,
    W1920 = 1920,
    W1366 = 1366,
}

#[derive(Serialize, Deserialize)]
pub struct IBingRequest {
    resolution: WallpaperResolution,
    format: String,
    index: i32,
    mkt: String,
}

#[derive(Deserialize, Serialize)]
pub struct IBingResponse {
    pub url: String,
}

pub async fn get_latest_bing_wallpaper_url(
    resolution: WallpaperResolution,
) -> Result<String, Box<dyn Error>> {
    let client: reqwest::Client = reqwest::Client::new();
    let builder = client.request(Method::GET, "https://bing.biturl.top/");
    let builder = builder.query(&IBingRequest {
        resolution,
        format: String::from("json"),
        index: 0,
        mkt: String::from("zh-CN"),
    });
    let response = builder.send().await?;

    let response_text = response.text().await?;

    let response_body: IBingResponse = serde_json::from_str(&response_text)?;
    Ok(response_body.url)
}
