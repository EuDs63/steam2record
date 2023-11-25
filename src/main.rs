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
            match config.enable_api {
                1 => {
                    if api_neodb::operate(name, hours, config.neodb_token.to_string()).await.unwrap()  {
                        count += 1;
                    }
                },
                _ => {
                    println!("{} played {} hours", name, hours);
                }
                
            }

        }
    }
    println!("{} games have been marked", count);

    Ok(())
}
