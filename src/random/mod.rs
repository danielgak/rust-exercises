use std::collections::HashMap;

pub fn main() {
    median_and_mode(&[1, 2, 2, 5, 3, 1, 2]);

    pig_latin(&String::from("apple"));
    pig_latin(&String::from("slippery"));
    pig_latin(&String::from("çopple"));

    let mut company: Company = HashMap::new();
    add(&mut company, &String::from("Add dani to sales"));
    // add(&mut company, &String::from("some other command"));
    add(&mut company, &String::from("Add regina to engineering"));
    add(&mut company, &String::from("Add fran to sales"));

    println!("{:?}", company);
}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
pub fn median_and_mode(arr: &[i64]) {
    let median = match arr.get(arr.len() / 2) {
        Some(v) => *v,
        None => 0,
    };

    let mut map = HashMap::new();
    for i in arr {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value >= max {
            max = value;
            mode = *key;
        }
    }

    println!("median: {median}, mode: {mode}");
}

// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
pub fn pig_latin(word: &String) {
    if word.is_empty() {
        return;
    }

    let first_byte = word.as_bytes()[0];
    if !first_byte.is_ascii_alphabetic() {
        println!("{word}");
        return;
    }

    let vocals = b"aeiou";
    if vocals.contains(&first_byte.to_ascii_lowercase()) {
        println!("{word}-hay");
        return;
    }

    let (prefix, sufix) = word.split_at(1);
    println!("{sufix}-{prefix}ay");
}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
type Company = HashMap<String, Vec<String>>;
pub fn add(company: &mut Company, addCommand: &String) {
    let mut split = addCommand.split_whitespace();
    match split.next() {
        Some(v) => assert_eq!(v, "Add"),
        None => return,
    };
    let subject = match split.next() {
        Some(v) => v.to_string(),
        None => return,
    };
    match split.next() {
        Some(v) => assert_eq!(v, "to"),
        None => return,
    };
    let deparment = match split.next() {
        Some(v) => v.to_string(),
        None => return,
    };

    company.entry(deparment).or_insert(Vec::new()).push(subject);
}
