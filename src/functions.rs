
pub fn greet(name: String) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn greet_greets_correctly() {
        let greeting = greet("Peter".to_string());
        assert_eq!(greeting, "Hello Peter!")
    }
}

