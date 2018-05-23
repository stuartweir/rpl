enum Unit {Celsius, Fahrenheit}

fn main() {
    let f_temp = temp_conversion(5, Unit::Fahrenheit);
    println!("The f conversion is: {}", f_temp);

    let c_temp = temp_conversion(70, Unit::Celsius);
    println!("The c conversion is: {}", c_temp);

    let fib = fibonacci(10);
    println!("Fibonacci Number: {}", fib);
}

fn temp_conversion(temperature: i32, convert_to: Unit) -> i32 {
    let new_temp = match convert_to {
        Unit::Fahrenheit => (temperature as f32 * (9.0/5.0)) + 32.0,
        Unit::Celsius => (temperature as f32 - 32.0) * (5.0/9.0)
    };
    new_temp as i32
}

fn fibonacci(n: i64) -> i64 {
    let fib = match n {
        0 => n,
        1 => n,
        _ => (fibonacci(n-1) + fibonacci(n - 2))
    };
    fib
}