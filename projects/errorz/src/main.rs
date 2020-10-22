use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Returns a Result enum , which has Ok and Err as options.
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open");

    // Dies
    // panic!("crash and burn");
}
