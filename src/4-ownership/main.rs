fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_len(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_len(s: &mut String) -> usize {
    s.push_str("a");
    s.len()
}


