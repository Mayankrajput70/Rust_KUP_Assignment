/// Complex number structure
pub struct Complex {
    real: f64,
    img: f64,
}
fn main() {
    let complex_1 = Complex {
        real: 50.0,
        img: 80.0,
    };
    let complex_2 = Complex {
        real: 20.0,
        img: 80.0,
    };
    addition(&complex_1, &complex_2);
    subtraction(&complex_1, &complex_2);
    multiplication(&complex_1, &complex_2);
}
/// addition function add two  complex numbers using structures.
///
/// #Arguments
///
/// Reference of two complex number.
///
/// #Return
///
/// No return.
fn addition(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!("Sum of the complex numbers = {} + {}i",
            complex_1.real + complex_2.real,
            complex_1.img + complex_2.img
        );
    } else {
        println!("Sum of the complex numbers = {} {}i",
            complex_1.real + complex_2.real,
            complex_1.img + complex_2.img
        );
    }
}
/// subtraction function subtract two complex numbers using structures.
///
/// #Arguments
///
/// Reference of two complex number.
///
/// #Return
///
/// No return.
fn subtraction(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!("Difference of the complex numbers = {} + {}i",
            complex_1.real - complex_2.real,
            complex_1.img - complex_2.img
        );
    } else {
        println!("Difference of the complex numbers = {} {}i",
            complex_1.real - complex_2.real,
            complex_1.img - complex_2.img
        );
    }
}
/// multiplication function multiply two complex numbers using structures.
///
/// #Arguments
///
/// Reference of two complex number.
///
/// #Return
///
/// No return.
fn multiplication(complex_1: &Complex, complex_2: &Complex) {
    if complex_1.img + complex_2.img >= 0.0 {
        println!("Multiplication of the complex numbers = {} + {}i",
            complex_1.real * complex_2.real,
            complex_1.img * complex_2.img
        );
    } else {
        println!("Multiplication of the complex numbers = {} {}i",
            complex_1.real * complex_2.real,
            complex_1.img * complex_2.img
        );
    }
}