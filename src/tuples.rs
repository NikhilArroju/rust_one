//tuples group diff vales
// Max is 21 elements (Weird check it out)
pub fn run() {
    let t: (&str, &str, u8) = ("Geralt", "Rivia", 127);
    println!("{} of {} is {}", t.0, t.1, t.2);
}
