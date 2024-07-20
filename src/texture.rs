use std::cmp::min;

#[cfg(feature = "texture")]
#[cfg_attr(
    feature = "texture",
    doc = "Checks if the data is a texture via `from_utf8`. This function performs better in small files."
)]
pub fn is_texture_std(data: &[u8]) -> bool {
    std::str::from_utf8(data).is_ok()
}

#[cfg(feature = "texture")]
#[cfg_attr(
    feature = "texture",
    doc = "Checks if the data is a texture. This function is faster in large files than `is_texture_std`, but it can also be slower in some cases."
)]
pub fn is_texture_manual(data: &[u8], min_infer_length: i32) -> bool {
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
