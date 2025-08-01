pub fn run() {
    println!("--- Control Flow Examples ---");
    if_statement();
    loop_statement();
    while_statement();
    for_statement();
    match_statement();
    range_match_statement();
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

fn loop_statement() {
    let mut count = 0;

    //loop is an infinite loop unless we break out of it (with break or continue)
    loop {
        println!("count: {}", count);
        count += 1;

        if count >= 3 {
            break;
        }
    }
}

//while is used when we know how many times we want to loop
//loop as long as the condition is true
fn while_statement() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//for is used when we want to iterate over a collection of items
fn for_statement() {
    //1..4 is a range, it does not include the last number
    //1..=4 is a range that includes the last number
    for number in 1..4 {
        println!("{}", number);
    }
}

fn match_statement() {
    let number = 5;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"), //_ is a catch all pattern
    }
}

fn range_match_statement() {
  let score = 88;

  match score {
    90..=100 => println!("A"),
    80..=89 => println!("B"),
    70..=79 => println!("C"),
    0..=69 => println!("D Or F"),
    _ => println!("Invalid score"),
  }

  //binding a matched value 
  let num = 7;

  match num {
    val @ 1..=5 => println!("Low number: {}", val),
    val @ 6..=10 => println!("medium number: {}", val),
    _ => println!("other"   ),
  }
}