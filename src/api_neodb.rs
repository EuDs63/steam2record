use reqwest;
use serde_json::Value;
use tokio::test;

// let base_url = "https://neodb.social/api/";

pub async fn search(name:&str) -> Result<(), reqwest::Error> {
    let url = format!("https://neodb.social/api/catalog/search?query={}&category=game&page=1", name);

    let response = reqwest::get(url).await?;

    // 检查响应状态码
    if response.status().is_success() {
        // 解析 JSON 响应
        let body: Value = response.text().await?.parse().unwrap();
        
        // 提取 "url" 字段
        if let Some(url) = body["data"][0]["url"].as_str() {
            println!("URL: {}", url);
        } else {
            eprintln!("Error: Could not extract 'url' field from JSON");
        }
    } else {
        eprintln!("API Request Failed: {}", response.status());
    }

    Ok(())
}

#[test]
async fn test_search_function() {
    let name = "Wallpaper Engine";
    search(name).await.unwrap();
}
