fn main() {
    // Using in-built objects to get MIN value of declared `type`
    let a_min: u8 = std::u8::MIN;
    let b_min: u16 = std::u16::MIN;
    let c_min: u32 = std::u32::MIN;
    let d_min: u64 = std::u64::MIN;
    let e_min: usize = std::usize::MIN;

    // Using in-built objects to get MAX value of declared `type`
    let a_max: u8 = std::u8::MAX;
    let b_max: u16 = std::u16::MAX;
    let c_max: u32 = std::u32::MAX;
    let d_max: u64 = std::u64::MAX;
    let e_max: usize = std::usize::MAX;

    println!("\nPrinting MIN & MAX values of unsigned integer: ");
    println!("Unsigned MIN: {}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("Unsigned MAX: {}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    // Declaring Signed Integers
    let a_min: i8 = std::i8::MIN;
    let b_min: i16 = std::i16::MIN;
    let c_min: i32 = std::i32::MIN;
    let d_min: i64 = std::i64::MIN;
    let e_min: isize = std::isize::MIN;

    let a_max: i8 = std::i8::MAX;
    let b_max: i16 = std::i16::MAX;
    let c_max: i32 = std::i32::MAX;
    let d_max: i64 = std::i64::MAX;
    let e_max: isize = std::isize::MAX;

    println!("\nPrinting MIN & MAX values of signed integer: ");
    println!("Signed MIN: {}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("Signed MAX: {}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    // Using floating points
    let a_min: f32 = std::f32::MIN;
    let b_min: f64 = std::f64::MIN;

    let a_max: f32 = std::f32::MAX;
    let b_max: f64 = std::f64::MAX;

    println!("\nPrinting floating values: ");
    println!("Min Floating points: {}, {}", a_min, b_min);
    println!("Max Floating points: {}, {}", a_max, b_max);

    let a: f64 = 1.0; 
    let b: f64 = 0.1;
    let c: f64 = 0.2;

    println!("\n{}, {}", a, b + c);

    // Usage of `Char` type
    let ch1: char = 'X';
    let ch2 = '\u{2603}';

    println!("{}, {}", ch1, ch2);

    // `Boolean` type
    let on: bool = true;
    let off = false;

    println!("\n {}, {}, {}", on, off, !on);
    println!("{}", on);

    // Declaring `const` variables;
    const PI: f64 = 3.1415;

    println!("{}", PI);
}
