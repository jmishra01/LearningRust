fn call_func<F>(func: F) 
where F: Fn()
{
    func();
}

fn main() {
    let word = "Hello, World!!".to_string();
    let func = || println!("{}", word);
    call_func(func);

    // Re-use word variable;
    println!("{}", word);
}
