fn main(){use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let _username_file_result = File::open("hello.txt");

    let mut _username_file = match _username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut _username = String::new();

    match _username_file.read_to_string(&mut _username) {
        Ok(_) => Ok(_username),
        Err(e) => Err(e),
    }
}
}
