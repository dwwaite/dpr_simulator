use polars::prelude::*;
use simple_error::bail;
use std::error::Error;
use std::fs::File;

pub fn write_to_parquet(
    output_path: &str,
    mut file_content: DataFrame,
) -> Result<String, Box<dyn Error>> {
    // Write the DataFrame content into the compressed parquet format.

    let target_file = File::create(output_path)?;

    let result = match ParquetWriter::new(target_file).finish(&mut file_content) {
        Ok(_) => format!("Results written to file '{}'!", output_path),
        _ => bail!("Unable to write to output file path '{}'!", output_path),
    };

    Ok(result)
}

/*
#[cfg(test)]
mod tests {
    use super::*;

}
*/
