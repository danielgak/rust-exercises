use std::collections::HashMap;
use std::ops::Add;

pub fn is_dup_v1(s: &str) -> bool {
    let mut chars = HashMap::new();

    if s.len() > 256 {
        return true;
    }

    for c in s.chars() {
        match chars.get(&c) {
            None => {
                chars.insert(c, true);
            }
            Some(_) => return true,
        }
    }

    false
}

// ASCII - 7 bits
// UTF-8 - (variable) 1 to 4 bytes

// 1, 2, 4, 8, 16, 32, 64, 128, 256
// we assume no more than 26 distinct characters, so they fit in an integer
pub fn is_dup_v2(s: &str) -> bool {
    let mut history: u32 = 0b0;

    if s.len() > 256 {
        return true;
    }

    for c in s.chars() {
        let index = c as u32 - ('a' as u32);
        let mask = 1 << index; // a = 1, b = 10, c= 100 ... and z fits before overflow
        let result = history & mask;
        if result > 0 {
            return true;
        }

        history = history | mask;
    }

    false
}

pub fn is_permutation(str1: &str, str2: &str) -> bool {
    let mut vec1 = Vec::from_iter(str1.chars());
    let mut vec2 = Vec::from_iter(str2.chars());

    vec1.sort();
    vec2.sort();

    let str1 = vec1.iter().collect::<String>();
    let str2 = vec2.iter().collect::<String>();

    str1 == str2
}

pub fn urlfy(str: &str) -> String {
    let char = "%20";
    let mut url = String::new();
    let mut add_replacement = false;

    for a in str.chars() {
        if a == ' ' {
            add_replacement = true;
            continue;
        }

        if add_replacement {
            url = url.add(&char);
            add_replacement = false;
        }

        url.push(a)
    }

    url
}

pub fn palindrome_permutation(str: &str) -> bool {
    let mut counts = HashMap::new();
    for a in str.chars() {
        let c = *counts.entry(a).or_insert(0) + 1;
        counts.insert(a, c + 1);
    }

    let mut exceptions = 0;
    for key in counts.keys() {
        if counts[key] % 2 != 0 {
            exceptions += 1;
        }
    }

    exceptions <= 2
}
