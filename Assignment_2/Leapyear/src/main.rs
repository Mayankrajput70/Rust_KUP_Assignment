fn main() {
    let years_arr: [i32; 11] = [2021, 2022, 2023, 2024, 2025, 2026, 2027, 2028, 2029, 2030, 2031];
    let mut count = 0;
    for element in 0..11 {
        if years_arr[element] % 4 == 0 {
            if years_arr[element] % 100 == 0 {
                if years_arr[element] % 400 == 0 {
                    count = count + 1;
                }
                else { }
            }
            else {
                count = count + 1;
            }
        }
        else { }
    }
    println!("Count of Leap Years is: {}", count);
}
