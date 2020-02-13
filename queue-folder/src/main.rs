// 按顺序创建文件夹
//
use std::fs;
use std::io::{stdin, Result};


fn get_number(file: &str) -> i32 {
    match fs::read_to_string(file) {
        Ok(v) => v.parse().unwrap_or(0) + 1,
        Err(_) => 1,
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
        Err(e) => println!("发生错误 {}", e),
    }
    println!("完成 {}", x);
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn it_get_number_success() {
        assert_eq!(get_number("test_foo") > 1, true);
    }

    #[test]
    fn it_get_number_no_foo_and_return_one() {
        assert_eq!(get_number("no_file") == 1, true);
    }

    #[test]
    fn it_get_number_wrong_number_and_return_one() {
        assert_eq!(get_number("test_foo_null") == 1, true);
    }

    #[test]
    fn it_create_folder_success() {
        create_folder(32, "test_create").unwrap();
        assert_eq!(Path::new("32_test_create/0-draft").exists(), true);
        fs::remove_dir_all("32_test_create").unwrap();
    }

    #[test]
    fn it_save_number_success() {
        use std::fs;
        let test_file = "test_foo_save";
        save_number(test_file, 22).unwrap();
        assert_eq!(get_number(test_file) == 23, true);
        fs::remove_file(test_file).unwrap();
    }
}
