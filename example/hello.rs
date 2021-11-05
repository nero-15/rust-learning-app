use std::fmt::{self, Formatter, Display};

fn main() {
    //println!("Hello World!\nI'm a Rustacean!");

    // let x = 5 +  90 +  5;
    // println!("Is `x` 10 or 100? x = {}", x);

    print()
}

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn print(){
    // println!("{} days", 31);
    // println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Printing with `{:?}` is similar to with `{}`.
    // println!("{:?} months in a year.", 12);
    // println!("{1:?} {0:?} is the {actor:?} name.",
    //          "Slater",
    //          "Christian",
    //          actor="actor's");

    // // `Structure` is printable!
    // println!("Now {:?} will print!", Structure(3));
    
    // // The problem with `derive` is there is no control over how
    // // the results look. What if I want this to just show a `7`?
    // println!("Now {:?} will print!", Deep(Structure(7)));

    // let name = "Peter";
    // let age = 27;
    // let peter = Person { name, age };

    // // Pretty print
    // println!("{:#?}", peter);

    // let minmax = MinMax(0, 14);

    // println!("Compare structures:");
    // println!("Display: {}", minmax);
    // println!("Debug: {:?}", minmax);

    // let big_range =   MinMax(-300, 300);
    // let small_range = MinMax(-3, 3);

    // println!("The big range is {big} and the small is {small}",
    //          small = small_range,
    //          big = big_range);

    // let point = Point2D { x: 3.3, y: 7.2 };

    // println!("Compare points:");
    // println!("Display: {}", point);
    // println!("Debug: {:?}", point);

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
    }
}