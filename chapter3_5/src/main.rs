fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
    println!("Converting 32 F to celsius is {}", convert_to_celsius(32));
    println!("first fibonnaci number is {}", get_fibonnacci_num(1));
    println!("first fibonnaci number is {}", get_fibonnacci_num(0));
    println!("sixth fibonnaci number is {}", get_fibonnacci_num(6));
    print_lyrics()
}


fn convert_to_celsius(x: i32) -> i32 {
    (x - 32) * 5 / 9
}

fn get_fibonnacci_num(x: i32) -> i32 {
    if x <= 1 {
        return x }
    else {
        return get_fibonnacci_num(x-1) + get_fibonnacci_num(x - 2)
    }
}


fn print_lyrics() {
//    let gift_list: [str, 3] = [];
    let days = [
        ("first",  "A partridge in a pear tree"),
        ("second",  "Two turtle doves"),
        ("third",  "Three French hens")
    ];
    for (i, (day, _)) in days.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", day);
        println!("{}", i);
        while i != 0 {
            println!("{}", days[i].1);
            let i = i - 1;
        }
//        for i in gift_list.iter() {
//            println!("{}", gift);
//        }
//        gift_list.push(gift)
    }
}


//
//On the first day of Christmas my true love sent to me
//A partridge in a pear tree
//On the second day of Christmas my true love sent to me
//Two turtle doves
//And a partridge in a pear tree
//On the third day of Christmas my true love sent to me
//Three French hens, two turtle doves
//And a partridge in a pear tree
//On the fourth day of Christmas my true love sent to me
//Four calling birds, three French hens, two turtle doves
//And a partridge in a pear tree
//On the fifth day of Christmas my true love sent to me
//Five gold rings, four calling birds, three French hens, two turtle doves
//And a partridge in a pear tree
//On the sixth day of Christmas my true love sent to me
//Six geese a laying, five gold rings, four calling birds
//Three French hens, two turtle doves
//And a partridge in a pear tree
//On the seventh day of Christmas my true love sent to me
//Seven swans a swimming, six geese a laying, five gold rings
//Four calling birds, three French hens, two turtle doves
//And a partridge in a pear tree
//On the eighth day of Christmas my true love sent to me
//Eight maids a milking, seven swans a swimming, six geese a laying
//Five gold rings, four calling birds, three French hens, two turtle doves
//And a partridge in a pear tree
//On the ninth day of Christmas my true love sent to me
//Nine drummers drumming
//On the tenth day of Christmas my true love sent to me
//Ten pipers piping
//Nine drummers drumming, ten pipers piping
//Drumming, piping, drumming, piping
//Eight maids a milking, seven swans a swimming, six geese a laying
//Five gold rings, four calling birds, three French hens, two turtle doves
//And a partridge in a pear tree
//On the eleventh day of Christmas my true love sent to me
//Eleven ladies dancing, ten pipers piping, nine drummers drumming
//Eight maids a milking, seven swans a swimming, six geese a laying
//Five gold rings, four calling birds, three French hens, two turtle doves
//And a partridge in a pear tree
//On the twelfth day of Christmas my true love sent to me
//Twelve Lords a leaping, eleven ladies dancing, ten pipers piping
//Nine, drummers drumming, eight maids a milking
//Seven swans a swimming, six geese a laying
//And five gold rings, four calling birds, three French hens, two turtle doves
//And a partridge in a pear tree, and a partridge in a pear tree