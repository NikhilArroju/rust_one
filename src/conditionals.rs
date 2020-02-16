pub fn run() {
    let age: u8 = 27;
    let check_id: bool = true;

    //If else
    if age >= 21 && check_id {
        println!("what would you like to drink");
    } else if age <= 21 && check_id {
        println!("please leave");
    } else {
        println!("need an id");
    }

    //shord hand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}
