
// T is a type parameter name.
// We can only use types whose values can be ordered. 

// Monomorphization: Rust compiler implements the generic method by all 
// types it is used with throughout the program.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 2500, 100, 65];
    let result: &i32 = largest(&number_list);
    println!("Largest number is: {}", result);
}