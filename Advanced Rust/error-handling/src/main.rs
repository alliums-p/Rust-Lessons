fn erroneous_call (){
    // non-recoverable error handling
    panic!("Error detected here!");
}

fn main() {
    // erroneous_call();

    // Backtrace error using cmd `RUST_BACKTRACE=1 cargo run`
}