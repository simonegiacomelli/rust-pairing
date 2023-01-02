fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse::<IpAddr>()
        .expect("Hardcoded IP address should be valid");

    println!("{home}")
}
