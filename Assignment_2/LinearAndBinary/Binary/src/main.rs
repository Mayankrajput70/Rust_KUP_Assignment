fn binary_search(item: &i32, array: &[i32]) -> i32 {
    let mut indexPosition = -1;

    if array.is_empty() {
        return indexPosition;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if &array[mid] > item {
            right = mid - 1;
        } else if &array[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if &array[left] == item {
        indexPosition = left as i32;
        return indexPosition;
    } else {
        return indexPosition;
    }
}
fn main() {
    let array: [i32; 9] = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let item = 80;
    let index = binary_search(&item, &array);

    if index >= 0 {
        println!("Element found at position: {}", index);
    }
    else {
        println!("Element not found");
    }
}