fn main() {
    let phrases = [
        "two turtle doves",
        "three French hens",
        "Four calling birds",
        "Five golden rings",
        "six geese a-laying",
        "Seven swans a-swimming",
        "eight maids a-milking",
        "Nine ladies dancing",
        "ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 { 
        println!("On the first day of Christmas, my true love sent to me");
        for j in (0..i).rev() { 
            println!("{},", phrases[j]);
        }
        println!("A partridge in a pear tree");
        println!()
    }
}