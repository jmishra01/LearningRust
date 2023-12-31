use std::{cell::Cell, fmt::Display};

#[derive(Debug)]
struct Person {
    name: String,
    age: Cell<usize>,
}

impl Person {
    pub fn new(name: String, age: usize) -> Self {
        Person {
            name,
            age: Cell::new(age),
        }
    }

    fn set_age(&mut self, age: usize) {
        self.age = age.into();
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Person ({} {})", self.name, self.age.get()))
    }
}

fn main() {
    let mut person = Person::new("abcd".to_string(), 23);

    println!("{}", person);
    person.set_age(25);

    println!("{}", person);
}
