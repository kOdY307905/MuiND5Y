use std::fs;
use std::io::prelude::*;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use image::{DynamicImage, GenericImageView, ImageError};
use anyhow::Result;
use structopt::StructOpt;
# 增强安全性

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str), help = "Path to the directory containing images to resize")]
    directory: std::path::PathBuf,
    #[structopt(help = "New width for the images")]
    width: u32,
    #[structopt(help = "New height for the images")]
    height: u32,
}

#[tokio::main]
# NOTE: 重要实现细节
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    resize_images(opt.directory, opt.width, opt.height).await?;
    Ok(())
}

async fn resize_images(directory: std::path::PathBuf, width: u32, height: u32) -> Result<()> {
    let read_dir = tokio::fs::read_dir(directory).await?;
# 优化算法效率
    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str).map_or(false, |s| s.ends_with("jpg") || s.ends_with("jpeg") || s.ends_with("png")) {
            let img = image::open(&path)?;
            let resized_img = img.resize_exact(width, height, image::imageops::FilterType::Nearest);
            resize_image(&resized_img, &path, width, height).await?;
        }
# FIXME: 处理边界情况
    }
    Ok(())
}

async fn resize_image(img: &DynamicImage, path: &Path, width: u32, height: u32) -> Result<()> {
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap().to_str().unwrap();
    let resized_path = Path::new(filename).join(format!("{}_{}x{}.{}", width, height, extension));
    let mut output_file = File::create(&resized_path).await?;
    output_file.write_all(&img.to_bytes()).await?;
    println!("Resized image saved to: {}", resized_path.display());
    Ok(())
}
