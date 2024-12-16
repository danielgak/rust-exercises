pub fn main() {
    let s1 = String::new(); // empty
    let s2 = "initial contents".to_string(); // parsing a str literal
    let s3 = String::from("initial contents"); // another function to parse the str literal

    // UTF 8 allows any properly encoded data
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let s4 = String::from("something ") + &String::from("extra");
    let s5 = format!("{s1} {s2} {s3} {s4}"); // format! macro uses only references
    println!("{s5}");

    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = s1 + &s2;
    println!("s3 is {s3}, s2 is {s2}, s1 can be used");
    // s1 has moved so it does not exist anymore! but s2 is still there
    // The + operator uses the add method, whose signature looks something like this:
    //
    // fn add(self, s: &str) -> String { ... }
    //
    // so s1 is no lnger valid in the main scope bc the add function takes ownership of self,as self does not have an &
    // while s2 gets automatically cohersed from the &String argument given by us into a &str.
    // so, when we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    //

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // in this way we do not change ownership of s2
    s1.push('l');
    println!("s1 is {s1}, s2 is {s2}");

    let hello = "Здравствуйте"; // the first byte of З is 208 and the second is 151,

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
