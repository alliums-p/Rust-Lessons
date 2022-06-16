fn double_str(s: &String) -> String {
    format!("{}{}", s, s)
}

fn main() {
    let str: String = "general".to_string();

    let str2: String = double_str(&str);

    println!("{}", str);
    println!("{}", str2);
}
