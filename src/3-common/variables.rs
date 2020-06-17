const MAX_POINTS: i32 = 100_000;

fn main() {
    // SHADOWING
    let x = MAX_POINTS;

    let x = x + 1;

    let x = x * 2;

    println!("X value = {}", x);


    // SHADOWING variable type casting
    let chars = "123";
    println!("spaces = {}", chars);
    let chars = chars.len();
    println!("spaces = {}", chars);
}
