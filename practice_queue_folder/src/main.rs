use std::fs::{self, DirBuilder};
use std::io::{stdin, Result};

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn get_queue_number(queue_file: &str) -> i32 {
    if let Ok(s) = fs::read_to_string(queue_file) {
        let number: i32 = plus_one(s.parse().unwrap_or(1));
        number
    } else {
        1
    }
}

fn make_numbered_directory(x: i32, name: &str) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(x.to_string() + "_" + name + "/0-draft")
}

fn save_current_queue_number(queue_file: &str, x: i32) -> Result<()> {
    fs::write(queue_file, x.to_string())?;
    Ok(())
}

fn main() {
    const QUEUE_FILE: &str = "foo";
    println!("请输入任务名称：");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim();

    let num: i32 = get_queue_number(QUEUE_FILE);
    match make_numbered_directory(num, &name[..]) {
        Ok(_) => {
            save_current_queue_number(QUEUE_FILE, num).unwrap();
            println!("完成");
        }
        Err(e) => println!("未创建目录 {}", e),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_plus_one() {
        assert_eq!(plus_one(3), 4);
    }

    #[test]
    fn it_get_queue_number_success() {
        assert_eq!(get_queue_number("for_test") > 1, true);
    }

    #[test]
    fn it_get_queue_number_fail_and_get_one() {
        assert_eq!(get_queue_number("no_file") == 1, true);
    }

    #[test]
    fn it_save_number(){
        use std::fs;
        let test_file = "for_test_save";
        save_current_queue_number(test_file, 22).unwrap();
        assert_eq!(get_queue_number(test_file) == 23, true);
        fs::remove_file(test_file).unwrap();
    }

}