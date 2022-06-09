fn main() {
    // Arithmetic Operations 
    // (types must match to perform calculation)
    // Cannot calculate type i64 + i16
    let first: i64 = 20;
    let second: i64 = 10;

    println!(
        "{}, {}, {}, {}, {}",
        first+second,
        first-second,
        first*second,
        first/second,
        first%second,
    )
}
