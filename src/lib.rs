use std::error::Error;

pub fn convert_value(input_value: &str, err_message: &str) -> Result<i32, Box<dyn Error>> {

    let value: i32 = match input_value.parse() {
        Ok(x) => x,
        _ => simple_error::bail!(err_message),
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_value() {
    
        let output = convert_value("3", "something went wrong").unwrap();    
        assert_eq!(output, 3);
    
    }

    #[test]
    fn test_convert_value_fail() {
    
        let result = convert_value("a", "something went wrong");
        assert!(result.is_err());    
    }
}

