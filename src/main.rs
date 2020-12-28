use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting)
    }
}

fn main() {
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();

    println!("Hello, {}!", capitalize_first(&your_name));

    let visitors = [
        Visitor::new("bert", "Hullo!"),
        Visitor::new("bob", "Welcome in!"),
        Visitor::new("joe", "Your drink is ready."),
    ];
    let visitor = visitors.iter().find(|visitor| visitor.name == your_name);
    match visitor {
        Some(visitor) => {
            println!("{:?}", visitor);
            visitor.greet()
        }
        None => println!("Sorry, you are not welcome!"),
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
