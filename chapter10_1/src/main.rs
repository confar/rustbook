
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    &largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(number_list);
    println!("The largest number is {}", result);

    let number_list = vec!["y", "m", "c", "a"];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}