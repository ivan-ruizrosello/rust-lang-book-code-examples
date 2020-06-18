fn main() {
    let reference_to_nothing = dangle();

    let s = no_dangle();
}

fn dangle () -> &String { 
    let s = String::from("Hello");

    &s
}

fn no_dangle () -> String { 
    let s = String::from("hell-o");

    s
}