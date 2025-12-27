# EuroScope Aviso Installer

A modern, cross-platform application to easily install "Aviso" sections into EuroScope LFXX sector files. Built with [Tauri](https://tauri.app/) (Rust) and [Vue 3](https://vuejs.org/).

## Features

- **Easy Installation**: Automatically copies Aviso section content into your target `.sct` files.
- **SCT & ESE Support**: Handles both `.sct` (Sector) and optional `.ese` (EuroScope Environment) files.
- **Verification**: Checks if Aviso packages are already installed to prevent duplicates.
- **Dual Modes**:
  - **Local File**: Install from local Aviso files.
  - **GitHub Integration**: Fetch and install Aviso packages directly from the [GitHub repository](https://github.com/arthurpar06/lfxx-aviso).
- **Modern UI**: Clean and responsive interface powered by Vue 3 and Tailwind CSS.

## Prerequisites

Before building the project, ensure you have the following installed:

- **[Rust](https://www.rust-lang.org/tools/install)** (latest stable)
- **[Node.js](https://nodejs.org/)** (LTS version recommended)

## Development Setup

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/arthurpar06/aviso-installer.git
    cd aviso-installer
    ```

2.  **Install dependencies**:
    ```bash
    npm install
    ```

3.  **Run the development server**:
    ```bash
    npm run tauri dev
    ```
    This will start the Vite frontend server and launch the Tauri application window.

## Build for Production

To build the application for your operating system:

```bash
npm run tauri build
```

The build artifacts will be located in `src-tauri/target/release/bundle`.

## Project Structure

- `src/`: Vue 3 frontend source code.
- `src-tauri/`: Rust backend source code (Tauri core).
- `src-tauri/src/lib.rs`: Main application logic and Tauri commands.
- `src-tauri/src/aviso/`: Aviso installation and parsing logic.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
