fn main() {
    // Print the first n positive integers
    let n = 10;
    let mut i = 1;

    loop {
        println!("loop: {}", i); // statements inside the loop
        i = i + 1;               // Increment
        
        if i >  n{
            break;
        }
    }

    println!("in-between loops: {}",  i);
    i = 1;

    while i<= n {
        println!("while: {}", i);
        i = i+1;
    }

    println!("in-between loops: {}", i);

    for i in 1..=10 {
        println!("for: {}", i);
    }

    let cheat_code = [32, 49, 66, 75];

    for i in 0..cheat_code.len() {
        println!("cheat code: {}", cheat_code[i]);
    }

    for value in cheat_code {
        println!("cheat code by value: {}", value);
    }
}