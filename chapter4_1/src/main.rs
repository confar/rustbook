
fn calculate_lenght(s: String) -> (String, usize)  {
    (s, s.len())
}

fn main() {
    // create value
    let s1 = String::from("hello");
    // borrow it in another function so that s1 drops implicitly
    let (s2, s2_lenght) = calculate_lenght(s1);
    println!("string {} and lenth is {}", s2, s2_lenght);
}
