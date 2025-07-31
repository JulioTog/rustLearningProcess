pub fn run() {
    println!("--- Variables Examples ---");
    immutability_example();
    mutability_example();
    integers_example();
    // more to come...
}

fn immutability_example() {
    let x = 5;
    println!("Immutability: x = {}", x);
}

fn mutability_example() {
    let mut x = 5;
    x = 6;
    println!("Mutability: x = {}", x);
}

fn integers_example() {
    let a = 10; //inferred to be i32
    let b: i64 = 20; //explicit type
    let c = -5; //negative integer
    println!("Integers: a = {}, b = {}, c = {}", a, b, c);
}