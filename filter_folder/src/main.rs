use fs_extra::dir::remove;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Error;

use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

use fs_extra::dir::copy;
use fs_extra::dir::create_all;
use fs_extra::dir::CopyOptions;

type FsExtraResult = fs_extra::error::Result<()>;
/// 查找并复制文件夹
#[derive(StructOpt, Debug)]
#[structopt(
    name = "filter",
    about = "从文件夹中找出需要的文件夹复制到目标位置。"
)]
struct Opt {
    /// 存储查找的文件夹名称的文本文件路径
    #[structopt(short = "k", long = "keywords")]
    keywords: PathBuf,

    /// 源文件夹路径
    #[structopt(short = "s", long = "source")]
    source: PathBuf,

    /// 目标文件夹路径
    #[structopt(short = "t", long = "target")]
    target: PathBuf,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn read_keywords(keywords_file: PathBuf) -> Result<Vec<String>, String> {
    let file = match File::open(keywords_file) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let content = BufReader::new(file);
    let mut vec = Vec::new();
    for line in content.lines() {
        let str = line.unwrap().to_string();
        vec.push(str);
    }
    Ok(vec)
}

fn read_dirs(dir: PathBuf) -> Result<Vec<DirEntry>, Error> {
    let mut vec = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            // let file_name = entry.file_name().into_string().unwrap();
            vec.push(entry);
        }
        Ok(vec)
    } else {
        Err(Error::new(ErrorKind::Other, "不是有效的文件夹。"))
    }
}

fn copy_folder(from: &String, to: &String) -> FsExtraResult {
    let from_path = Path::new(from);
    let to_path = Path::new(to);
    let options = CopyOptions::new();
    copy(&from_path, &to_path, &options).unwrap();
    Ok(())
}

fn find_and_copy(dirs: Vec<DirEntry>, keywords: &Vec<String>, target: &String, verbose: u8) {
    for dir in dirs {
        for keyword in keywords {
            let file_name = dir.file_name().into_string().unwrap();
            if file_name.contains(keyword) {
                if verbose > 0  {
                    println!("找到 {} ", keyword);
                }
                copy_folder(&dir.path().into_os_string().into_string().unwrap(), target).unwrap();
            }
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    // println!("{:?}", opt);

    // 获取要查找的文件夹名列表
    let keywords = read_keywords(opt.keywords);

    let dirs = read_dirs(opt.source);

    let target = opt.target.into_os_string().into_string().unwrap();

    if !Path::new(&target).exists() {
        create_all(&target, true).unwrap();
    } else {
        remove(&target).unwrap();
    }

    find_and_copy(dirs.unwrap(), &keywords.unwrap(), &target, opt.verbose);

    println!("完成");
}
