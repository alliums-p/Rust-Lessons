pub fn convert(_roman_numeral: &str) -> u32 {
    let mut sum = 0;
    for ch in _roman_numeral.chars() {
        println!("ch {}", ch);
        match ch {
            'I' => { sum += 1; } 
            'V' => { sum += 5; }
            'X' => { sum += 10; } 
            'L' => { sum += 50; }
            'C' => { sum += 100; } 
            'D' => { sum += 500; } 
            'M' => { sum += 1000; } 
            _ => {}
        }
    }

    return sum;
}


#[cfg(test)]
mod tests {

    use super::*;

    // setup

    #[test]
    fn it_should_return_one() {
        assert_eq!(convert("I"), 1);
    }

    #[test]
    fn it_should_return_two() {
        assert_eq!(convert("II"), 2);
    }

    #[test]
    fn it_should_handle_additive_digits() {
        assert_eq!(convert("VII"), 7);
        assert_eq!(convert("MMMDCCCLXXXVIII"), 3888);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_fails() {
        assert_eq!(true, false);
    }

    // tear-down
}
