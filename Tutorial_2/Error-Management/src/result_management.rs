use std::fs::File;
use std::io::{self,Read};


pub fn read_file_contents(path: &str) -> io::Result<String>{
    let mut file = match File::open(path){
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut contents= String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e)
    }
}