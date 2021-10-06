fn main() {
    let years_arr: [i32; 11] = [2021, 2022, 2023, 2024, 2025, 2026, 2027, 2028, 2029, 2030, 2031];
    let mut count = 0;
    for i in 0..11 {
        if years_arr[i] % 4 == 0 {
            if years_arr[i] % 100 == 0 {
                if years_arr[i] % 400 == 0 {
                    //println!("{:?} is leap year", years_arr[i]);
                    count = count + 1;
                }
                else {
                    //println!("{:?} is not leap year", years_arr[i]);
                }
            }
            else {
                //println!("{:?} is leap year", years_arr[i]);
                count = count + 1;
            }
        }
        else {
            //println!("{:?} is not leap year", years_arr[i]);
        }
    }
    println!("Count of Leap Years is: {}", count);
}