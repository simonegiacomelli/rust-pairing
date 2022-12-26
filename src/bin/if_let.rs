fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    if Some(3u8) == config_max {
        println!("config_max == 3u8");
    }
}
