use std::path::Path;
use tokio::fs;
use tokio::io::AsyncReadExt;
use image::{self, ImageOutputFormat, open};
use anyhow::Result;

/// Adjusts the dimensions of a batch of images.
///
/// # Arguments
///
/// * `input_dir` - A string slice that represents the path to the directory containing the images to be resized.
/// * `output_dir` - A string slice that represents the path to the directory where the resized images will be saved.
/// * `new_width` - The new width for the resized images.
/// * `new_height` - The new height for the resized images.
async fn resize_images_batch(input_dir: &str, output_dir: &str, new_width: u32, new_height: u32) -> Result<()> {
    // Read all file entries in the input directory.
    let files = fs::read_dir(input_dir).await?;

    // Iterate over the files.
    for file in files {
        let file = file?;
        let path = file.path();

        // Check if the path is a file and has a supported image extension.
        if path.is_file() && path.extension().map_or(false, |e| e == "jpg" || e == "png" || e == "jpeg" || e == "gif") {
            // Read the image file asynchronously.
            let mut file = fs::File::open(&path).await?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).await?;

            // Load the image from the buffer.
            let img = image::load_from_memory(&buffer)?;

            // Resize the image.
            let (width, height) = img.dimensions();
            let resized_img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

            // Create the output path for the resized image.
            let output_path = output_dir.to_string() + "/" + path.file_name().unwrap().to_str().unwrap();

            // Save the resized image to the output directory.
            resized_img.write_to_file(&output_path, ImageOutputFormat::Jpeg(80))?;
            println!("Resized image saved to: {}", output_path);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Specify the input and output directories, along with the new dimensions.
    let input_dir = "./input_images";
    let output_dir = "./resized_images";
    let new_width = 800;
    let new_height = 600;

    // Ensure the output directory exists.
    fs::create_dir_all(output_dir).await?;

    // Resize the images in the batch.
    resize_images_batch(input_dir, output_dir, new_width, new_height).await
}
