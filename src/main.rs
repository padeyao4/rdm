use std::{
    cell::RefCell,
    io::Error,
    os::linux::fs::MetadataExt,
    path::{Path, PathBuf},
    rc::Rc,
};

use crypto::digest::Digest;
use crypto::md5::Md5;

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug)]
struct Cli {}

fn main() {}

#[test]
fn test() {
    for entry in walkdir::WalkDir::new("C:/Users/11818/Desktop/AnimeGANv3-Python")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        println!("{:?}", entry.file_name());
    }
}

#[test]
fn md5_test() {
    let mut hasher = Md5::new();
    hasher.input_str("hello");
    let ans = hasher.result_str();
    println!("{ans}");
}

#[test]
fn test_inde() {
    let path = Path::new(r"C:\Users\11818\Desktop\AnimeGANv3-Python\job.yml");
    println!("{:?}", path.metadata());
}

#[derive(Debug, Serialize, Deserialize)]
enum FileType {
    FILE,
    DIRECTORY,
    SYMBOL,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    path: PathBuf,
    file_type: Option<FileType>,
    hash: Option<String>,
    inode: Option<u64>,
    children: Option<Rc<RefCell<Vec<Node>>>>,
}

impl Node {
    pub fn new(path: &Path) -> Node {
        Node {
            path: path.to_path_buf(),
            file_type: Option::None,
            hash: Option::None,
            inode: Option::None,
            children: Option::Some(Rc::new(RefCell::new(Vec::new()))),
        }
    }

    fn sum_file_hash(path: &Path) -> Result<String, Error> {
        let file = std::fs::read(path)?;
        let mut hasher = Md5::new();
        hasher.input(&file);
        let ans = hasher.result_str();
        Ok(ans)
    }

    fn build_children_node(path: &Path) -> Result<Node, Error> {
        let mut root = Node::new(path);

        let meta = path.metadata()?;
        root.inode = Option::Some(meta.st_ino());


        if path.is_dir() {
            root.file_type = Option::Some(FileType::DIRECTORY);
        }

        return Ok(root);
    }
}

// use linux stat to query inode info
// linux cp -l 
#[test]
fn list_file_type() {
    use std::os::linux::fs::MetadataExt;
    use std::path::Path;

    let src_file = Path::new("/root/rscp");
    let hard_file = Path::new("/root/test.hard");
    let link_file = Path::new("/root/test.link");

    let src_file_meta = src_file.metadata().unwrap();

    println!("{}", src_file_meta.st_ino());
    println!("{:?}", src_file_meta.file_type());
    println!("{:?}", src_file_meta.is_dir());
    println!("{:?}", hard_file.metadata().unwrap().file_type());
}
