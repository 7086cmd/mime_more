#![cfg(feature = "texture")]

//! We use this feature to determine if the data is a texture. This is useful for bundlers, as it can help to determine if the data is a texture, and if it is, we can use a different algorithm to process it.
//! In Dataurl, we use this feature to avoid applying Base64 to the data if it is a texture. This can save a lot of time and memory, especially for large files.
//!
//! # Example
//!
//! ```rust
//! use mime_more::texture::is_texture_std;
//!
//! let texture = b"Hello, world! This is Rust.";
//! let texture_with_utf8 = "Hello, world! This is Rust. \nこんにちは、世界！\n你好，世界！".as_bytes();
//! let non_texture = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0];
//!
//! assert!(is_texture_std(texture));
//! assert!(is_texture_std(texture_with_utf8));
//! assert!(!is_texture_std(non_texture));
//! ```

#[cfg_attr(
    feature = "texture",
    doc = "Checks if the data is a texture via `from_utf8`. This function performs better in small files."
)]
pub fn is_texture_std(data: &[u8]) -> bool {
    std::str::from_utf8(data).is_ok()
}

#[cfg_attr(
    feature = "texture",
    doc = "Checks if the data is a texture. This function is faster in large files than `is_texture_std`, but it can also be slower in some cases."
)]
pub fn is_texture_manual(data: &[u8], min_infer_length: i32) -> bool {
    use std::cmp::min;
    let min_infer_length = min_infer_length as usize;
    let mut i = 0;
    while i < min(min_infer_length, data.len()) {
        let byte = data[i];

        if byte.is_ascii() {
            // ASCII byte (0x00 to 0x7F), single byte UTF-8
            i += 1;
        } else if (byte & 0b1110_0000) == 0b1100_0000 {
            // Two-byte UTF-8 sequence (0xC0 to 0xDF)
            if i + 1 >= data.len() || (data[i + 1] & 0b1100_0000) != 0b1000_0000 {
                return false; // Invalid UTF-8 sequence
            }
            i += 2;
        } else if (byte & 0b1111_0000) == 0b1110_0000 {
            // Three-byte UTF-8 sequence (0xE0 to 0xEF)
            if i + 2 >= data.len()
                || (data[i + 1] & 0b1100_0000) != 0b1000_0000
                || (data[i + 2] & 0b1100_0000) != 0b1000_0000
            {
                return false; // Invalid UTF-8 sequence
            }
            i += 3;
        } else if (byte & 0b1111_1000) == 0b1111_0000 {
            // Four-byte UTF-8 sequence (0xF0 to 0xF7)
            if i + 3 >= data.len()
                || (data[i + 1] & 0b1100_0000) != 0b1000_0000
                || (data[i + 2] & 0b1100_0000) != 0b1000_0000
                || (data[i + 3] & 0b1100_0000) != 0b1000_0000
            {
                return false;
            }
            i += 4;
        } else {
            return false; // Invalid UTF-8 byte
        }
    }

    true // All bytes are part of valid UTF-8 sequences
}

#[cfg(feature = "texture")]
#[cfg_attr(
    feature = "texture",
    doc = "Check if the MIME is a texture. This function uses `mime`, and returns if the MIME type is a texture (type is `text`, or is `json`, `svg`, or `xml`, etc.)."
)]
pub fn is_texture_mime(mime: &mime::Mime) -> bool {
    mime.type_() == mime::TEXT
        || mime.subtype() == mime::JSON
        || mime.subtype() == mime::SVG
        || mime.subtype() == mime::XML
}

#[cfg(feature = "texture")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mime_is_texture() {
        assert!(is_texture_mime(&mime::TEXT_PLAIN));
        assert!(is_texture_mime(&mime::APPLICATION_JSON));
        assert!(is_texture_mime(&mime::IMAGE_SVG));
        assert!(is_texture_mime(&mime::TEXT_HTML));
        assert!(!is_texture_mime(&mime::APPLICATION_OCTET_STREAM));
        assert!(!is_texture_mime(&mime::APPLICATION_PDF));
        assert!(!is_texture_mime(&mime::IMAGE_PNG));
        assert!(!is_texture_mime(&mime::IMAGE_BMP));
    }

    #[test]
    fn std_content_is_texture() {
        let texture = b"Hello, world! This is Rust.";
        let binding = "Hello, world! This is Rust. こんにちは、世界！你好，世界！".to_string();
        let texture_with_utf8 = binding.as_bytes();
        let non_texture = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
        ];

        assert!(is_texture_std(texture));
        assert!(is_texture_std(texture_with_utf8));
        assert!(!is_texture_std(non_texture));
    }

    #[test]
    fn manual_content_is_texture() {
        let texture = b"Hello, world! This is Rust.";
        let binding = "Hello, world! This is Rust. こんにちは、世界！你好，世界！".to_string();
        let texture_with_utf8 = binding.as_bytes();
        let non_texture = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
        ];

        assert!(is_texture_manual(texture, 64));
        assert!(is_texture_manual(texture_with_utf8, 64));
        assert!(!is_texture_manual(non_texture, 64));
    }
}
