fn call_fn_once<F>(func: F)
    where F: FnOnce(String) {
    let v = ", World!!".to_string();
    func(v);
}


fn call_func<F>(mut func: F)
where
    F: FnMut(String),
{
    let v = ", World!!".to_string();
    func(v);
}

fn fnmut_type() {
    let mut word = "Hello".to_string();
    let func = |next_word: String| {
        word += &next_word; 
    };
    call_func(func);
    println!("Variable value is updated - {}", word);
}


fn fn_once_type() {
    let mut word = "Hello".to_string();
    let func = |next_word: String| {
        word += &next_word; 
    };
    call_fn_once(func);
    println!("Variable value is updated - {}", word);
}

fn fn_type() {
    let func = |word: String| println!("{}", word);
    call_func(func);
    call_fn_once(func);
}

fn main() {
    fn_type();
    fnmut_type();
    fn_once_type();
}
