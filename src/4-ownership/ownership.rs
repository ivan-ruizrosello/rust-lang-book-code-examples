fn main() {
    let mut s = "hello"; 

    // s += "a"; // Error: binary assignment operation `+=` cannot be applied to type `&str

    let mut st = String::from("Hello!");
    
    st.push('a');
    st += "a";
    println!("{}", st + "H");

    /******
     * 
     */

    {
        let s1 = String::from("hello");
        let s2 = s1;
    
        // println!("{}", s1); // Error: value borrowed to s2
    }
    
    {
        let s1 = String::from("Hello");
        let s2 = s1.clone();

        println!("{} | {}", s1, s2);
    }

    {
        let s1 = String::from("Hello");
        let s2 = takes_and_gives_back(s1);

        println!("{}", s2);
        takes_ownership(s2);
        // println!("{}", s2); // Error: lost ownership
        
    }
}

fn takes_ownership (s: String) { 
    println!("Taking ownership '{}'", s);
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

// Es tedioso tener que devolverlo cada vez, mirar "references / borrowing"
fn calculate_length (s: String) -> (String, usize){ 
    let length = s.len();

    (s, length)
}