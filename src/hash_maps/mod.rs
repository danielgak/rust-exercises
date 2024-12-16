use std::collections::HashMap;

pub fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 40);
    println!("{scores:?}");

    scores.entry(String::from("Purple")).or_insert(70);
    scores.entry(String::from("Purple")).or_insert(80);
    println!("{scores:?}");

    let text = "hello world wonderful world world wonderful, sayonara baby";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
