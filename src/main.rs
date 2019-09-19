use std::env;

fn main() {
    let key = "SERVICE_NAME";
    match env::var(key) {
        Ok(val) => println!("Hello, {}", val),
        _ => println!("Hello World!"),
    }
}
