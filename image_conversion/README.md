# Image Converter

A robust command-line utility for converting and resizing images between different formats, written in Rust.

## Features

- **Multiple Format Support**:
    - JPEG/JPG
    - PNG
    - WebP
    - AVIF
- **Image Manipulation**:
    - Resize images while maintaining aspect ratio
    - Set custom dimensions (width and/or height)
    - Configurable output quality
- **Flexible Configuration**:
    - Environment variable support via `.env` files
    - Command-line argument interface
    - Comprehensive logging system

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/image-converter
cd image-converter
```

2. Build the project:

```bash
cargo build --release
```

The compiled binary will be available in `target/release/image_converter`.

## Configuration

### Environment Variables

Create a `.env` file in the project root with the following options:

```env
# Image quality (1-100)
IMAGE_QUALITY=90

# Log level (error, warn, info, debug, trace)
RUST_LOG=info
```

### Command Line Arguments

The utility supports the following command-line arguments:

| Argument    | Short | Long       | Description                           | Required |
|-------------|-------|------------|---------------------------------------|----------|
| Input path  | `-i`  | `--input`  | Path to input image                   | Yes      |
| Output path | `-o`  | `--output` | Path for converted image              | Yes      |
| Format      | `-f`  | `--format` | Output format (jpeg, png, webp, avif) | Yes      |
| Width       | `-w`  | `--width`  | New width in pixels                   | No       |
| Height      | `-H`  | `--height` | New height in pixels                  | No       |
| Help        | `-h`  | `--help`   | Help                                  | No       |

## Usage Examples

- [Command Line Interface Usage Guide](./cli-guide.md)

1. Basic format conversion:

```bash
image_converter -i input.jpg -o output.png -f png
```

2. Resize image to specific width (maintaining aspect ratio):

```bash
image_converter -i input.jpg -o output.webp -f webp -w 800
```

3. Resize image to exact dimensions:

```bash
image_converter -i input.png -o output.avif -f avif -w 800 -H 600
```

## Logging

The utility uses the `env_logger` crate for logging. Set the `RUST_LOG` environment variable to control log levels:

- `error`: Only errors
- `warn`: Warnings and errors
- `info`: General information, warnings, and errors (recommended)
- `debug`: Detailed information for debugging
- `trace`: Very verbose logging

Example log output:

```
[2024-11-11T00:13:31Z INFO  image_conversion] Loading image from: ./images/eddie-alien.jpg
[2024-11-11T00:13:31Z INFO  image_conversion] Original dimensions: 1024x1024
[2024-11-11T00:13:31Z INFO  image_conversion] New dimensions: 1024x1024
[2024-11-11T00:13:31Z INFO  image_conversion] Converting to WebP format with quality 90
[2024-11-11T00:13:32Z INFO  image_conversion] Successfully saved converted image to: ./images/eddie-alien.webp
[2024-11-11T00:13:32Z INFO  image_conversion] Image conversion completed successfully
```

## Error Handling

The utility provides detailed error messages for common issues:

- Invalid input file path
- Unsupported image formats
- Permission issues
- Invalid dimensions
- Conversion failures

## Performance Considerations

- The utility uses the Lanczos3 algorithm for image resizing, providing high-quality results
- Large images may require significant memory
- AVIF encoding can be CPU-intensive

## Technical Details

- [UML sequence diagrams](./sequence-diagram-doc.md)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [image-rs](https://github.com/image-rs/image) - Rust image processing foundation
- [clap-rs](https://github.com/clap-rs/clap) - Command line argument parsing
- [env_logger](https://github.com/env-logger-rs/env_logger) - Logging implementation

## Support

For issues, questions, or contributions, please open an issue in the GitHub repository.
