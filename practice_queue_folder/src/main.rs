use std::fs::{self, DirBuilder};
use std::io::{stdin, Result};

const QUEUE: &str = "foo";

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn get_queue_number() -> Option<i32> {
    if let Ok(s) = fs::read_to_string(QUEUE) {
        let number: Option<i32> = plus_one(s.parse().ok());
        number
    } else {
        None
    }
}

fn make_numbered_directory(x: i32, name: &str) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(x.to_string() + "_" + name)
}

fn save_current_queue_number(x: i32) -> Result<()> {
    fs::write(QUEUE, x.to_string())?;
    Ok(())
}

fn main() {
    println!("请输入名称：");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim();

    let num: i32 = get_queue_number().unwrap_or(1);
    match make_numbered_directory(num, &name[..]) {
        Ok(_) => {
            save_current_queue_number(num).unwrap();
            println!("完成");
        }
        Err(e) => println!("未创建目录 {}", e),
    }
}
