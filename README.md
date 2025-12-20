# Smart Road - Development Setup

This guide will help you set up the development environment for Smart Road on macOS, Linux, and Windows.

## Prerequisites

All platforms require:

- Rust (latest stable version) - Install from [rustup.rs](https://rustup.rs/)

## Platform-Specific Setup

### macOS

1. Install Homebrew (if not already installed):

   ```sh
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

2. Install SDL2:

   ```sh
   brew install sdl2
   ```

3. Build the project:

   ```sh
   cargo build
   ```

   **Note:** The project includes a `.cargo/config.toml` that automatically configures the library search paths for Homebrew-installed SDL2. No manual environment variable setup is needed!

### Linux (Ubuntu/Debian)

1. Install a C compiler (required for bundled SDL2):

   ```sh
   sudo apt-get update
   sudo apt-get install build-essential cmake
   ```

   For Fedora:

   ```sh
   sudo dnf install gcc gcc-c++ cmake
   ```

   For Arch:

   ```sh
   sudo pacman -S base-devel cmake
   ```

2. Build the project:

   ```sh
   cargo build
   ```

   **Note:** The project uses the "bundled" feature on Linux, which compiles SDL2 from source automatically. This ensures compatibility across all Linux distributions.

### Windows

#### Option A: Using Visual Studio (MSVC - Recommended)

1. Install Visual Studio 2019 or later with "Desktop development with C++" workload

   - Download from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/)
   - Make sure to check "Desktop development with C++" during installation
   - This includes CMake which is required for bundled SDL2

2. Build the project:

   ```cmd
   cargo build
   ```

   The "bundled" feature will automatically compile SDL2 for you.

#### Option B: Using MinGW

1. Install MinGW-w64 via [MSYS2](https://www.msys2.org/)

2. In MSYS2 terminal, install the required packages:

   ```sh
   pacman -S mingw-w64-x86_64-toolchain mingw-w64-x86_64-cmake
   ```

3. Build the project:
   ```cmd
   cargo build
   ```

## Running the Project

Once the setup is complete, run the project with:

```sh
cargo run
```

You should see a yellow window appear. Close it with the X button or press ESC.

## How It Works

The project uses **platform-specific dependencies** in `Cargo.toml`:

- **macOS**: Uses Homebrew-installed SDL2 (system library)
  - Faster builds (no compilation needed)
  - The `.cargo/config.toml` file automatically configures library paths
- **Linux & Windows**: Uses the "bundled" feature
  - Compiles SDL2 from source during the first build
  - Ensures compatibility across different distributions
  - Requires a C compiler (gcc, clang, or MSVC)

## Troubleshooting

### macOS: "library 'SDL2' not found" error

Make sure SDL2 is installed via Homebrew:

```sh
brew install sdl2
```

The `.cargo/config.toml` should handle the library paths automatically.

### Linux/Windows: CMake errors

Make sure you have CMake installed:

- **Linux**: `sudo apt-get install cmake` (Ubuntu/Debian)
- **Windows**: Included with Visual Studio C++ tools or install via MSYS2

### Linux/Windows: Compiler errors

Make sure you have a C compiler:

- **Linux**: `sudo apt-get install build-essential`
- **Windows**: Install Visual Studio with C++ tools or MinGW via MSYS2

### All platforms: Rust version issues

Ensure you're using the latest stable Rust:

```sh
rustup update stable
```

## Project Structure

```
smart-road/
├── .cargo/
│   └── config.toml       # Platform-specific build configuration
├── src/
│   ├── main.rs           # Entry point
│   ├── render/           # Rendering module
│   │   ├── mod.rs        # Module declaration
│   │   ├── sdl2_manager.rs # SDL2 wrapper
│   │   └── world.rs      # World rendering
│   ├── sim/              # Simulation logic
│   └── spawn/            # Entity spawning
├── Cargo.toml            # Project dependencies (platform-specific)
└── SETUP.md              # This file
```

## Technical Details

### Why Different Approaches for Different Platforms?

- **macOS**: The "bundled" feature has issues with CMake detection on macOS, even when CMake is installed via Homebrew. Using the system-installed SDL2 from Homebrew is more reliable.

- **Linux & Windows**: The "bundled" feature works well and ensures that everyone has the same SDL2 version, regardless of their system's package manager or installed libraries.

### About .cargo/config.toml

This file sets the library search paths for macOS:

- `/opt/homebrew/lib` for Apple Silicon Macs
- `/usr/local/lib` for Intel Macs

This eliminates the need to manually set `LIBRARY_PATH` environment variables.

## Need Help?

If you encounter any issues:

1. Ensure you have the latest stable Rust installed (`rustup update stable`)
2. Verify platform-specific dependencies are installed (SDL2 on macOS, C compiler on Linux/Windows)
3. Try `cargo clean` and rebuild

For further assistance, create an issue in the project repository.
