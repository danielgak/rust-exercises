pub fn main() {}

fn problem_01() {
    let mut vect1 = vec![3, 4, 2, 1, 3, 3];
    let mut vect2 = vec![4, 3, 5, 3, 9, 3];
    let mut res = Vec::new();

    &mut vect1.sort();
    &mut vect2.sort();

    let mut i = 0;
    while i < vect1.len() {
        res.push((vect1[i] - vect2[i]).abs());
    }

    let f = *res.iter().reduce(|acc, e| acc + e).expect("there is len!");
    println!("{f}");
}
