use std::fs::{DirBuilder};
use std::fs::File;
use std::io::prelude::*;


fn main() {
    // create_folder();
    // create_folder().unwrap();
    match create_folder() {
    	Ok(_)  => println!("成功"),
        Err(_) => println!("失败")
    };
    // println!("Hello, world!");

    // let path = "./baz";

    // assert!(fs::metadata(path).unwrap().is_dir());
    // let path = get_primary_key();

    // let fd_name = "./".to_owned() + &path.unwrap().trim(); //+ &path.unwrap();
    // println!("{:?}", &path.unwrap().trim());
}

/// 根据获取的文件夹名称创建文件夹
fn create_folder () -> Result<(), String> {
	get_primary_key()
	.map_err(|err| err.to_string())
	.and_then(|key| {
		println!("请输入任务名称：");
		let mut buffer = String::new();
		std::io::stdin().read_line(&mut buffer)
		.map_err(|err| err.to_string())
		.map(|_| {
			let folder_path = get_folder_name(key, buffer.trim());
			DirBuilder::new().recursive(true).create(&folder_path).unwrap();
			let folder_draft_path = folder_path.clone() + "/0-draft";
			DirBuilder::new().recursive(true).create(&folder_draft_path).unwrap();
			key
		})
	})
	.and_then(|key| {
		write_current_key(key)
	})
}



/// 组装文件夹名称
fn get_folder_name(key: i32, folder_name: &str) -> String {
    "./".to_string() + &key.to_string() + "_" + folder_name
}

/// 创建文件夹成功后，将最新序列号写入 `foo.txt` 文件
fn write_current_key (key: i32) -> Result<(), String> {
	File::create("./foo.txt")
	.map_err(|err| err.to_string())
	.and_then(|mut file| {
		file.write_all(key.to_string().as_bytes())
		.map_err(|err| err.to_string())
	})
}

/// 读取 `foo.txt` 文件获取当前的序列号
fn get_primary_key () -> Result<i32, String> {
	File::open("./foo.txt")
	.map_err(|err| err.to_string())
	.and_then(|mut file| {
		let mut contents = String::new();
		file.read_to_string(&mut contents)
		.map_err(|err| err.to_string())
		.map(|_| contents)
	})
	.and_then(|contents| {
		contents.trim().parse::<i32>()
		.map_err(|err| err.to_string())
	})
	.map(|n| n + 1)
}