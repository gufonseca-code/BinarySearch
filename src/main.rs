fn binary_search(array: &[i32], low: i32, high: i32, x: i32, mut cc: u32) -> String {
   
    if high >= low {
        
        let mid = low + (high - low) / 2;

        if array[mid as usize] == x { return format!("Found at index {mid}, took {cc} times to finish") }

        if array[mid as usize] > x { 
            cc += 1;
            return binary_search(array, low, mid - 1, x, cc) 
            
        }
        
        cc += 1;
        return binary_search(array, low, mid + 1, x, cc)
        
    }

    format!("No matching index found, took {cc} times to finish")
}

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    static QUERY: i32 = 0;
    let cc: u32 = 0;
    println!("{}", binary_search(&array, 0, array.len() as i32 - 1, QUERY, cc));
}
