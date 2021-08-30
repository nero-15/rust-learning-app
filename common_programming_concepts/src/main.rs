fn main() {

    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // const MAX_POINTS: u32 = 100_000;
    // println!("{}", MAX_POINTS);
    //
    // let x = 5;
    //
    // println!("The value of x is: {}", x);
    //
    // let x = x + 1;
    //
    // println!("The value of x is: {}", x);
    //
    // let x = x * 2;
    //
    // println!("The value of x is: {}", x);
    //
    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("{}", spaces);

    // https://doc.rust-lang.org/book/ch03-02-data-types.html

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let t = true;
    println!("The value of t is: {}", t);

    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {}", tup);


}
