use filetime::{set_file_mtime, FileTime};
use std::fs;

fn main() {
    let filename = "Cargo.toml";
    let metadata = fs::metadata(filename).unwrap();
    if let Ok(old_mtime) = metadata.modified() {
        println!("mtime was {:?}", old_mtime);
    } else {
        panic!("Not supported on this platform");
    }

    let current_mtime = FileTime::now();
    set_file_mtime(filename, current_mtime).unwrap();
    println!("new mtime is {:?}", current_mtime);
}
