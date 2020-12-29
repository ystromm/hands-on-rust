use crate::VisitorAction::{AcceptWithNote, Refuse};
use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the three house {}!", self.name),
            VisitorAction::AcceptWithNote { .. } => {
                println!("Welcome to the three house {}!", self.name);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}.", self.name);
                }
            }
            Refuse => println!("Do not allow {} in!", self.name),
            VisitorAction::Probation => {
                println!("{} is now a probationary member!", self.name);
            }
        }
    }
}

fn main() {
    let mut visitors = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new(
            "Bob",
            AcceptWithNote {
                note: String::from("Milk in the fridge!"),
            },
            15,
        ),
        Visitor::new("Joe", Refuse, 30),
    ];

    loop {
        println!("Hello, what's your name (leave empty to quit)?");
        let your_name = what_is_your_name();
        println!("Hello, {}!", capitalize_first(&your_name));
        let visitor = visitors.iter().find(|visitor| visitor.name == your_name);
        match visitor {
            Some(visitor) => {
                println!("{:?}", visitor);
                visitor.greet()
            }
            None => {
                if !your_name.is_empty() {
                    println!("{} is not on the list.", your_name);
                    visitors.push(Visitor::new(&your_name, VisitorAction::Probation, 0));
                } else {
                    break;
                }
            }
        }
    }
    println!("{:#?}", visitors);
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
