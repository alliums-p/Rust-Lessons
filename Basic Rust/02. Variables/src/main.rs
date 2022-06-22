fn main() {
    // Declaring variables using `let` keyword
    let x = 2;
    let y = 5;
    println!("Hello, world! {} + {} = {}", x, y, x+y);

    // Declaring `type` of the variable
    let a: u8 = 255;
    let b: u16 = 65535;
    let c: u32 = 4294967295;
    let d: u64 = 18446744073709551615;
    
    println!("{}, {}, {}, {}", a, b, c, d);

    // Default variable is set as Immutable.
    // Must declare `mut` to make the variable value change
    let mut changable: u64 = 123456;
    changable = changable + 2;
    println!("{}", changable);
}