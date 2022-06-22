fn main() {
    let new_arr: [u32; 5] = [10, 50, 44, 29, 2];

    println!("New array: {:?}", new_arr);
    println!("First element: {}", new_arr[0]);

    println!("Length of array is {}", new_arr.len());

    // slices array from index 1 to 3 (here, the index 3 of the array is also sliced)
    let sliced_arr = &new_arr[1..3];
    println!("Sliced array of new_arr: {:?} {}", sliced_arr, sliced_arr.len());

    string_call();
}

// About String in Rust
fn string_call() {

    // &str: string slice (copy/referencing the data)
    // "": String literal
    // to_string & String::from method converts string literal to a mutable string

    let name: &str = "Allimus";

    let occupation: String = "Blockchain Engineer".to_string();
    let address: String = String::from("Nepal");

    let address_slice = &address[1..4];

    println!("Name: {}", name);
    println!("Name length is {}", name.len());
    println!("Occupation: {}", occupation);
    println!("Sliced address: {}", address_slice);
    println!("Address: {}", address);

    println!("String concatination: {}", ["Hello", "world"].concat());
}