fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_len(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem


    /** Error */
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_len(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}
