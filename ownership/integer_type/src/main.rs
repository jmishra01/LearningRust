mod integer_type {
    pub fn run() {
        let mut salary: f32 = 1000.0;
        println!("Current salary {}", salary);

        increment(&mut salary);

        println!("Salary after increment {}", salary);
    }

    fn increment(salary: &mut f32) {
        *salary *= 1.10;
    }
}

mod string_type {
    pub fn run() {
        let name = String::from("Hello");
        {
            let hello = name;
        }
        println!("name {}", name);
    }
}


fn main() {
    integer_type::run();
}
