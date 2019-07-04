use std::io::{self, Write};

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
