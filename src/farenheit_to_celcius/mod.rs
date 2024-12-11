use std::io;

pub fn main() {
    println!("Farenheit to celcius");
    let farenheit = loop {
        let mut farenheit_string = String::new();
        io::stdin()
            .read_line(&mut farenheit_string)
            .expect("Failed to read line");

        break match farenheit_string.trim().parse::<i16>() {
            Ok(num) => num,
            Err(_) => {
                println!("not valid!");
                continue;
            }
        };
    };

    println!("Celcius {} ", (farenheit - 32) * 5 / 9); // this will overflow
}
