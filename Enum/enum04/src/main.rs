use std::fmt::Display;


enum Animal {
    Tiger(String),
    Lion(String),
    City(String),
    Place(String)
}

impl Display for Animal {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
            Animal::Place(x) =>  println!("Place - {}", x),
            Animal::Lion(x) =>  println!("Lion - {}", x),
            Animal::City(x) =>  println!("City - {}", x),
            Animal::Tiger(x) =>  println!("Tiger - {}", x),
    };
        write!(f, "{}", "hello")
}
}


fn show(animal: Animal) {
    println!("{}", animal);
}


fn main() {
    let city_name =Animal::City("Bhopal".to_string());
    println!("{}", city_name);
    show()
}
