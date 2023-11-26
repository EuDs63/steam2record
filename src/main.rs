use csv::ReaderBuilder;
use std::{error::Error, fs::File};
use tokio;

mod app_config;
mod api_neodb;
mod api_bangumi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = app_config::AppConfig::from_file("config.toml").unwrap();
    //println!("neodb_token is {}", config.neodb_token);

    let csv_path = "steam-library.csv";

    let file = File::open(csv_path).expect("Couldn't open file");
    let mut reader = ReaderBuilder::new().from_reader(file);

    // 遍历表头，获取索引
    let headers = reader.headers().expect("no header");
    let name_index = headers.iter().position(|r| r == "game").unwrap();
    let hours_index = headers.iter().position(|r| r == "hours").unwrap();

    //记录总数
    let mut neodb_count = 0;
    let mut bangumi_count = 0;

    // 遍历
    for result in reader.records() {
        let record = result.expect("Couldn't get record");
        // 处理 CSV 记录中的数据
        if let (Some(name), Some(hours)) = (record.get(name_index), record.get(hours_index)) {
            // 判断是否启用neodb
            match config.neodb_enable {
                true => {
                    if api_neodb::operate(name, hours, config.neodb_token.to_string()).await.unwrap()  {
                        neodb_count += 1;
                    }
                },
                _ => {
                    
                }
                
            }
            // 判断是否启用bangumi
            match config.bangumi_enable {
                true => {
                    if api_bangumi::operate(name, hours, config.bangumi_token.to_string()).await.unwrap() {
                        // 累加
                        bangumi_count += 1;
                    }
                },
                _ => {
                    
                }
            }

        }
    }
    println!("{} games have been marked on neodb", neodb_count);
    println!("{} games have been marked on bangumi", bangumi_count);

    Ok(())
}
