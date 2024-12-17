pub fn main() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2]; // reference to the third element, but this one can panic
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // this one no panic, as its returns an option
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid.
    // You can’t have mutable and immutable references in the same scope.

    let mut v = vec![1, 2, 3, 4, 5];

    let _first = &v[0];

    v.push(6); // from now on, first is no longer valid

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // to mutate we need to defeer
    }
    // iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules.

    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in this vector.
}
