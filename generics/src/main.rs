// https://doc.rust-lang.org/book/ch10-02-traits.html

// Original code
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// We could also implement largest by having the function return a reference to a T value in the slice.
// If we change the return type to &T instead of T, thereby changing the body of the function to return a reference,
// we wouldn’t need the Clone or Copy trait bounds and we could avoid heap allocations.
// Try implementing these alternate solutions on your own!
// If you get stuck with errors having to do with lifetimes, keep reading: the “Validating References with Lifetimes” section coming up will explain,
// but lifetimes aren’t required to solve these challenges.

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
