fn main() {
    let tuple_e: (char, i32, bool) = ('E', 5i32, true);
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);
}