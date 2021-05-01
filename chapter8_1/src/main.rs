#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    let v = vec![1, 2, 3];
    println!("the vector is {:?}", v);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
            println!("{}", i);
            *i += 50;
        println!("{}", i);
        }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:#?}", i);
    }

    let mut x: &str = "32";
    let r: &mut str = &mut x;
    *r.push_str("31");
    println!("{:?}", r)

    // Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.


    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

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
