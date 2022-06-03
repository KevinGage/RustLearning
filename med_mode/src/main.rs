// Given a list of integers,
// use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
use rand::Rng;
use std::collections::HashMap;

fn print_median(input: &[u32]) {
    let mut v = input.to_vec();

    v.sort();

    let middle = v.len() / 2;

    match v.get(middle) {
        Some(m) => println!("Median:  {}", m),
        None => println!("No median found..."),
    }
}

fn print_mode(input: &[u32]) {
    let mut map: HashMap<&u32, u32> = HashMap::new();

    for num in input {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    println!("Map: {:?}", map);

    let top_char = map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    println!("Mode: {}", top_char.0);
}

fn main() {
    let list1 = [1, 2, 3, 4, 5];
    let list2 = [5, 4, 3, 2, 1];
    let list3 = [5, 1, 4, 2, 3];
    let list_rand = [
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
        rand::thread_rng().gen_range(1..11),
    ];

    println!("List: {:?}", list1);
    print_median(&list1);
    print_mode(&list1);

    println!("List: {:?}", list2);
    print_median(&list2);
    print_mode(&list2);

    println!("List: {:?}", list3);
    print_median(&list3);
    print_mode(&list3);

    println!("List: {:?}", list_rand);
    print_median(&list_rand);
    print_mode(&list_rand);
}
