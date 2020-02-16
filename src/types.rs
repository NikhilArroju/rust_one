/*
Primitive Types-----------------
Integers : u8, i8, u16, i16, u32, i32, u64..till 128
floats : f32, f64
Boolean bool
Characters char
Tuples
Arrays
*/

pub fn run() {
    //for datatypes
    //defult i32,f64
    let a = 2;
    let u: u32 = 43; // u cannot be negative u
    let b = 0.2;
    let ch = 'p';
    let emoji = '\u{1F600}';
    println!("{}{}", ch, emoji);
    println!("{:?}", (a, b, u));
    println!("Max size of i32 is {}", std::i32::MAX);
    println!("Max size of i64 is {}", std::i64::MAX);
    println!("Max size of f32 is {}", std::f32::MAX);
    println!("Max size of f64 is {}", std::f64::MAX);
    println!("{} {}", true, 9 < 7);
}
