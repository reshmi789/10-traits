// Finding the largest number in a list of numbers

fn main() {

    // store a list of integers in the variable number_list
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];
    /*
    We iterate through all the numbers in the list.
    if the current number is greater than the number stored in largest, 
    replace the number in that variable.
    */
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}