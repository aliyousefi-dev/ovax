# Building Ovax

This guide explains how to set up your environment and build **Ovax** from source. Because Ovax links directly to FFmpeg C-libraries for maximum performance, you must set up the following dependencies.

---

## Install Rust
You must have the Rust toolchain installed. If you do not have it, install it via [rustup.rs](https://rustup.rs/):
* **Windows:** Download and run `rustup-init.exe`.
* **Linux/Mac:** Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

## Install System Dependencies
Ovax uses `ffmpeg-next`, which requires **LLVM** (for generating Rust-to-C bindings) and the **FFmpeg Shared Libraries**.

### Install LLVM (Required)
LLVM is necessary for the Rust compiler to "read" the FFmpeg C-headers.
1. Download the latest LLVM LLVM-Release-Package from the [LLVM Releases Page](https://github.com/llvm/llvm-project/releases).
2. During installation, **you must check the box** that says:
   > **"Add LLVM to the system PATH for all users"** (or current user).

### Setup FFmpeg Libraries
You need the **Shared** version of FFmpeg, which includes the necessary `.lib` and `.h` files for the build.
1. Download the "Shared" release from [gyan.dev](https://www.gyan.dev/ffmpeg/builds/) (e.g., `ffmpeg-release-full-shared.7z`).
2. Extract the archive into the root of the **Ovax** project folder.
3. Rename the extracted folder to exactly: `ffmpeg-lib`.

> **Note:** Ovax uses the `ffmpeg-next` crate as the core engine to interact with MP4 files. This allows us to process video data with significantly lower overhead than calling an external FFmpeg process.


## Building the Project

1. Open the **Ovax** project folder in VS Code.
2. Ensure you have the **rust-analyzer** extension installed.
3. Press `Ctrl + Shift + P` and search for **"Tasks: Run Task" > "Build Rust"**.

Done !
