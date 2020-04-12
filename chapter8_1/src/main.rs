fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3];
    println!("the vector is {:?}", v);

    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.extend([1,2,4].iter());
    println!("the vector is {:?}", v1);

    let third = v1[2];
    println!("the third value is {}", third);

    v1.pop();
    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = v[0];

    v.push(6);

    println!("The first element is: {}", first);
    for i in &mut v {
        println!("{}", i);
    }

}
