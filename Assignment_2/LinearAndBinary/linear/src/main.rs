fn main() {
    let mut array: [i32; 8] = [10, 20, 40, 30, 60, 50, 90, 80];

    println!("array size is :{}", array.len());

    for item in array.iter().enumerate() {
        let (element, num): (usize, &i32) = item;
        println!("array[{}] = {}", element, num);
    }
}
