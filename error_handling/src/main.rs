use std::fs::File;
use std::io::ErrorKind;
use std::net::IpAddr;

fn main() {
    // unrecoverable_errors_with_panic() 
    // recoverable_errors_with_result()
    to_panic_or_not_to_panic()
}

fn to_panic_or_not_to_panic(){
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);
}

fn recoverable_errors_with_result(){
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn unrecoverable_errors_with_panic(){
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}