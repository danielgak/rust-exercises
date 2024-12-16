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

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
pub fn median_and_mode(arr: &[i64]) -> (i64, i64) {
    let median = 0;
    let mode = 0;

    let len = arr.len() / 2;
    let arr = arr.sort();

    return (median, mode);
}

// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
pub fn pig_latin() {}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

type Company = HashMap<String, [String; 100]>;
pub fn add(mut company: &Company, addCommand: String) {
    let check = Collection::from([1, 2, 4, 4]);

    let [action, subject, direction, deparment] = addCommand.split_whitespace(); // ?
    assert_eq!(action, "Add");
    assert_eq!(direction, "to");

    company.entry(deparment).or_insert(Array::from(subject))
}
