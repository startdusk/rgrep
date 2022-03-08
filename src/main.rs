use std::{env, fs::metadata, fs::File, io::Read};

use color_eyre::owo_colors::OwoColorize;
use regex::Regex;

fn main() {
    color_eyre::install().unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("no seearching any");
        return;
    }

    let pattern = &args[1];
    let filename = &args[2];
    let r = Regex::new(pattern).unwrap();
    for entry in glob::glob(&filename).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if !metadata(&path).unwrap().is_dir() {
                    let mut f = File::open(&path).unwrap();
                    let mut buf = String::new();
                    f.read_to_string(&mut buf).unwrap();
                    let mut iter = buf.split("\n").into_iter();
                    let mut i: i32 = 0;
                    if let Some(name) = path.file_name() {
                        println!("{:?}", name);
                    }
                    loop {
                        i += 1;
                        match iter.next() {
                            Some(line) => {
                                for caps in r.captures(line) {
                                    for idx in 0..caps.len() {
                                        let cap = caps.get(idx).unwrap();
                                        let start = cap.start();
                                        let new_line = line.replace(
                                            cap.as_str(),
                                            &format!("{}", cap.as_str().red().bold()),
                                        );
                                        println!("\t{}:{} {}", i.blue(), start.yellow(), new_line);
                                    }
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
