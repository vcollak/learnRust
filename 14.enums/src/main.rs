//derive debug so we can print the IpAddrKind
//this enum simply holds V4 and V6 values
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

//holds the IP address and kind
//structs can hold enums
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//this enum has a string associated with each enumeration
enum IpAddrKindWithStrings {
    V4(String),
    V6(String),
}

//the types associated with enums do not have to all be the same
//this one holds four u8 for V4 and one String for V6
enum IpAddrKindWithMixedTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    /////////////////////////////
    // Enums and Structs together
    /////////////////////////////
    //instantiate ipv4 and ipv6
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //define the address for home
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    //define the address for loopback (ipv6)
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //call route
    route(IpAddrKind::V4);

    //////////////////////////////////
    // Enums with values
    //////////////////////////////////

    //We attach data to each variant of the enum directly, so there is no need for an extra struct.
    let home1 = IpAddrKindWithStrings::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddrKindWithStrings::V6(String::from("::1"));

    //use enums where each enum has different types associated with
    //You can put any kind of data inside an enum variant: strings, numeric types, or structs,
    //for example. You can even include another enum!
    let home2 = IpAddrKindWithMixedTypes::V4(127, 0, 0, 1);
    let loopback2 = IpAddrKindWithMixedTypes::V6(String::from("::1"));
}

//just prints the IpAddrKind
fn route(ip_kind: IpAddrKind) {
    println!("Routing {:?}", ip_kind);
}
