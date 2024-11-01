fn binary_search(array: &[i32], low: i32, high: i32, x: i32) -> Option<i32> {
    if high >= low {
        let mid = low + (high - low) / 2;

        if array[mid as usize] == x { return Some(mid) }

        if array[mid as usize] > x { return binary_search(array, low, mid - 1, x) }

        return binary_search(array, low, mid + 1, x)
    }

    None
}

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    static QUERY: i32 = 0;
    let result = binary_search(&array, 0, array.len() as i32 - 1, QUERY);

    if result.is_none() {
        println!("No matches found");
    } else {
        println!("Element is present at index {:?}", result.unwrap());
    }

}
