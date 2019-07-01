use regex::Regex;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::path::PathBuf;

use dotext::doc::MsDoc;
use dotext::docx::Docx;
use std::env;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;

#[derive(Debug)]
struct Document {
    code: String,
    title: String,
    send_to: String,
    send_by: String,
    send_date: String,
}

fn read_lines(path: PathBuf) -> Result<Vec<String>, String> {
    let mut file = Docx::open(&path).expect("Cannot open file");
    let mut isi = String::new();
    let _ = file.read_to_string(&mut isi);
    let split = isi.split("\n\n");
    let mut vec: Vec<String> = vec![];

    split.for_each(|v| {
        vec.push(v.trim().to_string())
    });
    Ok(vec)
}

fn _reg_search(reg: Regex, line: &String) -> Option<String> {
    let mat = reg.find(line);
    if mat != None {
        let value = String::from(mat.unwrap().as_str());
        Some(value)
    } else {
        None
    }
}

fn _is_white_line(line: &String) -> bool {
    let white_reg = Regex::new(r"^\s*$").unwrap();
    white_reg.is_match(line)
}

fn _get_code(line: &String) -> Option<String> {
    let code_reg = Regex::new(r"\S+〔\d{4}〕\d+号").unwrap();
    _reg_search(code_reg, line)
}

fn _get_send_date(line: &String) -> Option<String> {
    let date_reg = Regex::new(r"^\d{4}年\d{1,2}月\d{1,2}日$").unwrap();
    _reg_search(date_reg, line)
}

fn _get_send_to(line: &String) -> Option<String> {
    let send_to_reg = Regex::new(r"^\S+[：:]$").unwrap();
    _reg_search(send_to_reg, line)
}

fn parse_content(vec: Vec<String>) {
    let mut doc = Document {
        code: String::from(""),
        title: String::from(""),
        send_to: String::from(""),
        send_by: String::from(""),
        send_date: String::from(""),
    };

    let mut flag: u8 = 0b0000;
    const HAS_CODE: u8 = 0b0001;
    const HAS_TITLE: u8 = 0b0010;
    const HAS_SEND_TO: u8 = 0b0100;
    const HAS_SEND_DATE: u8 = 0b1000;

    let mut code_pos = 0;

    for (pos, e) in vec.iter().enumerate() {
        if flag & HAS_CODE == 0 && flag & HAS_TITLE == 0 {
            if let Some(pat) = _get_code(e) {
                flag |= HAS_CODE;
                doc.code = pat;
                code_pos = pos;
                continue;
            }
        }

        if flag & HAS_SEND_DATE == 0 {
            if let Some(pat) = _get_send_date(e) {
                flag |= HAS_SEND_DATE;
                doc.send_by = vec[pos - 1].to_string();
                doc.send_date = pat;
                continue;
            }
        }

        if flag & HAS_SEND_TO == 0 {
            if let Some(pat) = _get_send_to(e) {
                flag |= HAS_SEND_TO;
                doc.send_to = pat;

                if code_pos != 0 {
                    let mut nv = vec.clone();
                    let u: Vec<String> = nv.drain((code_pos + 1)..pos).collect();
                    let s = String::from_iter(u.clone());
                    doc.title = s;
                    flag |= HAS_TITLE;
                } else {
                    let mut title_vec: Vec<String> = vec![];
                    let mut not_white = false;
                    for i in 1..pos {
                        if vec[pos - i].to_string() != "" {
                            title_vec.push(vec[pos - i].to_string());
                        }
                        if title_vec.len() > 0 {
                            not_white = true;
                        }
                        if _is_white_line(&vec[pos - i]) && not_white {
                            title_vec.reverse();
                            let s = String::from_iter(title_vec.clone());
                            doc.title = s;
                            flag |= HAS_TITLE;
                            break;
                        }
                    }
                }
                continue;
            }
        }

        if flag == HAS_CODE | HAS_TITLE | HAS_SEND_TO | HAS_SEND_DATE {
            break;
        }
    }
    println!("{}\t{}\t{}", doc.send_date, doc.code, doc.title);
}

fn read_folder(dir: PathBuf) -> Result<(), Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry.path().into_os_string();
            let path = Path::new(&file_name);
            match path.extension() {
                Some(ext) if ext == "docx" => {
                    let vec = read_lines(path.to_path_buf()).unwrap();
                    parse_content(vec);
                }
                Some(_) => (),
                None => (),
            }
        }
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "不是有效的文件夹。"))
    }
}

fn main() {
    match env::args().nth(1) {
        Some(path) => {
            let p = Path::new(&path);
            match p.is_dir() {
                true => match read_folder(p.to_path_buf()) {
                    Ok(_) => println!("Done"),
                    Err(_) => (),
                },
                false => {
                    let p = PathBuf::from(path);
                    let vec = read_lines(p).unwrap();
                    parse_content(vec);
                }
            }
        }
        None => println!("not file or folder"),
    }
}
