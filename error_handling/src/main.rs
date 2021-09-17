use std::fs::File;

fn main() {
    // unrecoverable_errors_with_panic() 
    recoverable_errors_with_result()
}

fn recoverable_errors_with_result(){
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn unrecoverable_errors_with_panic(){
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}