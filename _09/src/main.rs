// fn main() {
//     println!("Start!");

//     panic!("自定义panic");
// }

use std::{fs::File};

fn main() {
    println!("Start!");

    let result = File::open("debug.txt");
    let file = match result {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => match File::create("debug.txt") {
                    Ok(file) => file,
                    _ => panic!("other error")
                },
                _ => panic!("other error")
            }
        }
    };
}
