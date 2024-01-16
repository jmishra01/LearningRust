async fn say_world() {
    println!("World");
}

#[tokio::main]
async fn main(){
    let op = say_world();
    println!("Hello");
    op.await;
}
