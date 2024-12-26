# Pomodoro Application in Rust and Leptos

A simple pomodoro web application built with Leptos and Rust, demonstrating basic state management and component
structure. The original idea came from a [ReactPractice](https://reactpractice.dev/exercise/build-a-pomodoro-app/)
exercise.

## Requirements

- [X] The user can start a 25-minute pomodoro, and the timer will go off once 25 minutes has elapsed.
- [ ] A 5-minute break timer is started after the 25-minute pomodoro completes.
- [ ] After four pomodoros, there is a longer, 15-30 minute break.
- [X] The user can start a new pomodoro at any time.
- [X] The user can pause the timer.
- [X] The user can stop the timer.
- [X] The user can reset the timer to 25 minutes.
- [ ] The user can customize the length of each timer.
- [ ] The user can hear a sound play when the timer goes off.
- [ ] The user can be notified visually with a confetti burst when the timer goes off.

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
- Basic styling with Tailwind CSS
- Timer logic and state management
- Component lifecycle methods
- Component communication
- Component state management
- Component event handling

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

