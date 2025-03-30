pub fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), Box<dyn std::error::Error>> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
