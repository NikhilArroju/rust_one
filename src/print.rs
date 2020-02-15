pub fn run() {
    println!("This is in print.rs file");
    // postitional arguments
    println!("{0} is from {1} {1}", "Geralt", "Rivia");
    //Named arguments
    println!(
        "This is named example\n{name} is from {place}",
        name = "Yennefer",
        place = "Vengerberg"
    );
    //Place holder traits
    //for git testing
    println!(
        "{0} in binary:{0:b}\nin hexadecimal : {0:x}\nin octal : {0:o}",
        1111
    );
    //debug trait
    println!("{:?}", (11, true, "test"));
    //math
    println!("{}", 10 * 2314)
}
