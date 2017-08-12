use std::io;
use std::process;

fn how_many() -> i32 {
    println!("HOW FAR WILL WE TAKE THIS");

    let mut how_many = String::new();

    io::stdin().read_line(&mut how_many)
        .expect("Failed to read line");

    match how_many.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature, bye");
            process::exit(1);
        }
    }
}

fn fibonacci(term: u32) -> u32 {
    if term == 1 || term == 2 {
        return 1
    }

    fibonacci(term - 2) + fibonacci(term - 1)
}

fn main() {
    let how_many = how_many();

    for term in 1..how_many+1 {
        println!("Term {}: {}", term, fibonacci(term as u32))
    };
}
