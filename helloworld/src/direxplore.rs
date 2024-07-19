use std::env;
use std::fs;
use std::path::Path;

pub fn get_cur_dir() {
    let cur_dir = env::current_dir().unwrap();
    println!("Current directory: {}", cur_dir.display());
}

