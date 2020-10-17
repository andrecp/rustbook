#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        // Binds state variable.
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        // `_` Matches all other cases
        _ => 1,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    // Matching an Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", six, none);

    // Only want to execute something if the value is 3.
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // If let pattern for it!
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
