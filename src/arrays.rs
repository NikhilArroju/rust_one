//fixed length of same datatype
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 50];
    println!("{:?}", numbers);
    println!("{}", numbers[4]);

    numbers[0] = 1213;
    println!("{:?}", &numbers[0..2]); //should use & find out why

    //arrays are stack allocated
    println!("{} bytes", std::mem::size_of_val(&numbers)); //we need to pass a reference &

    //slices
    let slice: &[i32] = &numbers[0..3];
    println!("{:?}", slice);
}
