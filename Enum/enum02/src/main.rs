use std::fmt;

enum Person {
    Name {first: String, last: String}
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match self {
            Person::Name { first, last } => format!("First: {}, Last: {}",
            first.to_uppercase(),
            last.to_uppercase())
        };
        write!(f, "{}", r)
    }
}

fn main() {
    let person = Person::Name { first: "Jitendra".to_string(), last: "Mishra".to_string() };

    println!("{}", person);
}
