/* fileIO */

use std::fs;

fn __get_name__(name: String) {

    println! ("file to read {}", name);
}

pub fn get_contents_from_file (filename: String) -> String {

    let contents = fs::read_to_string (filename).expect ("Unable to open file for reading.");

    return contents;
}

pub fn save_data_from_content (contents: String) -> std::vec::Vec<char> {

    let mut data_vector = Vec::new();

    for iterator in contents.chars() {

        data_vector.push(iterator);
    }

    return data_vector;
}

pub fn get_data_from_content (vector: std::vec::Vec<char>) {

    for iterator in &vector {
        print!("{} ", iterator);
    }
}
