## Cargo

* build
    * --release
* check
* run
* update
    * Update dependencies
* doc
    * --open

## Basic

```rs

// A constant must declare the type.
const MAX_POINTS: u32 = 100_000;

// Variables are immutable by default.
let foo = 3;

// A mutable variable.
let mut foo = 4;

// A float
let y: f32 = 3.0;

// An integer
let x: i32 = 3;

// A boolean
let f: bool = false;

// A char
let c = 'c';

// A string
let c = "hello world";

// A tuple can have values of different types
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = x.0;

// An array must have all the values of the same type
// It is a single chunk of memory allocated on the stack
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [1, 2, 3, 4, 5];
let a = [0; 5]; // initialize [0 0 0 0 0]

// A struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user.email

// Just like Javascript you don't need to repeat they key, value 
// if they're the same.
let email = "aprado@castelan.com"
let user1 = User{
    email,
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

// And also has the ... syntax which uses the remaining fields from another struct.
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};

// Tuple struct
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);

// Static method new of the String type.
// static is called associated function in Rust and is defined
// in an `impl` block with no self.
String::new();

// Referencing strings with slices
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
// &str is a string slice reference type

// Reference, no copy in memory
&foo
&mut guess

// Cast
let guess: u32 = guess.trim().parse()

// Switch equivalent
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

// If statement
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}

// Ternary operation
let condition = true;
let number = if condition { 5 } else { 6 };

// A function 
fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    x // Last expression is returned, equivalent to return x;
}

// Returning from a loop after the break.
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

// Rust also supports while and for loops.
for number in (1..4).rev() {
    println!("{}!", number);
}
```
### Immutable vs mutable variables

Big data structures will be expensive to copy, mutating in place makes more sense. For smaller ones the functional programming style might make more sense.
 
### Result

The purpose of `Result` types is to encode error-handling information.

Values of the Result type, like values of any type, have methods defined on them. An instance of io::Result has an `expect` method that you can call. 

If this instance of io::Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. 
If this instance of io::Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it.

## Ownership

* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope the memory is automatically returned.

Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). 
The `drop` function in Rust will be familiar to you if you’ve used RAII patterns. It is what is called when it goes out of scope.


### Heap vs Stack

Heap is the "memory store", you allocate some memory free in the store and get a pointer back, you can allocate dynamic sizes. It is slower! You have to find a free memory slot and after to search it.

Stack is a fixed size known at compile time, it is faster!

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

### Some code...

```rs

// Alocates from the heap
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`

// Both x and y have a copy of 5 in the stack
let x = 5;
let y = x;

// There's only one hello string in the heap and two pointers to it.
// s2 now owns the reference to the heap, it moves from s1.
// can't use s1 anymore.
let s1 = String::from("hello");
let s2 = s1;

// A deep copy
let s1 = String::from("hello");
let s2 = s1.clone();

// Rust has a special annotation called the Copy trait that we can place on types like integers that are stored on the stack.
// If a type has the Copy trait, an older variable is still usable after assignment.
```

### Some more code...

```rs
fn main() {
    let s = String::from("hello");  // s comes into scope
    
    takes_ownership(s);             // s's value moves into the function..
                                    // ... and so is no longer valid here
    
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

```rs

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```


```rs
fn main() {
    let s1 = String::from("hello");

    // Just a reference, called borrow
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // References are immutable by default, making a mutable one.
    let mut s = String::from("hello");
    change(&mut s);

    // This would fail, can only have 1 mutable reference per scope to a value.
    // can't have a mutable and an immutable one in same scope as well.
    let r1 = &mut s;
    let r2 = &mut s;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Traits

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

// :? prints the Debug formatting, by default {} prints the Display formatting.
// and :#? is pretty printing.
println!("{:?} rectangle")
```

## Modules


If you want to make an item like a function or struct private, you put it in a module.

You then use the `pub` modifier to make it public! You gotta do in the fields as well in case of structs. For enums it isn't necessary to set on every field.

```rs
mod front_of_house {
    // private by default outside the module
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// brings to scope hosting
use crate::front_of_house::hosting;
hosting::add_to_waitlist();

// For data structures we use the full path
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Alias
use std::io::Result as IoResult;

// Bringing multiple things
use std::{cmp::Ordering, io};
use std::io::{self, Write}; // std::io and std::io::Write
use std::collections::*;

// Re-exporting, when we bring a name in the scope it is private by default
// this way others could also use hosting
pub use crate::front_of_house::hosting;

// loads the content of the module from a file called front_of_house.rs
mod front_of_house;
```

## Collections

See `hasmap`, `vectors` and `unicode`.

## Errors

* `Result<T, E>` for recoverable errors
* `panic!` for non recoverable errors

`Result` has the `.unwrap` method which either returns the result of panics!
The `.expect` is like `unwrap` but allows for a custom error message.

The `?` pattern allows you to return the error if an error is found or get the result.

You can also return an `Error` to propagate it.
```rs
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Above pattern can be expressed with `?` 
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even simpler
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

## Generics, traits & lifetimes

Generics can be used in structs, enums and functions, `<T>`. Can have multiple generic values as in `<T, U>`.

No runtime penality for generics, expanded by the compiler for each concrete type used.

Traits are like Interfaces.

```rs
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Anything that implements `summarize` with the signature below will fulfil the `Summary` trait.

```rs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

You can also have a default implementation such as

```rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

A function that can accept anything that implements the `Summary` trait:

```rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Must implement both Summary and Display
pub fn notify(item: &(impl Summary + Display)){}

// Alternative syntax
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug {
            
}

// To be used in returns without specifying the type!
fn returns_summarizable() -> impl Summary {
```

The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

```rs
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

## Testing

Function with `#[test]`, `#[should_panic]` for tests that should panic

`assert!`, `assert_eq!`, `assert_neq!`.

```rs
// Can pass a third parameter with the error message to asserts
assert!(
    result.contains("Carol"),
    "Greeting did not contain name, value was `{}`",
    result
);

// Can tailor the expect panic
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}

// Can return a Result enum with an error.
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

Runs in parallel threads by default, `cargo test -- --test-threads=1` to run sequentially.

`cargo test one_hundred` would only run the test whose function name contains `one_hundred`, can also use module name.

Tests with `#[ignore]` will only run with `cargo test -- --ignored`

Convention for unit tests is to have in the same file as the implementation in `src` but with the `tests` module

Convention for integration tests is to have a `tests` directory in the root of the project.

`cargo test --test integration_test` to run a single integration test by name matching.

## Exercises

* Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
* Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
