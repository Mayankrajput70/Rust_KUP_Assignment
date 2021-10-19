#[derive(PartialEq, Eq, Debug)]
/// enum Ip-address and it is String type
enum Ip {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
    ClassE(String),
}
/// check_ip_address which is used check the given ip_Address
///
/// #Arguments
///
/// position is tuple object of unsigned integer type
///
/// #Return
///
/// Returns Correct Ip Position.
fn list(position: (usize, usize, usize, usize)) {
    match position {
        (1..=127, 0..=255, 0..=255, 1..=254) => println!(
            "Ip::ClassA({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (128..=191, 0..=255, 0..=255, 1..=254) => println!(
            "Ip::ClassB({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (192..=223, 0..=255, 1..=254, 1..=254) => println!(
            "Ip::ClassC({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (224..=239, 0..=255, 0..=255, 0..=255) => println!(
            "Ip::ClassD({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        (240..=254, 0..=255, 0..=255, 0..=254) => println!(
            "Ip::ClassE({}.{}.{}.{})",
            position.0, position.1, position.2, position.3
        ),
        _ => println!("Incorrect Number"),
    }
}
/// Main method.
///
/// #Arguments
///
/// calling Address to check class type of ip address
///
/// #Return
///
/// No return.
fn main() {
    let address_1 = (192, 0, 1, 1);
    list(address_1);
    let address_2 = (230, 45, 6, 7);
    list(address_2);
    let address_3 = (170, 45, 23, 45);
    list(address_3);
    let address_4 = (198, 5, 6, 4);
    list(address_4);
}
