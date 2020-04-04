mod files_io;

fn main() {

    let data = files_io::get_contents_from_file ("__data__".to_string());
    let mut vector_with_content = files_io::save_data_from_content (data);
    vector_with_content.sort();
    vector_with_content.dedup();
    files_io::get_data_from_content (vector_with_content);
}
