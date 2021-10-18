#[derive(PartialEq, Eq, Debug)]
/// Passing Ip Class Name.
enum Ip {
    ClassA(String),
    ClassB(String),
    ClassC(String),
    ClassD(String),
}
/// This method shows the Ip format.
///
/// #Arguments
///
/// Checking the Position.
///
/// #Return
///
/// Returns Correct Ip Position.
fn list(position: (usize, usize, usize, usize)) -> Result<Ip, String> {
    match position {
        (1..=230, 0..=230, 1..=230, 1..=230) => Ok(Ip::ClassA(format!(
            "{}.{}.{}.{}",
            position.0, position.1, position.2, position.3
        ))),
        (0..=230, 45..=230, 6..=230, 7..=230) => Ok(Ip::ClassB(format!(
            "{}.{}.{}.{}",
            position.0, position.1, position.2, position.3
        ))),
        (0..=230, 0..=230, 10..=230, 45..=230) => Ok(Ip::ClassC(format!(
            "{}.{}.{}.{}",
            position.0, position.1, position.2, position.3
        ))),
        (224..=230, 0..=230, 0..=230, 0..=230) => Ok(Ip::ClassD(format!(
            "{}.{}.{}.{}",
            position.0, position.1, position.2, position.3
        ))),
        _ => Err(String::from("Incorrect Number")),
    }
}
/// This main method print the Ip.
///
/// #Arguments
///
/// Find the Ip number.
///
/// #Return
///
/// Returns successfully print Ip number.
fn main() {
    println!("{:?}", list((192, 0, 1, 1)));
    println!("{:?}", list((230, 45, 6, 7)));
    println!("{:?}", list((170, 45, 23, 45)));
    println!("{:?}", list((198, 5, 6, 4)));
}
