pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user(){
        assert_eq!("Hello User1", greet_user("User1"))
    }
}
