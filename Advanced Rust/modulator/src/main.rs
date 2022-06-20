fn main() {
    let a: u32 = 4;
    let b: u32 = 1;

    println!("Sum of {} & {} = {}", a, b, modulator::sum(a, b));

    // Direct module in lib file
    modulator::my_mod::show_it();

    // Separate file from lib. Module must be first imported into lib to use it here;
    modulator::spare::spare::spare_it();
}
