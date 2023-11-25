use csv::ReaderBuilder;
use std::{error::Error, fs::File};
use tokio;

mod AppConfig;
mod api_neodb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::AppConfig::from_file("config.toml").unwrap();
    //println!("neodb_token is {}", config.neodb_token);

    let csv_path = "steam-library.csv";

    let file = File::open(csv_path).expect("Couldn't open file");
    let mut reader = ReaderBuilder::new().from_reader(file);

    // 遍历表头，获取索引
    let headers = reader.headers().expect("no header");
    let name_index = headers.iter().position(|r| r == "game").unwrap();
    let hours_index = headers.iter().position(|r| r == "hours").unwrap();

    //记录总数
    let mut count = 0;

    // 遍历
    for result in reader.records() {
        let record = result.expect("Couldn't get record");
        // 处理 CSV 记录中的数据
        if let (Some(name), Some(hours)) = (record.get(name_index), record.get(hours_index)) {
            // 搜索得到uuid
            let uuid = api_neodb::search(name).await.unwrap();

            //对搜索到的uuid进行判断
            if uuid == "null" {
                println!("{} not found", name);
                continue;
            } else {
                // 根据游戏时间进行判断,默认为玩过
                let mut shelf_type = "complete";

                //无时间数据则判断为想玩
                if hours.is_empty() {
                    println!("{} hasn't played", name);
                    shelf_type = "wishlist";
                }
                //开始标记
                println!("try to mark {} ", name);
                //api_neodb::mark(uuid, config.neodb_token.to_string(), shelf_type).await.unwrap();
                let result = api_neodb::mark(uuid, config.neodb_token.to_string(), shelf_type).await.unwrap();
                if result == "\"OK\"" {
                    // 绿色字体
                    println!("\x1b[32m{} mark success\x1b[0m", name);
                    count += 1;
                } else {
                    // 红色字体
                    println!("\x1b[31m{} mark failed\x1b[0m", name);
                }
            }
        }
    }
    println!("{} games have been marked", count);

    Ok(())
}
