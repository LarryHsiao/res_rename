extern crate clap;

use clap::{Arg, App, Values};
use std::{fs, io};
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

fn main() {
    let matches = App::new("resname")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("PATH")
                .help("The path of file to rename.")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("replace")
                .short("r")
                .long("replace")
                .value_name("string")
                .help("For replace matched strings to _.")
                .takes_value(true)
                .multiple(true)
                .required(false)
        )
        .get_matches();
    let path = matches.value_of("PATH").unwrap().to_string();

    visit_dirs(Path::new(path.as_str()), &|file: &DirEntry| {
        let i_file_name = file.file_name();
        let mut buf = PathBuf::new();
        buf.push(file.path().parent().unwrap().to_str().unwrap());
        let file_name = i_file_name.to_str().unwrap().chars();
        let updated_name: String = file_name.map(|char| {
            let replace: Vec<&str> = matches.values_of("replace").unwrap_or(Values::default()).collect();
            match char {
                ' ' | '-' => '_',
                'A'..='Z' => char.to_ascii_lowercase(),
                _ => if replace.contains(&char.to_string().as_str()) {
                    '_'
                } else {
                    char
                }
            }
        }).collect();
        buf.push(updated_name);
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
