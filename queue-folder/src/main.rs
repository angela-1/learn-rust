
// 按顺序创建文件夹
// 
use std::fs;
use std::io::{stdin, Result};

fn get_number(file: &str) -> i32 {
	match fs::read_to_string(file) {
		Ok(v) => v.parse().unwrap_or(1) + 1,
		Err(_) => 1
	}
}

fn save_number(file: &str, x: i32) -> Result<()> {
    fs::write(file, x.to_string())?;
    Ok(())
}

fn create_folder(x: i32, name: &str) -> Result<()> {
    fs::create_dir_all(x.to_string() + "_" + name + "/0-draft")?;
    Ok(())
}

fn main() {
	const QUEUE_FILE: &str = "foo";
    println!("请输入任务名称：");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim();

    let x: i32 = get_number(QUEUE_FILE);

    match create_folder(x, name) {
    	Ok(_) => save_number(QUEUE_FILE, x).unwrap(),
    	Err(e) => println!("Error occured {}", e)
    }

    // save_number(QUEUE_FILE, x);
    println!("Hello, world! {}", x);
}
