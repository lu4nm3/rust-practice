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



    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}





