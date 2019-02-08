fn main() {
    enum IpAddrKind {
        V4,
        V6
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);



    fn route(ip_type: IpAddrKind) {

    }

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);



//    struct IpAddr {
//        kind: IpAddrKind,
//        address: String
//    }
//
//    let home = IpAddr {
//        kind: IpAddrKind::V4,
//        address: String::from("127.0.0.1")
//    };
//
//    let loopback = IpAddr {
//        kind: IpAddrKind::V6,
//        address: String::from("::1")
//    };



//    enum IpAddr {
//        V4(String),
//        V6(String)
//    }
//
//    let home = IpAddr::V4(String::from("127.0.0.1"));
//
//    let loopback = IpAddr::V6(String::from("::1"));



//    enum IpAddr {
//        V4(u8, u8, u8, u8),
//        V6(String)
//    }
//
//    let home = IpAddr::V4(127, 0, 0, 1);
//
//    let loopback = IpAddr::V6(String::from("::1"));



    use std::net::{Ipv4Addr, Ipv6Addr};

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr)
    }



    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}





