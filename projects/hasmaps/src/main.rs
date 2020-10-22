fn main() {
    println!("Hello, world!");
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // Constructs a hash from two vectors
    // One containing the keys and the other one the values.
    // underscores in the type let rust infer
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Red");
    let field_value = 30;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // the map borrows them.

    // Get value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("{:?}", score),
        None => println!("nothing"),
    }

    // Add if doesn't exist
    map.entry(String::from("Red")).or_insert(50);
    map.entry(String::from("Yellow")).or_insert(50);

    // Iterate
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // Change value of key based on existing
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // 0 if hasn't been seen
        let count = map.entry(word).or_insert(0);
        // increment 1 if has been seen
        *count += 1;
    }

    println!("{:?}", map);
}
