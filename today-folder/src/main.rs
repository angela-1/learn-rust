use chrono::prelude::*;
use std::fs::DirBuilder;


fn main() {
	
	let local = Local::now().format("%Y_%m_%d").to_string();
    println!("Hello, world! {}", local);
    create_folder(&local)
}

fn create_folder(folder_path: &str) {
	DirBuilder::new().recursive(true).create(&folder_path).unwrap();
}