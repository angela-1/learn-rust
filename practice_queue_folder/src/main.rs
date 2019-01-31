use std::fs;
use std::fs::DirBuilder;
use std::io::Result;

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

fn make_numbered_directory(x: i32) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(x.to_string() + "_")
}

fn save_current_queue_number(x: i32) -> Result<()> {
    fs::write(QUEUE, x.to_string())?;
    Ok(())
}

fn main() {
    let num: i32 = get_queue_number().unwrap_or(1);

    match make_numbered_directory(num) {
        Ok(_) => {
            save_current_queue_number(num).unwrap();
            println!("Done");
        }
        Err(_) => println!("Not create directory."),
    }
}
