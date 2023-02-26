use std::{
    cell::RefCell,
    io::Error,
    path::{Path, PathBuf},
    rc::Rc,
};

use crypto::digest::Digest;
use crypto::md5::Md5;

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

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
    HARDLINK,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    path: PathBuf,
    file_type: Option<FileType>,
    hash: Option<String>,
    inode: Option<u32>,
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

    fn build_children_node(root: &mut Node) {
        let root_path = root.path.as_path();
        // root_path.metadata()
    }
}
