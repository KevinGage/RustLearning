use core::hash::Hash;
use std::cmp::Eq;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Hash + Eq + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(2);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}

// Try modifying Cacher to hold a hash map rather than a single value.
// The keys of the hash map will be the arg values that are passed in,
// and the values of the hash map will be the result of calling the closure on that key.
// Instead of looking at whether self.value directly has a Some or a None value,
// the value function will look up the arg in the hash map and return the value if it’s present.
// If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.

// The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32.
// We might want to cache the results of closures that take a string slice and return usize values, for example.
// To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.
