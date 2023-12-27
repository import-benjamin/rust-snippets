use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn input_is_stored(day: u8) -> bool {
    Path::new(&format!("input_day{}.txt", day)).exists()
}

pub fn get_stored_input(day: u8) -> String {
    let filename = format!("input_day{}.txt", day);
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    return contents;
}

pub fn store_input(day: u8, content: String) -> () {
    let filename = format!("input_day{}.txt", day);
    let mut f = File::create(filename).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}
