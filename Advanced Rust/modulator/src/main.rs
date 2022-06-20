fn main() {
    let a: u32 = 4;
    let b: u32 = 1;

    println!("Sum of {} & {} = {}", a, b, modulator::sum(a, b));

    // Direct module in lib file
    modulator::my_mod::show_it();

    // Separate file from lib. Module must be first imported into lib to use it here;
    modulator::spare::spare::spare_it();

    // IN CASE YOU GET ERROR RUNNING THE PROJECT::
    // try `cargo run --bin modulator` or  `cargo run --bin token` 
    // as there has been an addition of another binary crate in the project
}
