
fn print_sum(v: ref::Vec<i32>)  {
    println!("{}", v[0] + v[1]);
    // v is dropped and deallocated here
    return v
}

fn main() {
    let mut v = Vec::new(); // creating the resource
    for i in 1..1000 {
        v.push(i);
    }
    // at this point, v is using
    // no less than 4000 bytes of memory
    // -------------------
    // transfer ownership to print_sum:
    let a = print_sum(&v);
    // we no longer own nor anyhow control v
    // it would be a compile-time error to try to access v here
    println!("{:?}", a);
    println!("We're done");
    // no deallocation happening here,
    // because print_sum is responsible for everything
}