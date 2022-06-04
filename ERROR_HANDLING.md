# Notes on error handling

## Panic

If there is a situation where the program should immediately halt throw panic
`panic!("crash and burn")`

## Option<T>

The Option<T> enum is used when a function can return something or None (rusts equivilent to null)

```
pub enum Option<T> {
    None,
    Some(T),
}
```

### Using Option<T>

Match is a good way to deal with functions that return Option<T>

```
let name = String::from("naufil");
println!(
    "Character at index 6: {}",
    match name.chars().nth(6) {
        Some(c) => c.to_string(),
        None => "No character at index 6!".to_string(),
    }
)
```

## Result<T, E>

Result<T, E> is an enum with a type that can return data and a type that can return an error

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Handing a Result<T, E>

When calling a function that returns a Result<T, E> match is a good way to handle the result

```
let f = File::open("hello.txt");

let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

### Returning a Result<T, E>

If a function should return either a result or an error then a Result<T,E> should be the return type.

```
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

## the ? Operator

Use a ? operator on a Result<T, E> or Option<T> to either get the value, or immediately return from the function with the error
The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.
For example if a function returns an Option<T>, ? can't be used internally on a function that returns Result<T, E>
There is a way to convert between Options and Results but I haven't learned them yet.

```
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

## Unwrap

Call .unwrap() on a Result<T,E> to return the data, or panic if there is an error
`let f = File::open("hello.txt").unwrap();`

## Expect

Call .expect() on a Result<T, E> to return the data, or panic with a particular message if there is an error
`let f = File::open("hello.txt").expect("Failed to open hello.txt");`
