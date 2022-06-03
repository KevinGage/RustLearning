use std::io;

fn main() {
    println!("Enter number of fibonacci iterations");

    let mut fib_count = String::new();

    let fib_count: u128 = loop {
        fib_count.clear();

        io::stdin()
            .read_line(&mut fib_count)
            .expect("Failed to read line");

        match fib_count.trim().parse() {
            Ok(num) => {
                if num < 1 {
                    println!("number must be greater than 0");
                } else {
                    break num;
                }
            }
            Err(_) => println!("invalid number: {}", fib_count),
        };
    };

    let mut num1: u128 = 0;
    let mut num2: u128 = 1;
    let mut num3: u128 = 0;

    for _number in 0..fib_count {
        num3 = num1 + num2;
        num1 = num2;
        num2 = num3;
    }

    println!("fib num: {}", num3);
}
