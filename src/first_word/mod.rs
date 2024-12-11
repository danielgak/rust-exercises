use std::io;

pub fn main() {
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    let word = first_word(&text);

    println!("{word}");
}

// write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

// str type accepts String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
