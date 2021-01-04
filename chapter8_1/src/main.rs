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


        // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
    //
    // let mut v1: Vec<i32> = Vec::new();
    // v1.push(1);
    // v1.extend([1,2,4].iter());
    // println!("the vector is {:?}", v1);
    //
    // let third = v1[2];
    // println!("the third value is {}", third);
    //
    // v1.pop();
    // match v.get(3) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }
    //
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = v[0];
    //
    // v.push(6);
    //
    // println!("The first element is: {}", first);
    // for i in &mut v {
    //     println!("{}", i);
    // }

}
