fn main() {
    let mut s = String::new();

    let data = "initial contents";

    // Anything that implements the Display trait implements to_string
    let s = data.to_string();

    let s = String::from("initial contents");

    // UTF-8 Encode
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Growing a string
    let mut s = String::from("foo");
    s.push_str("bar");

    // appending
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // formatting
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // A String is a wrapper over a Vec<u8>
    // Another point about UTF-8 is that there are actually three relevant ways
    // to look at strings from Rust’s perspective: as bytes, scalar values, and
    // grapheme clusters (the closest thing to what we would call letters).
    let hello = "Здравствуйте";

    // 4 bytes of string is "Зд", 2 bytes per character
    let s = &hello[0..4];

    // Iterating, this will be 6 chars because of how indi works
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // will print the 18 bytes that make part of it
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
