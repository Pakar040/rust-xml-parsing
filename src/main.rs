//use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("data/Pole_UP1-Existing.pplx");

    if path.exists() {
        println!("{:?}", path.as_os_str());

        
    } else {
        println!("Path does not exist");
    }
}
