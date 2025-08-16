<div align="center">

# 🖼️ Hidden Pixel Vault


</div>

## Overview

Hidden Pixel Vault is a tool built with Rust that lets you hide secret messages within PNG images. It offers a secure and reliable method for hiding your data in plain sight, without changing how the image looks.

### Why hidden-pixel-vault?

This project enables seamless, reversible steganography workflows with a focus on security and data integrity. The core features include:

- **Secure Data Embedding**: Embed messages into PNG images with confidence, ensuring data remains hidden and intact.
- **PNG Parsing & Manipulation**: Leverages robust structures to parse, validate, and modify PNG files efficiently.
- **Command-line Interface**: Offers flexible commands for encoding, decoding, removing, and managing hidden data.
- **Atomic File Operations**: Ensures safe updates with backup, rollback, and recovery mechanisms.
- **Chunk Management & Validation**: Handles PNG chunks with integrity checks, supporting advanced image processing workflows.

## 📦 Installation

### Prerequisites

Before you begin, ensure you have the Rust programming language installed. If you don't have it, you can get it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### From Source

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/Gaurav-Sharmaa/hidden-pixel-vault.git
    ```

2.  **Navigate to the project directory:**

    ```bash
    cd hidden-pixel-vault
    ```

3.  **Build and run:**

    Below are the commands to run the application. The `--release` flag is recommended for better performance.

    ### Normal Operations
    These commands automatically create a backup of your original image.

    - **Print all chunks from an image:**
      ```bash
      cargo run print path/to/your/image.png
      ```

    - **Encode a secret message into an image:**
      *(Note: The chunk type must be 4 characters long. For a private chunk like `RuSt`, the third character must be uppercase.)*
      ```bash
      cargo run encode path/to/your/image.png RuSt "This is a secret message"
      ```

    - **Decode a secret message from an image:**
      ```bash
      cargo run decode path/to/your/image.png RuSt
      ```

    - **Remove a hidden message chunk from an image:**
      ```bash
      cargo run remove path/to/your/image.png RuSt
      ```

    ### Safety Commands
    Manage your image backups with these commands.

    - **Restore the original image from a backup:**
      ```bash
      cargo run restore path/to/your/image.png
      ```

    - **Check the backup status of an image:**
      ```bash
      cargo run status path/to/your/image.png
      ```

    - **Remove backup files for an image:**
      ```bash
      cargo run cleanup path/to/your/image.png
      ```

## 📚 Documentation

If you want to learn more about how a PNG is made and why it was created, you can read about it here:

- [PNG (Portable Network Graphics) Specification, Version 1.2](https://www.libpng.org/pub/png/spec/1.2/PNG-Introduction.html)

## 🌟 Show Your Support

If you find this project useful, please consider giving it a ⭐️ on [GitHub](https://github.com/yourusername/hidden-pixel-vault).

<div align="center">
Made with 🦀 by https://github.com/Gaurav-Sharmaa
</div>
