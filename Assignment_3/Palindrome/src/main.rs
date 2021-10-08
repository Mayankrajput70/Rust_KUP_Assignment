fn main() {
    let string: &str = "madam";
    let length = string.len();
    for element in 0..length {
        if &string.as_bytes()[element] != &string.as_bytes()[length - element - 1] {
            println!("String is Not Palindrome");
            break;
           }
        else {
            println!("String is Palindrome");
            break;
        }
    }
}
