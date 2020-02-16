//primitive str is immutable fixed length in memeory
//String is growable heap allocated structure data structure

pub fn run() {
    let mut s = String::from("Hello ");
    println!("{}", s);
    s.push('W');
    s.push_str("orld!");
    println!("{}", s.capacity());
    let mut s1 = String::with_capacity(10);
    s1.push('t');
    s1.push_str("sdfas");
    println!("len is {} and capacity is {}", s1.len(), s1.capacity());

    // Assertion testing
    assert_eq!(7, s1.len()); //only shows error when wrong
    assert_eq!(10, s1.capacity());
}
