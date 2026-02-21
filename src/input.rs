use std::io::{self, Write};

pub fn read_position() -> usize {
    loop {
        print!("Enter position (1-9): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove whitespace/newline
        let trimmed = input.trim();

        match trimmed.parse::<usize>() {
            Ok(num) if (1..=9).contains(&num) => {
                return num - 1; // convert to 0-based index
            }
            _ => {
                println!("Invalid input. Please enter a number between 1 and 9.");
            }
        }
    }
}
