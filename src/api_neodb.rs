use std::collections::HashMap;

use reqwest;
use serde_json::Value;
use tokio::test;

pub async fn search(name:&str) -> Result<String, reqwest::Error>{
    // 构造请求url
    let url = format!("https://neodb.social/api/catalog/search?query={}&category=game&page=1", name);
    // 发送请求
    let body = reqwest::get(&url).await?.text().await?;
    // 解析请求为json
    let body: Value = body.parse().unwrap();
    // 尝试获取uuid
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
    // 发送请求
    let res = client
        .post(&url)
        .header("Authorization", authorization_header)
        .json(&body)
        .send()
        .await?;
    // 解析请求为json
    let body = res.text().await?;
    let res:Value = body.parse().unwrap();
    let res = res["message"].to_string();
    Ok(res)
}

pub async fn operate(name: &str, hours: &str, token: String) -> Result<bool, reqwest::Error> {
    // 搜索得到uuid
    let uuid = search(name).await.unwrap();

    //对搜索到的uuid进行判断
    if uuid == "null" {
        // 红色字体
        println!("\x1b[31m{} not found\x1b[0m", name);
        return Ok(false);
    } else {
        // 根据游戏时间进行判断,默认为玩过
        let mut shelf_type = "complete";

        //无时间数据则判断为想玩
        if hours.is_empty() {
            println!("{} hasn't played", name);
            shelf_type = "wishlist";
        }
        //开始标记
        println!("try to mark {} on neodb ", name);
        //api_neodb::mark(uuid, config.neodb_token.to_string(), shelf_type).await.unwrap();
        let result = mark(uuid, token, shelf_type).await.unwrap();
        if result == "\"OK\"" {
            // 绿色字体
            println!("\x1b[32m{} mark on neodb success\x1b[0m", name);
            return Ok(true);
        } else {
            // 红色字体
            println!("\x1b[31m{} mark on neodb failed\x1b[0m", name);
            return Ok(false);
        }
    }
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
    let config = crate::app_config::AppConfig::from_file("config.toml").unwrap();
    let token = config.neodb_token;
    let shelf_type = "complete";
    let result = mark(uuid, token, shelf_type).await.unwrap();
    assert_eq!(result,"\"OK\"".to_string());
}
