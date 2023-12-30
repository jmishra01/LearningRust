mod with_args {
    fn call_func<F>(func: F)
    where
        F: FnOnce(&str),
    {
        let v = ", World!!";
        func(v);
        // func(v); -- Thourgh error
    }

    pub fn with_args() {
        let word = "Hello";
        let func = |next_word: &str| {
            let combined = format!("{}{}", word, next_word);
            println!("{}", combined);
        };
        call_func(func);
        println!("{}", word); // Re-use word variable
    }
}

mod without_args {
    fn call_func<F>(func: F)
    where
        F: FnOnce(),
    {
        func();
        // func(); -- Thourgh error
    }

    pub fn without_args() {
        let word = "Hello";
        let func = || {
            let combined = format!("{}{}", word, ", World!!");
            println!("{}", combined);
        };
        call_func(func);
        println!("{}", word); // Re-use word variable
    }
}

fn main() {
    with_args::with_args();
    without_args::without_args();
}
