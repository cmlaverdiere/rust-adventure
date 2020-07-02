use std::io::{self, Write};
use std::{thread, time};

const PRINT_DELAY_MS: time::Duration = time::Duration::from_millis(25);

const READ_STRING_FAILURE: &str = "What? I didn't get that...";

pub fn prompt() -> String {
    let mut result = String::new();

    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut result)
        .expect(READ_STRING_FAILURE);

    result.pop();

    result
}

pub fn delay_print(s: &str) {
    s.chars().for_each(|c| {
        print!("{}", c);
        io::stdout().flush().unwrap();
        thread::sleep(PRINT_DELAY_MS);
    });
}
