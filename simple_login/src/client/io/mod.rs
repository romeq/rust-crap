use std::io;
use std::io::Write;

pub fn ask_input(q: &str) -> String {
    print!("{}: ", q);
    io::stdout()
        .flush()
        .unwrap();

    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input: String = input
        .trim() // trim out newlines
        .parse()
        .unwrap();
    return input;
}