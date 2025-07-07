# Image Downloader (Rust)

A command-line tool written in Rust to extract and download all images from a webpage or local HTML file.

## Features

- Extract image links from URL or local HTML file
- Multi-threaded parallel downloading
- Proxy support (HTTP/HTTPS/Global)
- Customizable User-Agent
- Automatic home directory path expansion (supports `~`)
- Progress display and error handling

## Installation

### From Source

1. Ensure you have [Rust toolchain](https://www.rust-lang.org/tools/install) installed
2. Clone repository:
   ```bash
   git clone https://github.com/your-repo/image-downloader.git
   cd image-downloader
   ```
3. Build and install:

    ```bash
    cargo install --path .
    ```

### Precompiled Binaries

Download prebuilt binaries from Releases page for your platform.

## Usage

```bash
image-downloader [OPTIONS]
```

### Basic Options

| Option         | Short | Description                       |
| :------------- | :---- | :-------------------------------- |
| --url          | -u    | Target webpage URL                |
| --html         | -H    | Local HTML file path              |
| --dir          | -d    | Output directory (default: ./img) |
| --user-agent   | -a    | Custom User-Agent                 |
| --proxy        | -p    | proxy address                     |
| --aria2c_proxy |       | aria2c apply proxy                |

## Examples

1. Download images from webpage:

    ```bash
    image-downloader -u https://example.com -d ~/images
    ```

2. Download from local HTML file:

    ```bash
    image-downloader -H page.html -d ./downloads
    ```

3. Use proxy :

    ```bash
    image-downloader -u https://example.com -p socks5://127.0.0.1:1080
    ```

4. Custom User-Agent:

    ```bash
    image-downloader -u https://example.com -a "My Custom User Agent"
    ```

## Dependencies

- aria2c (for actual downloading)

- Supported image formats: jpg, png, gif, webp (case insensitive)

## Contributing

Issues and Pull Requests are welcome
