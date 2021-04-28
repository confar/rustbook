fn main() {
    let mut s = String::from("Hello, world!");
    let s1 = String::from("Hello");
    let word =  first_word(&s);
    println!("first word is {}", word);
    println!("first word is {}", first_word(&s1));
    s.clear();
    println!("string is {}", s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}
