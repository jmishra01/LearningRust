#![allow(dead_code, unused_variables)]
use std::future::Future;


async fn foo() -> usize {
    0
}

fn bar() -> impl Future<Output = usize> {
    async {
        println!("foo1");
        foo().await;
        println!("foo2");
        0
    }
}

fn main() {
    println!("Hello, world!");
    let x = foo();
    let y = bar();
}
