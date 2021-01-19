
fn main() {
    (0..std::u16::MAX)
        .filter(|&port| std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port)).is_ok())
        .for_each(|port| println!("Port {} is open", port));
}
