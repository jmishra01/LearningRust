mod greeting;
mod goodbye;

fn main() {
    println!("Hello, world!");


    // Greeting
    greeting::description();

    greeting::casual::english();
    greeting::casual::spanish();

    greeting::formal::spanish();
    greeting::formal::english();

    // Goodbye
    goodbye::description();

    goodbye::casual::english();
    goodbye::casual::spanish();

    goodbye::formal::spanish();
    goodbye::formal::english();


}
