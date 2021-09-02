fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let x = 5;
    let y = x;


    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
