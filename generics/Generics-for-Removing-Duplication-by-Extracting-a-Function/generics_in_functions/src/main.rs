// Two functions that differ only in their names and the types in their signatures

/*
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
*/

// parameter names in Rust are short
// Short for “type,” T is the default choice of most Rust programmers.
/*
the function largest is generic over some type T. This function has one parameter named list, 
which is a slice of values of type T. 
The largest function will return a value of the same type T
*/
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
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


//the body of largest won’t work for all possible types that T could be. 
// Because we want to compare values of type T in the body, 
// we can only use types whose values can be ordered. 

// Solution -
// To enable comparisons, the standard library has 
// the std::cmp::PartialOrd trait that you can implement on types 