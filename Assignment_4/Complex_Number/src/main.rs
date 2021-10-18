/// Numbers Structure
struct Numbers {
    num_first: u32,
    num_second: u32,
}
/// Main method print the complex numbers.
///
/// #Arguments
///
/// complex of the Structure Number.
///
/// #Return
///
/// Returns successfully print all.
fn main() {
    let sum = Numbers {
        num_first: 80,
        num_second: 20.0 as u32,
    };
    println!("Addition of two numbers is {:?}i ", sum.addition());
    println!("Subtraction of two numbers is: {:?}i", sum.subtract());
    println!("Multiplication of two numbers is: {:?}i", sum.multiplication());
    println!("Division of two numbers is: {:?}i", sum.division());
}
/// Numbers method calculate the two complex numbers.
///
/// #Arguments
///
/// complex of the Structure Number.
///
/// #Return
///
/// Returns result of two number.
impl Numbers {
    fn addition(&self) -> u32 {
        println!("First number and Second number is : {:?}, {:?}i", self.num_first, self.num_second);
        self.num_first + self.num_second
    }
    fn subtract(&self) -> u32 {
        self.num_first - self.num_second
    }
    fn multiplication(&self) -> u32 {
        self.num_first * self.num_second
    }
    fn division(&self) -> u32 {
        self.num_first / self.num_second
    }
}
