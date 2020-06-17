fn main() {
    another_function(5, 5.5);

    let a = 1;

    let b = { 
        let a = 2;
        a + 1
    };

    println!("a = {}  | b = {} | plus_one(a) = {}", a, b, plus_one(a));
}

fn another_function (x: i32, y: f32) { 
    println!("Another fn   x={}  y={}", x, y);
}

fn plus_one (n: i32) -> i32 { 
    n + 1 // sin parentesis es un return implicito
    // n + 1 == return n + 1;
}



fn fibonacci_sucesion (nth_start: i32, nth_end: i32) { 
    for i in nth_start..nth_end { 
        println!("{}", fibonacci(i))
    }
}

fn fibonacci (nth: i32) -> i32{
    if 0 == nth { 
        return 0;
    } else if 1 == nth { 
        return 1;
    }

    return fibonacci(nth - 1) + fibonacci(nth - 2);
}