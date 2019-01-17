use List::{Cons, Nil};
use std::ops::Deref; 
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self)->&T {
        &self.0
    }
}

fn hello(name : &str) {
    println!("Hello, {}", name)
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping Custom smart pointer with data {}", self.data);
    }
}

fn main() {

    // Box

    let b = Box::new(5);
    println!("{}", b);

    // let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));

    // Dereference

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref trait

    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(5,x);
    assert_eq!(5, *y); // *(y.deref())

    // Deref coercion

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    // Drop Trait

    let c = CustomSmartPointer{data:String::from("my stuff")};
    drop(c);
    let d = CustomSmartPointer{data:String::from("other stuff")};

    println!("Custom smart pointer created");

    // Reference count

    // let a = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    // let b = Box::new(Cons(3, Box::new(Cons(a))));
    // let c = Box::new(Cons(4, Box::new(a)));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Counter after creating a {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Counter after creating a {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("Counter after creating a {}", Rc::strong_count(&a));

    // Interiod mutability

    let x = 5;
    let y = &mut x;

}


