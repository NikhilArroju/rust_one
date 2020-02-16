pub fn run() {
    greeting("hello", "jane");
    let get_sum = add(3, 333);
    println!("{}", get_sum);

    //Closure
    let n3 = 10;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3; //we can also use outside vairables
    println!("{}", add_num(1, 2));
}
fn greeting(greet: &str, name: &str) {
    println!("{0}!,nice to meet you {1}", greet, name)
}
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
