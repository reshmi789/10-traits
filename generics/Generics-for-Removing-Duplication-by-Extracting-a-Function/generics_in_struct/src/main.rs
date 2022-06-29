/*
Point<T> struct to hold x and y coordinate values of any type.
*/

// First, we declare the name of the type parameter inside angle brackets 
// just after the name of the struct
// Then we use the generic type in the struct definition where we would otherwise 
// specify concrete data types.

#[derive(Debug)]

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?}", integer);
    println!("{:?}", float);
    // let wont_work = Point { x: 5, y: 4.0 };
    //     println!("{:?}", wont_work);
    // fields x and y must be the same type because both have the same generic data type T.
}
