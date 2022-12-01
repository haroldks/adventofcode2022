use std::fs::File;
use std::io::BufReader;

pub fn open_file(input: &str) -> BufReader<File> {
    let file = File::open(input).expect("File not found");
    let buffer = BufReader::new(file);
    return buffer;
}
