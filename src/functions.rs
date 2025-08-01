pub fn run() {
    println!("--- Functions Examples ---");

    say_hello();
    print_number(42);

    let doubled = doubled_number(5);
    println!("Doubled number: {}", doubled);
}

fn say_hello() {
    println!("Hello from a function");
}

fn print_number(x: i32) {
    println!("The number is: {}", x);
}

fn doubled_number(x: i32) -> i32 {
    x * 2 //no semicolon means this is the return value
}
