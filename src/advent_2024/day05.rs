use std::collections::HashMap;

pub fn problem_05_v1() {
    let (ordering_rules, updates) = parse_input(PROBLEM4_V1_INPUT);
    let mut total = 0;

    for line in updates {
        let mut forbidden: Vec<i32> = Vec::new();
        let mut valid = true;
        for n in line {
            if forbidden.contains(&n) {
                valid = false;
                break;
            }

            let restrictions = ordering_rules.get(&n);
            match restrictions {
                Some(r) => {
                    let mut copy = r.clone();
                    forbidden.append(&mut copy)
                }
                None => continue,
            }
        }

        if valid {
            total += 1
        }
    }

    println!("{total}")
}

pub fn parse_input(str: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut split = str.split("\n\n");
    let ordering_text = split.next().unwrap();
    let update_text = split.next().unwrap();
    let mut ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for ordering_entry in ordering_text.split("\n") {
        let mut split = ordering_entry.split('|');
        let num1 = split.next().unwrap().parse::<i32>().unwrap();
        let num2 = split.next().unwrap().parse::<i32>().unwrap();

        ordering_rules.entry(num1).or_insert(vec![]).push(num2)
    }

    for update_line in update_text.split("\n") {
        let vec = update_line
            .split(',')
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        updates.push(vec);
    }

    (ordering_rules, updates)
}

const PROBLEM4_V1_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
