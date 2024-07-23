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

## Benchmark result

```
     Running benches/function_benchmark.rs (target/release/deps/function_benchmark-ce680cff84e61f76)
Gnuplot not found, using plotters backend
ModuleType::from_known_str
                        time:   [25.839 ns 25.892 ns 25.945 ns]
                        change: [-3.5366% -3.3160% -3.0938%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

ModuleType::from_ext    time:   [288.99 ns 289.51 ns 290.04 ns]
                        change: [-2.5172% -2.2286% -1.9559%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

ModuleType::from_ext_light
                        time:   [34.723 ns 34.816 ns 34.918 ns]
                        change: [-1.9821% -1.7247% -1.4603%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

ModuleType::from_content
                        time:   [58.444 ns 58.535 ns 58.623 ns]
                        change: [-0.9603% -0.6856% -0.4151%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

ModuleType::is_texture_std
                        time:   [2.3147 ns 2.3186 ns 2.3239 ns]
                        change: [-1.9097% -1.5987% -1.1806%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe

ModuleType::is_texture_mime
                        time:   [32.679 ns 32.756 ns 32.831 ns]
                        change: [-1.7499% -1.2054% -0.6722%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

ModuleType::is_texture_manual
                        time:   [2.0520 ns 2.0657 ns 2.0824 ns]
                        change: [-8.7075% -8.0549% -7.1646%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high severe

ModuleType::is_texture_std #2
                        time:   [7.5723 ns 7.5904 ns 7.6098 ns]
                        change: [-8.9537% -8.4932% -7.9971%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

ModuleType::is_texture_mime #2
                        time:   [27.372 ns 27.453 ns 27.570 ns]
                        change: [+0.4884% +0.9216% +1.4523%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

ModuleType::is_texture_manual #2
                        time:   [6.1268 ns 6.1600 ns 6.2030 ns]
                        change: [-0.7090% -0.3402% +0.0458%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

ModuleType::is_texture_std #3
                        time:   [37.125 µs 37.223 µs 37.331 µs]
                        change: [-2.5728% -2.2761% -1.9794%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

ModuleType::is_texture_mime #3
                        time:   [27.118 ns 27.208 ns 27.323 ns]
                        change: [-0.6506% -0.2591% +0.4263%] (p = 0.35 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

ModuleType::is_texture_manual #3
                        time:   [10.296 µs 11.559 µs 12.854 µs]
                        change: [-47.428% -43.320% -38.513%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe

Dataurl::from_path      time:   [7.2911 µs 7.3147 µs 7.3452 µs]
                        change: [+0.5418% +0.8471% +1.2078%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

Dataurl::from_str       time:   [97.890 ns 98.028 ns 98.169 ns]
                        change: [+0.0012% +0.2894% +0.5681%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
```