/* fileIO */

use std::fs;

fn __get_name__(name: String) {

    println! ("file to read {}", name);
}

pub fn get_contents_from_file (filename: String) -> String {

    let contents = fs::read_to_string (filename).expect ("Unable to open file for reading.");

    return contents;
}

pub fn iterate_over_characters (contents: String) {

    for iterator in contents.chars().enumerate() {
        println!("{:?}", iterator);
    }
}
