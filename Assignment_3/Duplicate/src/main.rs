fn main() {
    let string: &str = "mayank singh rajput";
    let length = string.len();
    let mut count: i32;
    let mut char: Vec<char> = string.chars().collect();
    for element in 0..length {
     count = 1;
     for second_loop in element + 1..length {
         if char[element] == char[second_loop] && char[element] != ' ' {
            count = count + 1;
            char[second_loop] = '0';
            }
          }
       if count > 1 && char[element] != '0' {
            println!("{}", char[element]);
            }
      }
}