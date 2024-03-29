use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct WordPosition {
    line: u32,
    column: u32,
}

#[derive(Debug)]
struct IndexEntry {
    file: PathBuf,
    position: WordPosition,
    containing_line: String,
}

type WordIndex = HashMap<String, Vec<IndexEntry>>;

fn combine_indexes(mut a: WordIndex, b: WordIndex) -> WordIndex {
    for (word, mut vec) in b {
        a.entry(word).or_default().append(&mut vec);
    }
    a
}

fn build_index(file: PathBuf) -> Result<WordIndex> {
    let mut file_index: WordIndex = HashMap::new();

    let f = File::open(file.clone())?;
    let reader = BufReader::new(f);

    for (index, line) in (0..).zip(reader.lines()) {
        let line = line?;
        for word in line.split_whitespace() {
            let ie = IndexEntry {
                file: file.clone(),
                position: WordPosition {
                    line: index + 1,
                    column: 0,
                },
                containing_line: line.clone(),
            };
            file_index.entry(String::from(word)).or_default().push(ie);
        }
    }
    Ok(file_index)
}

fn find_files(p: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if let Some(file_name) = p.file_name() {
        if let Some(file_str) = file_name.to_str() {
            // Ignore hidden paths
            if !file_str.starts_with('.') {
                if p.is_dir() {
                    for entry in read_dir(p)? {
                        let path = entry?.path();
                        let mut vec = find_files(path)?;
                        files.append(&mut vec);
                    }
                } else {
                    files.push(p.to_path_buf());
                }
            }
        }
    }
    Ok(files)
}

fn main() -> Result<()> {
    let p = std::fs::canonicalize(Path::new("."))?;
    let files = find_files(p)?;
    println!("{:?}", files);
    let mut index = WordIndex::new();
    for f in files.iter() {
        let file_index = build_index(f.clone())?;
        index = combine_indexes(index, file_index);
        // println!("{:?}", index);
    }
    for ie in index["column:"].iter() {
        println!("--- {} ---", ie.containing_line);
    }
    Ok(())
}
