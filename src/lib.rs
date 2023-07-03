use std::error::Error;
use std::fs::File;
use polars::prelude::*;
use simple_error::bail;

pub fn convert_value(input_value: &str, err_message: &str) -> Result<i32, Box<dyn Error>> {

    let value: i32 = match input_value.parse() {
        Ok(x) => x,
        _ => simple_error::bail!(err_message),
    };
    Ok(value)
}

pub fn write_to_parquet(output_path: &str, mut file_content: DataFrame) -> Result<String, Box<dyn Error>> {

    let target_file = File::create(output_path)?;

    let result = match ParquetWriter::new(target_file).finish(&mut file_content) {
        Ok(_) => format!("Results written to file '{}'!", output_path),
        _ => bail!("")
    };

    Ok(result)
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

