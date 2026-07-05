fn main() {
    // fn route(ip_kind: IpAddrKind) {}

    // route(four);
    // route(six);
    #[derive(Debug)]
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind::V6(String::from("::1"));

    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));

    println!("Home IP Address: {:?}", _home);
    println!("Loopback IP Address: {:?}", _loopback);

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: four,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: six,
    //     address: String::from("::1"),
    // };
}
