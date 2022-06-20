pub mod spare;

pub fn sum(a: u32, b: u32) -> u32 {
    a + b
}

pub mod my_mod {
    pub fn show_it() {
        println!("This is from my_mod module!");
    }
}