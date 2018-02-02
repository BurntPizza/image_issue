extern crate image;

use image::*;
use std::fs::*;
use std::env::*;
use std::io::*;

fn main() {
    let path = current_dir().unwrap().join("test.png");
    let file = File::open(&path).unwrap();
    println!("File found");
    let image = load(BufReader::new(file), PNG).unwrap();
    println!("Image loaded");
    println!("Dimensions: {:?}", image.dimensions());
}
