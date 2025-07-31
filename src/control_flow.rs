pub fn run() {
    println!("--- Control Flow Examples ---");
    if_statement();
}

fn if_statement() {
    let number = 7;

    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is equal to 5");
    } else {
        println!("Number is greater than 5");
    }

    //if is an expression, so it can be used in a let statement
    let result = if number > 5 {
        "big"
    } else {
        "small"
    };

    println!("Result: {}", result);
}