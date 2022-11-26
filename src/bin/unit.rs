fn main() {
    println!("one()={:?}", one())
}

fn one() -> () {
    let x = 1;
    x + 1;
}