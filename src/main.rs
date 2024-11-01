fn binary_search(array: &[i8], low: i32, high: i32, x: i8, mut cc: u32) -> String {
   
    if high >= low {
        
        let mid = low + (high - low) / 2;

        if array[mid as usize] == x { return format!("Found at index {mid}, took {cc} times to finish") }

        if array[mid as usize] > x { 
            cc += 1;
            return binary_search(array, low, mid - 1, x, cc) 
            
        }
        
        cc += 1;
        return binary_search(array, mid + 1, high, x, cc)
        
    }

    format!("No matching index found, took {cc} times to finish")
}

fn main() {
    static ARRAY: [i8; 1000000] = [1; 1000000];
    static QUERY: i8 = 0;
    let cc: u32 = 0;
    println!("{}", binary_search(&ARRAY, 0, ARRAY.len() as i32 - 1, QUERY, cc));
}
