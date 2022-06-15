fn main() {
    let str: String = "New String".to_string();
    let new_str: String = str; 

    // println!("{}", str); //Error - Borrow of moved value `str`. Such error is faced if an owner of a data is set as value of another data
    // to exit such state of error. One can use `clone()` method

    let solve_str: String = new_str.clone();

    println!("{}", solve_str); // no error faced now!
}
