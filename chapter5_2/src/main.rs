fn main() {
//    tuple way
    let rect = (300000000000000000, 500000000000000000);
    println!("The area of the square is {}", area(rect));
//    struct way
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the square is {}", rect1.area());

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32

}

fn area(dimensions: (i64, i64)) -> i32 {
    dimensions.0 * dimensions.1
}

impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height

    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
