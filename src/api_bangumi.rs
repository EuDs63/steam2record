use std::collections::HashMap;

use reqwest;
use serde_json::{Value, json};
use tokio::test;

pub async fn search(name:&str) -> Result<String,reqwest::Error>{
    let url = "https://api.bgm.tv/v0/search/subjects?limit=1";

    let body = json!({
        "keyword": name,
        "sort": "rank",
        "filter": {
            "type": [4],
            "nsfw": false
        }
    });


    let client = reqwest::Client::new();
    //发送请求
    let res = client
        .post(url)
        .header("User-Agent", "EuDs63/steam2record")
        .json(&body)
        .send()
        .await?;
    // 解析请求
    let body:Value = res.text().await?.parse().unwrap();
    //println!("{}",body);
    let id = body["data"][0]["id"].to_string();
    Ok(id)
}

pub async fn mark(id:String,token:String,shelf_type:i32) -> Result<bool,reqwest::Error>{
    let url = format!("https://api.bgm.tv/v0/users/-/collections/{}",id);
    let authorization_header = format!("Bearer {}",token);

    let body = json!({
        "type": shelf_type, //1:想玩，2：玩过，3：在玩
        "private": false
    });

    let client = reqwest::Client::new();
    //发送请求
    let res = client
        .post(&url)
        .header("User-Agent", "EuDs63/steam2record")
        .header("Authorization",authorization_header)
        .json(&body)
        .send()
        .await?;

    //println!("{}",res.text().await?);
    // 获取响应状态码
    let status_code = res.status();

    // 根据状态码进行判断
    if status_code.is_success() {
        println!("Request was successful! Status code: {}", status_code);
        return Ok(true);
    } else {
        println!("Request failed! Status code: {}", status_code);
        println!("Response is {}",res.text().await?);
        return Ok(false);
    }

}

pub async fn operate(name:&str,hours:&str,token:String) -> Result<bool,reqwest::Error>{
    // 搜索得到id
    let id = search(name).await.unwrap();

    //对搜索到的id进行判断
    if id == "null" {
        // 红色字体
        println!("\x1b[31m{} not found\x1b[0m", name);
        return Ok(false);
    } else {
        // 根据游戏时间进行判断
        let mut shelf_type = 2;
        
        // 无时间数据则判断为想玩
        if hours.is_empty() {
            println!("{} hasn't played", name);
            shelf_type = 1;
        }
        //开始标记
        println!("try to mark {} on bangumi ", name);
        let result = mark(id, token, shelf_type).await.unwrap();
        if result {
            // 绿色字体
            println!("\x1b[32m{} mark on bangumi success\x1b[0m", name);
            return Ok(true);
        } else {
            // 红色字体
            println!("\x1b[31m{} mark on bangumi failed\x1b[0m", name);
            return Ok(false);
        }
    }
}

#[test]
async fn test_search_function(){
    let name = "Squirrelmageddon!";
    let id  = search(name).await.unwrap();
    assert_eq!(id,"null".to_string());
}

#[test]
async fn test_mark_function(){
    let id = "10468".to_string();
    let config = crate::app_config::AppConfig::from_file("config.toml").unwrap();
    let token = config.bangumi_token;
    let shelf_type = 2;
    let result = mark(id,token,shelf_type).await.unwrap();
    assert!(result);
}