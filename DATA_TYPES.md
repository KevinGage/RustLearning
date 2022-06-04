# Notes on data types

## Scalars (primitives)

### Numbers

Signed / Unsigned Integers i32, u32
`let x: u32 = 5`

Floats
`let x: f32 = 5.1`

### Bool

`let x: bool = true`

### Char

`let x: char = 'a'`

## Compound Types (fixed length collections)

### Tupple

fixed length collection of multiple types
`let tup: (i32, f64, u8) = (500, 6.4, 1);`

### Array

fixed length collection of similar types
`let a: [i32; 5] = [1, 2, 3, 4, 5];`

### Slice

[reference](https://doc.rust-lang.org/book/ch04-03-slices.html)

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

Reference part of a string

```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

String literals are actually slices
`let s = "Im a slice"`

## Complex Types

### Struct

a collection on labaled values (object)
can implement (impl) methods and functions

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

create an instance

```
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

add a method

```
impl User {
    fn name_plus_email(&self) -> String {
        self.username + self.email
    }
}
```

add an associate function

```
impl User {
    fn new(name: String) -> User {
        User {
            active: true,
            username: name,
            email: "default",
            sign_in_count: 0
        }
    }
}
```

### Enum

a collection of types
useful for match

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
```

```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Vector

a collection of similar types that can grow/shrink
`let v: Vec<i32> = Vec::new();`
`let v = vec![1, 2, 3];`

`v.push(5);`

### Hash Map

a collection of key/value pairs that can grow/shrink
`let mut scores = HashMap::new();`
`let mut scores: HashMap<&str, u32> = HashMap::new();`

`scores.insert(String::from("Blue"), 10);`
