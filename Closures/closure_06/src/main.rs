struct Closure<'a> {
    s : String,
    t : &'a String,
}

impl<'a> FnOnce<()> for Closure<'a> {
    type Output = String;
    fn call_once(mut self, _args : ()) -> Self::Output {
        self.s += &*self.t;
        self.s
    }
}

fn take_closure(value: impl FnOnce() -> String) {
    println!("Value is: {}", value());
}

fn main(){
    let val = String::from("world!");
    let closure = Closure { s: String::from("hello "), t: &val };
    take_closure(closure);
}
