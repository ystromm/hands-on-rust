use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();

    println!("Hello, {}!", capitalize_first(&your_name));

    if your_name == "bert" {
        println!("Welcome!");
    } else {
        println!("Sorry, you are not welcome!")
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    chars
        .next()
        .map(|first_letter| first_letter.to_uppercase())
        .into_iter()
        .flatten()
        .chain(chars)
        .collect()
}
fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
