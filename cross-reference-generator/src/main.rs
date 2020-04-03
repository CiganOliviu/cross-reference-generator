mod files_io;

fn main() {

    let data = files_io::get_contents_from_file ("src/data.data".to_string());

    println!("{}", data);
}
