#[derive(PartialEq, Eq, Debug)]
/// Passing coordinate.
enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}
#[derive(PartialEq, Eq, Debug)]
/// enum Position.
enum Positions {
    Origin(Coordinate, Coordinate),
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
}
/// This method shows the Quadrant Position.
///
/// #Arguments
///
/// Checking the Position.
///
/// #Return
///
/// Returns positions lies on which quadrant.
fn quadrant_positions(point: (i32, i32)) -> Result<Positions, String> {
    match point {
        (x_axis, y_axis) if x_axis == 0 && y_axis == 0 => Ok(Positions::Origin(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis > 0 && y_axis > 0 => Ok(Positions::First(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis > 0 => Ok(Positions::Second(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis < 0 => Ok(Positions::Third(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        (x_axis, y_axis) if x_axis > 0 && y_axis < 0 => Ok(Positions::Fourth(
            Coordinate::Abscissa(point.0),
            Coordinate::Ordinate(point.1),
        )),
        _ => Err(String::from("Incorrect Positions")),
    }
}
/// This main method print the Coordinate numbers.
///
/// #Arguments
///
/// Find the Quadrant.
///
/// #Return
///
/// Returns successfully print quadrant position.
fn main() {
    let check_position = (2, -2);
    let check_position2 = (-1, -2);
    println!("{:?}", quadrant_positions(check_position));
    println!("{:?}", quadrant_positions(check_position2));
}
