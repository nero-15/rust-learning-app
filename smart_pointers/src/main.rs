enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    //boxPointers()
    // deref() 
    dropFunc()
}

fn boxPointers(){
    // let b = Box::new(5);
    // println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}

struct MyBox<T>(T);

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deref(){
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


use std::mem::drop;

fn dropFunc(){
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");

    // error↓↓
    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };
    // println!("CustomSmartPointer created.");
    // c.drop();
    // println!("CustomSmartPointer dropped before the end of main.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

