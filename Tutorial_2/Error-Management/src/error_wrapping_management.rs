use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Read};


// Bir dosyayı okuyan basit bir fonksiyon
pub fn read_file(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    let mut content = String::new();
    file.read_to_string(&mut content)
        .with_context(|| format!("Failed to read content from file: {}", file_path))?;
    
    Ok(content)
}