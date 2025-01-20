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



