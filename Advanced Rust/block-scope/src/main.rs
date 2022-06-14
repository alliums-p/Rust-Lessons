fn change_val(mut x: i32) -> i32 {
    x += 1;
    return x;
}

fn main() {
    // {
    //     let num = 10;
    //     println!("Hello, world!, {}", num);
    // }

    // // The following value of the num would not be accessible here due to scope restriction
    // println!("This is num: {}", num);

    // =============================================

    let val : i32 = 0;
    change_val(val);

    println!("{}", val); // This will return 0 instead of 1 due to the block scope limitation. 
    // Value declared in one scope cannot be changed from another scope.
    // Instead, val = change_val(val) can be used in the main() function to change the value if the val variable is set to mutable 
}
