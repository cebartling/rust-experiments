# Leptos Hello World App with Tailwind CSS

A hello world application built with Leptos and styled using Tailwind CSS, served with Trunk.

## Prerequisites

- Rust and Cargo
- Node.js and npm
- Trunk (`cargo install trunk`)

## Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd leptos-hello-world
```

2. Install dependencies:
```bash
# Install npm dependencies
npm install

# Install Rust dependencies
cargo build
```

## Project Structure

```
leptos-hello-world/
├── src/
│   └── main.rs
├── styles/
│   ├── input.css
│   └── output.css
├── Cargo.toml
├── index.html
├── package.json
├── package-lock.json
├── tailwind.config.js
└── Trunk.toml
```

## Development

Run these commands in separate terminals:

```bash
# Terminal 1: Watch Tailwind CSS changes
npm run watch:css

# Terminal 2: Run Trunk development server
trunk serve
```

The app will be available at [`http://localhost:8080`](http://localhost:8080).

## Production Build

```bash
# Build CSS
npm run build:css

# Build application
trunk build
```

The built files will be in the `dist` directory.

## Key Files

- `src/main.rs`: Main application code
- `styles/input.css`: Tailwind directives
- `index.html`: HTML template
- `tailwind.config.js`: Tailwind configuration
- `Trunk.toml`: Trunk configuration

## NPM Scripts

- `watch:css`: Watch and compile Tailwind CSS in development
- `build:css`: Build and minify CSS for production

## Troubleshooting

- If Tailwind classes aren't applying, check if `output.css` is being generated
- Verify the CSS path in `index.html` matches your output location
- Ensure `tailwind.config.js` includes the correct file patterns in its content array


