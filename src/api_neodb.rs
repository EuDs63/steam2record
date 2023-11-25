use std::{collections::HashMap};

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
    let res = body["data"][0]["uuid"].to_string();
    Ok(res)
}

pub async fn mark(uuid: String, token: String, shelf_type: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://neodb.social/api/me/shelf/item/{}", uuid);
    let authorization_header = format!("Bearer {}", token);

    let mut body = HashMap::new();
    body.insert("shelf_type", shelf_type);
    body.insert("visibility", "0"); // 0:公开，1:仅关注者，2:仅自己
    body.insert("post_to_fediverse", "false");

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .header("Authorization", authorization_header)
        .json(&body)
        .send()
        .await?;

    let body = res.text().await?;
    let res:Value = body.parse().unwrap();
    let res = res["message"].to_string();
    Ok(res)
}

#[test]
async fn test_search_function() {
    let name = "Squirrelmageddon!";
    let uuid  = search(name).await.unwrap();
    println!("uuid is {}",uuid);
    assert_eq!(uuid,"null".to_string());
}

#[test]
async fn test_mark_function(){
    let uuid = "4ZIvmft6PYfyWsZwZWZpAl".to_string();
    let config = crate::AppConfig::AppConfig::from_file("config.toml").unwrap();
    let token = config.neodb_token;
    let shelf_type = "complete";
    let result = mark(uuid, token, shelf_type).await.unwrap();
    assert_eq!(result,"\"OK\"".to_string());
}
