fn main() {

    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    let x = 5;

    println!("The value of x is: {}", x);

    let x = x + 1;

    println!("The value of x is: {}", x);

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // https://doc.rust-lang.org/book/ch03-02-data-types.html

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
}
