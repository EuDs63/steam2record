use std::{error::Error, fs::File};

use csv::ReaderBuilder;

mod api_neodb;

fn main() -> Result<(), Box<dyn Error>> {
    let csv_path = "steam-library.csv";

    let file = File::open(csv_path).expect("Couldn't open file");
    let mut reader = ReaderBuilder::new().from_reader(file);

    // 遍历表头，获取索引
    let headers = reader.headers().expect("no header");
    let name_index = headers.iter().position(|r| r == "game").unwrap();
    let hours_index = headers.iter().position(|r| r == "hours").unwrap();

    // 遍历
    for result in reader.records() {
        let record = result.expect("Couldn't get record");
        // 处理 CSV 记录中的数据
        if let(Some(name), Some(hours)) = (record.get(name_index), record.get(hours_index)) {
            //判断，无数据则判断为想玩
            if hours.is_empty(){
                println!("{} hasn't played", name);
                continue;
            }
            println!("{} played for {} hours", name, hours);
        }
    }


    Ok(())
}
