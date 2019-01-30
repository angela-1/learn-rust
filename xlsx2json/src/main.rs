use calamine::Error;
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::collections::HashMap;
use std::env;
use std::fs;

fn get_keys(start: u32, range: &Range<DataType>) -> Vec<String> {
    let col = range.get_size().1 as u32;
    let mut keys: Vec<String> = vec![];
    for n in 0..col {
        keys.push(range.get_value((start, n)).unwrap().to_string());
    }
    keys
}

fn get_range(path: &str) -> Result<Range<DataType>, Error> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or(Error::Msg("Cannot find 'Sheet1'"))??;
    Ok(range)
}

fn parse_row(keys: &Vec<String>, range: &Range<DataType>) -> HashMap<String, String> {
    let mut line: HashMap<String, String> = HashMap::new();
    for n in 0..range.get_size().1 {
        line.insert(keys[n].to_string(), range.get((0, n)).unwrap().to_string());
    }
    line
}

fn main() {
    let file = env::args().nth(1).expect("Please provide a xlsx file");

    let title_line = env::args()
        .nth(2)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();

    let dest = env::args().nth(3).unwrap_or(file.clone() + ".json");

    let range = get_range(&file).ok().expect("fail");
    let keys = get_keys(title_line, &range);
    // println!("{:?}", &keys);

    let mut result: Vec<HashMap<String, String>> = Vec::new();

    let (row, col) = range.get_size();
    let data_line = title_line + 1;
    for n in data_line..row as u32 {
        let r = range.range((n, 0), (n, col as u32 - 1));
        result.push(parse_row(&keys, &r));
    }
    let s = serde_json::ser::to_string(&result).unwrap();
    fs::write(dest, s).expect("Unable to write file");
    println!("生成成功！");
}
