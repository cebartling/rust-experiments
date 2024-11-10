# Command Line Usage Guide for Image Converter

This guide demonstrates how to run the image converter utility through Cargo, with various examples and configurations.

## Initial Setup

First, ensure you're in the project directory and create a `.env` file with basic configuration:

```bash
# Create .env file
echo "IMAGE_QUALITY=90" > .env
echo "RUST_LOG=info" >> .env
```

## Basic Usage Examples

### 1. Basic Format Conversion (JPG to PNG)
Using long-form arguments:
```bash
cargo run -- --input samples/photo.jpg --output converted/photo.png --format png
```

Using short-form arguments:
```bash
cargo run -- -i samples/photo.jpg -o converted/photo.png -f png
```

### 2. Resize Image to Specific Width
Maintains aspect ratio:
```bash
cargo run -- -i samples/photo.jpg -o converted/photo_resized.webp -f webp -w 800
```

### 3. Resize to Exact Dimensions
```bash
cargo run -- -i samples/photo.png -o converted/thumbnail.jpg -f jpeg -w 300 -h 200
```

### 4. Convert to AVIF with Debug Logging
```bash
RUST_LOG=debug cargo run -- -i samples/photo.png -o converted/photo.avif -f avif
```

### 5. Convert with Custom Quality Setting
```bash
IMAGE_QUALITY=95 cargo run -- -i samples/photo.jpg -o converted/high_quality.jpg -f jpeg
```

## General Syntax

The basic command structure is:
```bash
cargo run -- [OPTIONS] --input <INPUT> --output <OUTPUT> --format <FORMAT>
```

## Development and Debugging

### Run with Debug Logging
```bash
RUST_LOG=debug cargo run -- [options]
```

### Run with Backtrace
```bash
RUST_BACKTRACE=1 cargo run -- [options]
```

## Production Usage

For production use, build and run the release version:

```bash
# Build release version
cargo build --release

# Run the compiled binary directly
./target/release/image_converter -i input.jpg -o output.png -f png
```

## Example Output

When running the converter, you'll see output similar to this:
```
[2024-11-10T12:00:00Z INFO  image_converter] Loading image from: samples/photo.jpg
[2024-11-10T12:00:00Z INFO  image_converter] Original dimensions: 1920x1080
[2024-11-10T12:00:00Z INFO  image_converter] New dimensions: 800x450
[2024-11-10T12:00:00Z INFO  image_converter] Converting to Jpeg format with quality 90
[2024-11-10T12:00:00Z INFO  image_converter] Successfully saved converted image to: converted/photo.jpg
[2024-11-10T12:00:00Z INFO  image_converter] Image conversion completed successfully
```

## Environment Variables

The following environment variables can be used to configure the converter:

- `IMAGE_QUALITY`: Set the quality of the output image (1-100)
- `RUST_LOG`: Set the logging level (error, warn, info, debug, trace)
- `RUST_BACKTRACE`: Enable backtrace for debugging (0, 1)

You can set these either in your `.env` file or directly in the command line.
