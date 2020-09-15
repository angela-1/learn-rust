use chrono::prelude::Local;
use std::fs;
use std::io;

fn main() {
    let local = Local::now().format("%Y_%m_%d").to_string();
    create_folder(&local).unwrap();
    println!("完成 {}", local);
}

fn create_folder(folder_path: &str) -> io::Result<()> {
    fs::create_dir_all(folder_path)?;
    Ok(())
}
