fn main() {
    // Vector puts items next to each other in memory
    // dynamic scales, can hold different types.
    // Vec<T> generic to specify the type.
    let v: Vec<i32> = Vec::new();

    // Same thing with a macro!
    let v = vec![1, 2, 3];

    println!("Hello, world!");

    let mut v = Vec::new();
    v.push(5);

    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // can't because already have a read reference
    // v.push(6);

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Mutating the reference through the pointer.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // An enum with a custom type.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
