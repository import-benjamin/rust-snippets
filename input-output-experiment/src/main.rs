use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn main() {

    let file = File::open("hello.txt").expect("Can't read file hello.txt");
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {name, ..}) => {
                println!("{name}");
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }
}

