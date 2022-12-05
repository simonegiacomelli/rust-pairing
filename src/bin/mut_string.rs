fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(" added");
    let len = calculate_length(&mut s1);
    s1.push_str(" added2");
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length( s: &mut String) -> usize {
    s.push_str(" cl");
    s.len()
}