fn call_func<F>(mut func: F) 
where F: FnMut(String)
{
    let v = ", World!!".to_string();
    func(v);
}

fn main() {
    let mut word = "Hello".to_string();
    let func = |next_word: String| {
        word += &next_word;
        println!("{}", word);
    };
    call_func(func);
    // Re-use word variable, word variable is updated.;
    println!("Variable is updated - {}", word);
}
