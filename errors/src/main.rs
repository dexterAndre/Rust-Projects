use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    return fs::read_to_string("hello.txt");
}

fn main() {
    read_username_from_file();
}