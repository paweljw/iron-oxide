use std::io;

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

fn main() {
    println!("Please provide mode of (F, C)");
    let mut mode = String::new();

    io::stdin().read_line(&mut mode)
        .expect("Failed to read line");

    let mode = mode.trim();

    println!("Please provide temperature");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: i32 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature");
            return;
        }
    };

    if mode == "F" {
        println!("{}F is {}C", temperature, fahrenheit_to_celsius(temperature))
    } else if mode == "C" {
        println!("{}C is {}F", temperature, celsius_to_fahrenheit(temperature))
    } else {
        println!("Invalid mode provided")
    }
}
