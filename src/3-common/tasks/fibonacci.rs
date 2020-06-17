fn main() {
    println!("loop: {} | recursive: {}", fibonacci(10), fibonacci_recursive(10)); 
 }
 
 fn fibonacci (n: i32) -> i32 { 
     if n == 0 {
         return 0;
     } else if n == 1 {
         return 1
     }
 
     let mut actual = 1;
     let mut next = 1;
 
     for _i in 2..n { 
         let aux = actual;
         actual = next;
         next = aux + actual;
     }
 
     next
 }
 
 fn fibonacci_recursive (n: i32) -> i32 { 
     if n == 0 { 
         return 0;
     } else if n == 1 { 
         return 1;
     } else {
         return fibonacci_recursive(n -1 ) + fibonacci_recursive(n - 2)
     }
 }