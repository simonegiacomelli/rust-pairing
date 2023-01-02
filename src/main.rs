#[cfg(test)]
mod tests {
    #[test]
    fn test_in_root() {
        let result = 2 + 2;
        assert_eq!(result, 5);
    }
}

fn main() {
    println!("hello main!")
}