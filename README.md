# The `mime_more` Crate

`mime_more` is an extension to the `mime` crate, offering additional functionality for working with MIME types. ⚡️:crab:

## Features

- Built on `mime`, inheriting all its capabilities.
- **Extension-Based MIME Guessing**:
  - **`extension` Feature**: Uses the `mime_guess` crate to guess the MIME type based on file extensions.
  - **`extension-light` Feature**: A lighter version of the `extension` feature, supporting only common extensions. About 8x faster than `extension`!
- **Magic Byte-Based MIME Guessing** (`magic` Feature): Uses the `infer` crate to determine MIME type based on file magic bytes.
- **Texture Validation** (`texture` Feature): Provides high-performance methods for texture validation.
- **Data URL Generation** (`dataurl` Feature): Generates and parses data URLs with relatively shorter lengths and less time. It encodes `Source Han Sans` font in about 1.5 ms and decodes it in about 6 ms!

## Usage

Add `mime_more` to your `Cargo.toml`:

```toml
[dependencies]
mime_more = "0.1.4"
```

Example usage:

### Directly Parsing MIME Types

```rust
use mime_more::Mime;

fn main() {
    let mime = Mime::from_extension("html").unwrap();
    println!("{}", mime);
}
```

### Guessing MIME Types via File Path

> Needs `extension` (function is named `from_path`) or `extension-light` (function is named `from_path_light`) feature.

```rust
use mime_more::Mime;

fn main() {
    let mime = Mime::from_path_light("index.html").unwrap().to_string();
    assert_eq!(mime, "text/html".to_string());
}
```

### Inferring MIME Types via File Magic Bytes

> Needs `magic` feature.

```rust
use mime_more::Mime;

fn main() {
    let data = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0];
    let mime = Mime::from_magic(data).unwrap().to_string();
    assert_eq!(mime, "image/png".to_string());
    // `89 50 4E 47 0D 0A 1A 0A` is the magic byte sequence for PNG.
}
```

### Validating Textures

> Needs `texture` feature.

```rust
use mime_more::Mime;

fn main() {
    let data = Mime::from_str("image/png").unwrap();
    assert!(data.is_texture());
    
    let data = Mime::from_str("application/json").unwrap();
    assert!(!data.is_texture());
}
```

### Generating Data URLs

> Needs `dataurl` feature.

```rust
use mime_more::{Mime, dataurl::Dataurl};

fn main() {
    let dataurl = Dataurl::from_path("path/to/file.jpg").unwrap();
    println!("{}", dataurl); // We implemented Display for Dataurl.
}
```

## License

This project is licensed under the MIT license.

## Authors

The project is currently developed by [Ethan Goh](https://github.com/7086cmd).
