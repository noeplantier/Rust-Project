fn check_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        Err("Password is too short".to_string())
    } else {
        Ok(())
    }
}



fn main() {
    println!("Hello, world!");
}
