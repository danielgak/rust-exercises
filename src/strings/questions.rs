use std::collections::HashMap;

pub fn dup_v1(s: &str ) -> bool {
    let mut chars = HashMap::new();

    if s.len() > 256 {
        return true;
    }


    for c in s.chars() {
        match chars.get(&c) {
            None => {
                chars.insert(c, true);
            },
            Some(_) => {
                return true
            },
        }
    }

    false
}


// ASCII - 7 bits
// UTF-8 - (variable) 1 to 4 bytes

// 1, 2, 4, 8, 16, 32, 64, 128, 256
// we assume no more than 26 distinct characters, so they fit in an integer
pub fn dup_v2(s: &str) -> bool {
    let mut history: u32 = 0b0;

    if s.len() > 256 {
        return true;
    }

    for c in s.chars() {
        let index = c as u32 - ('a' as u32);
        let bw = 1 << index; // a = 1, b = 10, c= 100 ... and z fits before overflow
        let result = history & bw;
        if result > 0 {
            return true
        }

        history = history | bw;
    }

    false
 }

