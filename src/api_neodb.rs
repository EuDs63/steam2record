use std::f32::consts::E;

use reqwest::{self, Error};
use serde_json::Value;
use tokio::test;

// let base_url = "https://neodb.social/api/";

// pub async fn search(name: &str) -> Result<String, reqwest::Error> {
//     let url = format!("https://neodb.social/api/catalog/search?query={}&category=game&page=1", name);

//     let response = reqwest::get(&url).await?;

//     // 检查响应状态码
//     if response.status().is_success() {
//         // 解析 JSON 响应
//         let body: Value = response.text().await?.parse().unwrap();
        
//         // 提取 "url" 字段
//         if let Some(url) = body["data"][0]["url"].as_str() {
//             return Ok(url.to_string());
//         } else {
//             return Err(reqwest::Error::builder()
//                 .status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
//                 .source(Box::new(reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "Could not extract 'url' field from JSON")))
//                 .build());
//         }
//     } else {
//         return Err(reqwest::Error::builder()
//             .status(response.status())
//             .source(Box::new(reqwest::Error::new(response.status(), format!("API Request Failed: {}", response.status()))))
//             .build());
//     }
// }

pub async fn search(name:&str) -> Result<String, reqwest::Error>{
    let url = format!("https://neodb.social/api/catalog/search?query={}&category=game&page=1", name);
    let body = reqwest::get(&url).await?.text().await?;
    let body: Value = body.parse().unwrap();
    let res = body["data"][0]["url"].to_string();
    Ok(res)
}

#[test]
async fn test_search_function() {
    let name = "Wallpaper Engine";
    search(name).await.unwrap();
}
