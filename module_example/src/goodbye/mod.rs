pub mod casual;

pub fn description() {
    println!("greeting message");
}

pub mod formal { // inline submodule
    pub fn english () {
        println!("hello");
    }

    pub fn spanish() {
        println!("hola");
    }
}

