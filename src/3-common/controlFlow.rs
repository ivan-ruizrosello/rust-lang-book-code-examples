fn main() {
    let number = 10;

    if number < 10 {
        println!("menor");
    } else {
        println!("mayor o igual");
    }

    let number = if true { 5 } else { 6 };

    println!("if variable init = {}", number);

    println!("loop return {}", return_loop());

    while_loop();

    for_loop();

    for number in (3..5).rev() {
        println!("{}!", number);
    }
}

fn return_loop() -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    return result;
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
