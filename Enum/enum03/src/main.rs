use std::fmt;

enum Calculation {
    Parameters {x: i32, y: i32, func: Box<dyn Fn(&i32, &i32) -> i32>}
}

impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match self {
            Calculation::Parameters { x, y, func } => func(x, y)
        };
        write!(f, "{}", r)
    }
}

fn main() {
    let calc = Calculation::Parameters {x: 12, y: 23, func: Box::new(|&x, &y| x + y)};

    println!("{}", calc);
}
