fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100000;
    println!("The value of const is: {}", MAX_POINTS);

    let spaces = "   ".len();
//    let spaces = spaces.len();
    println!("The len of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is: {}", guess);


}