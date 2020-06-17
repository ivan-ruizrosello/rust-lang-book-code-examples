fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let heart_eyed_cat = '😻';
    println!(" {} ",heart_eyed_cat);


    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, _, _) = tup;
    println!("{:?} | {} | tup.1 = {:?}", tup, x, tup.1);


    let arr: [i32; 5] = [1,2,3,4,5];
    let arr2 = [3; 5]; // [3,3,3,3,3]
}