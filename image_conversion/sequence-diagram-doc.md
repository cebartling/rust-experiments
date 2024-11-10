# Image Converter - Sequence Diagram Documentation

This document illustrates the sequence flow of the image converter utility using UML sequence diagrams.

## Main Program Flow

```mermaid
sequenceDiagram
    participant User
    participant Main
    participant Args Parser
    participant Environment
    participant Logger
    participant Converter
    participant Image Library
    participant FileSystem

    User->>Main: Run program with arguments
    activate Main
    
    Main->>Environment: Load .env file
    Environment-->>Main: Environment variables
    
    Main->>Logger: Initialize logger
    Logger-->>Main: Logger ready
    
    Main->>Args Parser: Parse command line arguments
    Args Parser-->>Main: Parsed arguments
    
    Main->>Converter: Convert image
    activate Converter
    
    Converter->>FileSystem: Read input file
    FileSystem-->>Converter: Raw image data
    
    Converter->>Image Library: Open image
    Image Library-->>Converter: Image object
    
    alt Resizing Requested
        Converter->>Image Library: Resize image
        Image Library-->>Converter: Resized image
    end
    
    Converter->>Image Library: Convert format
    Image Library-->>Converter: Converted image
    
    Converter->>FileSystem: Write output file
    FileSystem-->>Converter: Write confirmation
    
    Converter-->>Main: Conversion result
    deactivate Converter
    
    Main-->>User: Success/Error message
    deactivate Main
```

## Error Handling Flow

```mermaid
sequenceDiagram
    participant User
    participant Main
    participant Converter
    participant Logger
    participant Error Handler
    
    User->>Main: Run with invalid arguments
    activate Main
    
    Main->>Converter: Convert image
    activate Converter
    
    alt Invalid Input Path
        Converter->>Error Handler: File not found error
        Error Handler->>Logger: Log error
        Error Handler-->>Main: Error result
    else Invalid Format
        Converter->>Error Handler: Format error
        Error Handler->>Logger: Log error
        Error Handler-->>Main: Error result
    else Conversion Error
        Converter->>Error Handler: Processing error
        Error Handler->>Logger: Log error
        Error Handler-->>Main: Error result
    end
    
    deactivate Converter
    Main-->>User: Error message
    deactivate Main
```

## Environment Configuration Flow

```mermaid
sequenceDiagram
    participant Program
    participant Dotenv
    participant Environment
    participant Logger
    
    Program->>Dotenv: Load .env file
    
    alt File Exists
        Dotenv->>Environment: Set IMAGE_QUALITY
        Dotenv->>Environment: Set RUST_LOG
        Environment-->>Program: Variables loaded
    else File Not Found
        Dotenv->>Environment: Use defaults
        Environment-->>Program: Default values
    end
    
    Program->>Logger: Configure with RUST_LOG
    Logger-->>Program: Logger configured
```

## Image Processing Flow

```mermaid
sequenceDiagram
    participant Converter
    participant Image Library
    participant Format Handler
    
    Converter->>Image Library: Load image
    Image Library-->>Converter: Image object
    
    alt Resize Requested
        Converter->>Image Library: Calculate dimensions
        Image Library-->>Converter: New dimensions
        Converter->>Image Library: Resize image
        Image Library-->>Converter: Resized image
    end
    
    Converter->>Format Handler: Get output format
    Format Handler-->>Converter: Format configuration
    
    alt JPEG/PNG
        Converter->>Image Library: Basic conversion
    else WebP
        Converter->>Image Library: WebP conversion
    else AVIF
        Converter->>Image Library: AVIF conversion
    end
    
    Image Library-->>Converter: Converted image
```

These sequence diagrams illustrate:
1. The main program flow from user input to output
2. Error handling pathways
3. Environment configuration process
4. Detailed image processing steps

The diagrams show the interaction between different components of the system and how data flows through the application during the image conversion process.
