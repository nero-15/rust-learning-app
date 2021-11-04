fn main() {
    //println!("Hello World!\nI'm a Rustacean!");

    // let x = 5 +  90 +  5;
    // println!("Is `x` 10 or 100? x = {}", x);

    print()
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn print(){
    // println!("{} days", 31);
    // println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
}