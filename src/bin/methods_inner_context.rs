struct Point(i32, i32);

fn main() {
    let p = Point(3, 2);

    impl Point {
        fn mul(&self) -> i32 { self.0 * self.1 }
    }

    println!("{}", p.mul());
}