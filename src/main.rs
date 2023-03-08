use std::{
    cell::RefCell,
    io::Error,
    path::{Path, PathBuf},
    rc::Rc,
};

use clap::Parser;
use crypto::digest::Digest;
use crypto::md5::Md5;

use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
struct Cli {
    /// directory path
    path: Option<PathBuf>,

    /// print json
    #[arg(short, long)]
    json: bool,

    /// Include hidden files in the calculation
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let cli = Cli::parse();
    let path = match cli.path {
        Some(p) => p,
        None => {
            println!("{}", "--help to show help message");
            return;
        }
    };
    if path.starts_with("..") || path.to_str().eq(&Option::Some(".")) {
        println!("directry not be . or ..");
        return;
    }
    let root = Node::scan(&path, cli.all);
    if cli.json {
        let json = serde_json::to_string_pretty(&root);
        match json {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("{}", root.hash.unwrap());
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    name: String,
    hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Rc<RefCell<Vec<Node>>>>,
}

impl Node {
    pub fn scan(path: &Path, all: bool) -> Node {
        let mut root = Node {
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            hash: Option::None,
            children: Option::None,
        };

        if path.is_file() {
            let md5 = Node::sum_hash(path);
            root.hash = Option::Some(md5.unwrap());
            return root;
        } else {
            let mut children = Vec::new();
            let mut hash_str = String::new();
            for entry in walkdir::WalkDir::new(path)
                .max_depth(1)
                .min_depth(1)
                .follow_links(true)
                .into_iter()
                .filter_map(|f| f.ok())
            {
                let file_name = entry.file_name().to_string_lossy();

                if file_name.starts_with(".") && !all {
                    continue;
                }

                let child = Node::scan(entry.path(), all);
                let child_hash = match &child.hash {
                    Some(s) => s,
                    None => "",
                };
                hash_str = hash_str + &child_hash;
                children.push(child);
            }
            root.hash = Option::Some(Node::sum_hash_str(&hash_str));
            root.children = Option::Some(Rc::new(RefCell::new(children)));
            return root;
        }
    }

    fn sum_hash_str(s: &str) -> String {
        let mut hasher = Md5::new();
        hasher.input_str(s);
        hasher.result_str()
    }

    fn sum_hash(path: &Path) -> Result<String, Error> {
        let file = std::fs::read(path)?;
        let mut hasher = Md5::new();
        hasher.input(&file);
        let ans = hasher.result_str();
        Ok(ans)
    }
}
