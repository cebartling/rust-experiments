use std::path::PathBuf;
use std::env;
use clap::{Parser};
use image::ImageFormat;
use log::{info, error};
use anyhow::{Result, Context};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input image path
    #[arg(short, long)]
    input: PathBuf,

    /// Output image path
    #[arg(short, long)]
    output: PathBuf,

    /// Output format (jpeg, png, webp, avif)
    #[arg(short, long)]
    format: String,

    /// New width (optional)
    #[arg(short, long)]
    width: Option<u32>,

    /// New height (optional)
    #[arg(short, long)]
    height: Option<u32>,
}


fn init() {
    dotenv::dotenv().ok();
    env_logger::init();
}

fn get_format(format_str: &str) -> Option<ImageFormat> {
    match format_str.to_lowercase().as_str() {
        "jpeg" | "jpg" => Some(ImageFormat::Jpeg),
        "png" => Some(ImageFormat::Png),
        "webp" => Some(ImageFormat::WebP),
        "avif" => Some(ImageFormat::Avif),
        _ => None,
    }
}

fn resize_image(img: &image::DynamicImage, width: Option<u32>, height: Option<u32>) -> image::DynamicImage {
    match (width, height) {
        (Some(w), Some(h)) => img.resize_exact(w, h, image::imageops::FilterType::Lanczos3),
        (Some(w), None) => {
            let ratio = w as f32 / img.width() as f32;
            let new_height = (img.height() as f32 * ratio) as u32;
            img.resize(w, new_height, image::imageops::FilterType::Lanczos3)
        }
        (None, Some(h)) => {
            let ratio = h as f32 / img.height() as f32;
            let new_width = (img.width() as f32 * ratio) as u32;
            img.resize(new_width, h, image::imageops::FilterType::Lanczos3)
        }
        (None, None) => img.clone(),
    }
}

fn convert_image(args: &Args) -> Result<()> {
    // Load quality setting from environment variable or use default
    let quality: u8 = env::var("IMAGE_QUALITY")
        .unwrap_or_else(|_| "90".to_string())
        .parse()
        .unwrap_or(90);

    info!("Loading image from: {}", args.input.display());
    let img = image::open(&args.input)
        .with_context(|| format!("Failed to open input image: {}", args.input.display()))?;

    info!("Original dimensions: {}x{}", img.width(), img.height());

    // Resize image if dimensions provided
    let img = resize_image(&img, args.width, args.height);
    info!("New dimensions: {}x{}", img.width(), img.height());

    // Get output format
    let format = get_format(&args.format)
        .with_context(|| format!("Unsupported format: {}", args.format))?;

    info!("Converting to {:?} format with quality {}", format, quality);

    // Save with format-specific options
    match format {
        ImageFormat::Jpeg => {
            img.save_with_format(&args.output, format)?;
        }
        ImageFormat::Png => {
            img.save_with_format(&args.output, format)?;
        }
        ImageFormat::WebP => {
            img.save_with_format(&args.output, format)?;
        }
        ImageFormat::Avif => {
            img.save_with_format(&args.output, format)?;
        }
        _ => {
            error!("Unsupported format");
            return Err(anyhow::anyhow!("Unsupported format"));
        }
    }

    info!("Successfully saved converted image to: {}", args.output.display());
    Ok(())
}

fn main() -> Result<()> {
    init();
    let args = Args::parse();

    match convert_image(&args) {
        Ok(_) => {
            info!("Image conversion completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Error converting image: {}", e);
            Err(e)
        }
    }
}


