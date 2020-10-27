// Recursive
// To know the List size it goes into recursion!
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

// Must implement the Deref trait to be dereferenced.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // our struct is a tupe of one element
        &self.0
    }
}
// DerefMut trait for mut references.

// Drop Trait is called once out of scope
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// RC allows for multiple owners -- reference counting.
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

use std::rc::Rc;

fn main() {
    // A simple box, allocates on the heap.
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    let x = 5;
    // reference
    let y = &x;
    let z = Box::new(x);
    let w = MyBox::new(x);


    assert_eq!(5, x);
    // dereference
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);


    // Drop
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");


    // With reference counting
    // Clone increments the reference count
    let ax = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    let bx = ListRc::Cons(3, Rc::clone(&ax));
    let cx = ListRc::Cons(4, Rc::clone(&ax));

}
