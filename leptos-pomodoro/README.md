# Pomodoro Application in Rust and Leptos

A simple pomodoro web application built with Leptos and Rust, demonstrating basic state management and component
structure.

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version). Current using `1.83.0 (90b35a623 2024-11-26)`
- WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install trunk`

## Features

- Simple pomodoro component demonstrating state management
- Client-side rendering (CSR) setup
- Console logging and error handling
- WebAssembly configuration

## Getting Started

1. Install dependencies:

   ```bash
   cargo build
   ```

2. Start the development server:

   ```bash
   trunk serve
   ```

The application will be available at `http://127.0.0.1:8080`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

