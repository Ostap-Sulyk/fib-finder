use std::{io, process};
fn main() {
    let mut index = String::new();
    println!("Please enter index of fib you want to find: ");

    io::stdin().read_line(&mut index).unwrap();

    let index = index.trim().to_owned().parse::<u32>().unwrap_or_else(|_| {
        println!("You entered some giberish");
        println!("Please enter a valid positive integer");
        process::exit(-1);
    });

    println!("{}", find_fib(index))
}

fn find_fib(index: u32) -> u128 {
    let mut current = 0;
    let mut next = 1;
    for _ in 0..index {
        let spare = next;
        next += current;
        current = spare;
    }
    current
}
