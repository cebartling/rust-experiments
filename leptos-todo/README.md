# Leptos Todo

A simple Todo application built with Rust and Leptos, using Tailwind CSS for styling.

## Project Structure

- `Cargo.toml`: Rust project configuration file.
- `package.json`: Node.js project configuration file.
- `index.html`: HTML entry point for the application.
- `src/main.rs`: Main Rust source file containing the application logic.
- `tailwind.config.js`: Tailwind CSS configuration.
- `Trunk.toml`: Trunk configuration file.

## Prerequisites

- Rust and Cargo installed
- Trunk installed (`cargo install trunk`)
- Node.js and npm installed

### Trunk

In the Rust ecosystem, Trunk is a build and bundling tool primarily used for building and
deploying WebAssembly (Wasm)-based web applications. It is designed to simplify the process
of managing assets, dependencies, and build steps for projects built with frameworks like Yew, Seed, or any other
Rust-based Wasm framework.

#### Key Features of Trunk

1. Build Automation: Compiles Rust code to WebAssembly. Handles asset bundling, such as CSS, JavaScript, and static
   files.
2. Development Server: Includes a lightweight development server with live reloading, making it easier to iterate
   quickly.
3. HTML Management: Allows embedding the Wasm module into an index.html file with minimal effort. Provides a simple
   declarative way to define your HTML template.
4. Asset Pipelines: Supports asset processing pipelines for tasks like CSS bundling and optional preprocessing.
5. Customizable: Provides configuration through a Trunk.toml file or command-line arguments.
6. Integration with Cargo: Works seamlessly with Rust’s build system, Cargo.

Trunk removes much of the boilerplate and complexity involved in setting up a modern web application with Rust and
WebAssembly. It takes care of tasks like file watching, live reloading, and efficient bundling, allowing developers to
focus on writing code. It is particularly useful when developing client-side web applications in Rust.

## Setup

1. **Install Rust dependencies:**

   ```sh
   cargo build
   ```

2. **Install Node.js dependencies:**

   ```sh
   npm install
   ```

## Development

1. **Build and watch Tailwind CSS:**

   ```sh
   npm run watch:css
   ```

2. **Run the Rust application:**

   ```sh
   cargo run
   ```

## Build for Production

1. **Build Tailwind CSS:**

   ```sh
   npm run build:css
   ```

2. **Build the Rust application:**

   ```sh
   cargo build --release
   ```

## License

This project is licensed under the ISC License.
