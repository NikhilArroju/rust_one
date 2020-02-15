//variables hold primitive data or a reference to data
//variables are immutable by default
//Rust is a block scoped language

pub fn run() {
    let name = "Geralt";
    let mut age = 128;
    println!("{} is {}", name, age);
    age = 130;
    println!("{} is {}", name, age);

    //Const
    const ID: i32 = 0987; //you have to explicitly mention the datatype for const
    println!("ID is {}", ID);
    //multiple
    let (name, age) = ("yen", 111);
    println!("{} is {}", name, age);
}
