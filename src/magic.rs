#![cfg(feature = "magic")]

/// Guesses the MIME type from the content via [`infer`](https://docs.rs/infer/latest/infer/).
pub fn mime_type_by_content(data: &[u8]) -> anyhow::Result<mime::Mime> {
    use infer::get;
    use mime::Mime;
    use std::str::FromStr;
    let inferred = get(data).ok_or_else(|| anyhow::anyhow!("Failed to infer mime type"))?;
    Ok(Mime::from_str(inferred.mime_type())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html() {
        let data =
            b"<!DOCTYPE html><html><head><title>Test</title></head><body><p>Test</p></body></html>";
        assert_eq!(mime_type_by_content(data).unwrap(), "text/html");
    }

    #[test]
    fn test_mp3() {
        let data = &[0xFF, 0xFB, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0]; // Music starts with FF FB
        assert_eq!(mime_type_by_content(data).unwrap(), "audio/mpeg");
    }

    #[test]
    fn test_jpg() {
        let data = &[
            0xFF, 0xD8, 0xFF, 0xE0, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
        ]; // Image starts with FF D8 FF E0
        assert_eq!(mime_type_by_content(data).unwrap(), "image/jpeg");
    }

    #[test]
    fn test_script() {
        let data = b"#!/usr/bin/env python\nprint('Hello, World!')";
        assert_eq!(mime_type_by_content(data).unwrap(), "text/x-shellscript");
    }

    #[test]
    fn test_png() {
        let data = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
        ];
        assert_eq!(mime_type_by_content(data).unwrap(), "image/png");
    }

    #[test]
    fn test_pdf() {
        let data = &[
            0x25, 0x50, 0x44, 0x46, 0x2D, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
        ];
        assert_eq!(mime_type_by_content(data).unwrap(), "application/pdf");
    }
}
