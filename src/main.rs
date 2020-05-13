use std::{fs, io};
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Path input required.");
    visit_dirs(Path::new(path.as_str()), &|file: &DirEntry| {
        let i_file_name = file.file_name();
        let mut buf = PathBuf::new();
        buf.push(file.path().parent().unwrap().to_str().unwrap());
        let x = i_file_name.to_str().unwrap().chars();
        let s: String = x.map(|x| match x {
            ' ' | '-' => '_',
            'A'..='Z' => x.to_ascii_lowercase(),
            _ => x
        }).collect();
        buf.push(s);
        let origin_path = file.path();
        let origin_path = origin_path.to_str().unwrap();
        let target_path = buf.to_str().unwrap();
        if !origin_path.eq(target_path) {
            println!("origin: {}\ntarget: {}", origin_path.to_string(), target_path.to_string());
            fs::rename(
                origin_path.to_string(),
                target_path.to_string(),
            ).unwrap();
        }
    }).unwrap();
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
