fn main() {
    let celsius_temp: f32 = 27.0;
    let farenheit_temp = celsius_to_farenheit(celsius_temp);

    println!(
        "C: {} | F: {}", 
        farenheit_to_celsius(farenheit_temp), 
        celsius_to_farenheit(celsius_temp)
    );
}

fn farenheit_to_celsius (f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit (c: f32) -> f32 { 
    c * 1.8 + 32.0
}