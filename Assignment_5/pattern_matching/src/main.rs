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
    X_axis(Coordinate, Coordinate),
    Y_axis(Coordinate, Coordinate),
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
}
/// quadrant_positions function is used check the Quadrant of the given point.
///
/// #Arguments
///
/// A point is tuple object of integer type.
///
/// #Return
///
/// No Returns.
fn quadrant_positions(point: (i32, i32)) {
    match point {
        (x_axis, y_axis) if x_axis == 0 && y_axis == 0 => println!(
            "Position::Origin(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis == 0 && y_axis != 0 => println!(
            "Position::Fourth(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis != 0 && y_axis == 0 => println!(
            "Position::Fourth(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis > 0 && y_axis > 0 => println!(
            "Position::First(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis < 0 && y_axis > 0 => println!(
            "Position::Second(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis < 0 && y_axis < 0 => println!(
            "Position::Third(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        (x_axis, y_axis) if x_axis > 0 && y_axis < 0 => println!(
            "Position::Fourth(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            x_axis, y_axis
        ),
        _ => println!("Incorrect"),
    }
}
/// Main function print the Coordinate numbers.
///
/// #Arguments
///
/// calling quadrant_positions function for check_position1 and check_position2
///
/// #Return
///
/// No Returns.
fn main() {
    let check_position1 = (2, -2);
    let check_position2 = (-1, -2);
    quadrant_positions(check_position1);
    quadrant_positions(check_position2);
}
