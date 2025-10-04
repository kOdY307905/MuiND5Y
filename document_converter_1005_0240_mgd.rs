use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio::util::AsyncWriteExt;
use std::path::Path;
use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::BufWriter;
use tokio::process::Command;

/// Convert a document from one format to another using a system command.
///
/// This function assumes that the system has the necessary command line tools installed
/// to perform the conversion (e.g., pandoc).
async fn convert_document(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Ensure that the input file exists
    if !input_path.exists() {
        return Err("Input file does not exist".into());
    }

    // Run the system command to convert the document
    let output = Command::new("pandoc")
        .arg(input_path)
        .arg("-o")
        .arg(output_path)
        .output()
        .await?;

    // Check if the command was successful
    if !output.status.success() {
        return Err("Conversion failed".into());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the input and output paths
    let input_path = Path::new("input.docx");
    let output_path = Path::new("output.pdf");

    // Convert the document
    convert_document(&input_path, &output_path).await?;

    println!("Document conversion successful!");

    Ok(())
}
