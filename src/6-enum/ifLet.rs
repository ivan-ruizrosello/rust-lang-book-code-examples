#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_u8_value = Some(0u8);

    // can b VERBOSE
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    /**
     * Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
     * In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
     */
    if let Some(3) = some_u8_value {
        println!("Three");
    }

    let coin = Coin::Quarter(UsState::Alabama);
}

fn match_coin(c: &Coin) -> bool {
    match c {
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            false
        }
        _ => true,
    }
}

fn if_let_coin(c: &Coin) -> bool {
    if let Coin::Quarter(us_state) = c {
        println!("State quarter from {:?}!", us_state);
        false
    } else {
        true
    }
}
