use std::fmt;

enum Number {
    First(i32),
    Second(f32),
    Third(i32, i32, i32)
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match *self {
            Number::First(x) => format!("[{}]", x),
            Number::Second(x) => format!("[{}]", x),
            Number::Third(x, y, z) => format!("[{}, {}, {}]", x, y, z),
        };
        write!(f, "{}", r)
    }
}

fn main() {
    let first = Number::First(12);
    let second = Number::Second(12.23);
    let third = Number::Third(1, 2, 3);

    println!("{}, {}, {}", first, second, third);
}
