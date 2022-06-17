fn main() {

    // Rust’s arrays have a fixed number of elements.
    // The vector type behaves like a dynamic array, where you can add and remove elements.
    // Vector is a generic type. To create a vector, we need to specify the type of its elements. 
    // The elements of a vector have to have the same type.

    // Vec<type> is a vector with fixed type
    // The vec! macro creates a vector with given elements.
    // Notice the &. Here we borrow the exponents vector.

    // `*` deferencing the referenced data where data type of exp ends up becoming &u32

    let exponents:  Vec<u32> = vec![17, 19, 24, 8];

    for exp in &exponents {
        println!("2.pow({}) = {}", exp, 2_i32.pow(*exp));
    }

    // Vec::new() can be used to initialize new vectors

    // Changing vector values
    print!("\n");
    let mut values:  Vec<u32> = vec![17, 19, 24, 8];

    for exp in &mut values {
        let value = 2_u32.pow(*exp);
        println!("2.pow({}) = {}", exp, value);
        *exp = value;
    }
    println!("\n{:?}\n", values);


    // Transforming vectors

    let mut data: Vec<u32> = Vec::new();

    data.push(1);
    data.push(2);
    data.push(3);

    println!("{:?}\n", data);

    // Error overbound of array elements:
    let mut i = 0;
    // while i < 10 {
    //     println!("{}", data[i]);
    //     i += 1;
    // }

    // Safe reading without out of bound: 
    while i < 10 {
        match data.get(i) {
            Some(correct_value) => 
                println!(
                    "i: {} - data.get(i): {}", i, correct_value
                ),
            None => 
                println!("i: {}, data.get(i) does not exist.", i)
        }
        i += 1;
    }

    // Removing elements from Vector
    let rm_element = data.remove(0);  // 0 is the index of the vector
    println!("\nRemoved element: {:?}", rm_element);
    println!("new data: {:?}", data);

    let rm_element2 = data.pop();  // 0 is the index of the vector
    println!("Removed element: {:?}", rm_element2);
    println!("new data: {:?}", data);

    // Using pop in safe way
    let pop_value: u32;
    match data.pop() {
        Some(value) => pop_value = value,
        None => pop_value = 0
    }

    println!("{}", pop_value);
}
