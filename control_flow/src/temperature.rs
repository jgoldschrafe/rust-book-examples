fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.) / 1.8
}

fn main() {
    println!("212F is {}C", fahrenheit_to_celsius(212.));
    println!("0C is {}F", celsius_to_fahrenheit(0.));
}