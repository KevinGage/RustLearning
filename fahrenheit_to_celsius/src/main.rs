use std::io;

fn main() {
    println!("Input a temperature in fahrenheit");

    let mut temp_f = String::new();

    io::stdin()
        .read_line(&mut temp_f)
        .expect("Failed to read line");

    let temp_f: f32 = temp_f.trim().parse().expect("Temperature must be a number");

    println!("Temp in f: {}", temp_f);

    let temp_c = (temp_f - 32.0) / 1.8;

    println!("Temp in c: {}", temp_c);
}
