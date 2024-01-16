fn main() {
    println!("Hello, world!");
    let lt = vec!["hello world", "", "bhopal jabalpur"];
    for i in lt {
        let x = i.split(" ");
        println!("Length = {:#?}", i.len());
        println!("{:#?}", x.clone().count());
    }
}
