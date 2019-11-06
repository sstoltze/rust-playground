use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Result;
use std::path::{Path, PathBuf};

struct WordPosition {
    line: u32,
    column: u32,
}

struct IndexEntry {
    file: PathBuf,
    positions: Vec<WordPosition>,
    containing_line: String,
}

fn build_index(file: PathBuf) -> HashMap<String, Vec<IndexEntry>> {
    let _ie = IndexEntry {
        file,
        positions: Vec::new(),
        containing_line: "test".to_string(),
    };
    let mut h = HashMap::new();
    h.insert("test".to_string(), Vec::new());
    h
}

fn find_files(p: PathBuf) -> Result<Vec<PathBuf>> {
    let mut res = Vec::new();
    match p.file_name() {
        Some(file_name) => {
            match file_name.to_str() {
                Some(file_str) => {
                    // Ignore hidden paths
                    if !file_str.starts_with(".") {
                        if p.is_dir() {
                            for entry in read_dir(p)? {
                                let entry = entry?;
                                let path = entry.path();
                                let mut vec = find_files(path)?;
                                res.append(&mut vec);
                            }
                        } else {
                            res.push(p.to_path_buf());
                        }
                    }
                }
                None => (),
            }
        }
        None => (),
    }
    Ok(res)
}

fn main() -> Result<()> {
    let p = std::fs::canonicalize(Path::new("../.."))?;
    let files = find_files(p)?;
    println!("{:?}", files);
    Ok(())
}
