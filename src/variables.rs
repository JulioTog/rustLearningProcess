pub fn run() {
    println!("--- Variables Examples ---");
    immutability_example();
    mutability_example();
    integers_example();
    floats_example();
    booleans_example();
    characters_example();
    constants_example();
    shadowing_example();
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

fn floats_example() {
    let x = 2.5; //inferred to be f64
    let y: f32 = 3.14; //explicit type

    println!("Floats: x = {}, y = {}", x, y);
}

fn booleans_example() {
    let is_rust_fun = true;
    let is_python_slow = false;

    println!("Booleans: is_rust_fun = {}, is_python_slow = {}", is_rust_fun, is_python_slow);

}

fn characters_example() {
    let letter = 'R';
    let emoji = '🚀';
    let symbol = '©';

    println!("Characters: letter = {}, emoji = {}, symbol = {}", letter, emoji, symbol);
}

fn constants_example() {
    const MAX_USERS: u32 = 100;
    println!("Constants: MAX_USERS = {}", MAX_USERS);
}

fn shadowing_example() {
    let x = 5;
    let x = x + 1;
    let x = format!("x is now: {}", x); //shadowing with a new value

    println!("Shadowing: x = {}", x);
}