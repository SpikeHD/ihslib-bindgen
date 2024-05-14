# ihslib-bindgen
Rust Bindgen for mariotaku/ihslib

# Requirements

* `protoc`
* `sdl2-dev`
* `libssl-dev`
* `libprotobuf-dev`
* `libmbedtls-dev`
* Maybe others that I forgot

# Usage

## Remotely

1. Add this under `[dependencies]`
   ```toml
   ihslib-bindgen = { git = "https://github.com/SpikeHD/ihslib-bindgen.git" }
   ```

## Locally

1. Clone the repo
   ```bash
   git clone https://github.com/SpikeHD/ihslib-bindgen.git
   ```
2. Run `cargo build`
3. To use locally in a project, add this under `[dependencies]`
   ```toml
   ihslib-bindgen = { path = "/path/to/ihslib-bindgen" }
   ```
