# Notes on smart pointers

## Box<T>

Allows storing data on the heap instead of stack.

When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

## Rc<T>

To enable multiple ownership. Note Arc<T> is the thread safe version of Rc<T>

```
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

## RefCell<T>

Enforces borrowing rules at runtime instead of compile time.
you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
Note Mutex<T> is similar but thread safe

```
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```
