use std::fs::File;

fn main() {
    // unrecoverable_errors_with_panic() 
    recoverable_errors_with_result()
}

fn recoverable_errors_with_result(){
    let f u32 = File::open("hello.txt");
}

fn unrecoverable_errors_with_panic(){
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}