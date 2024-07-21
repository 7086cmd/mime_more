#![cfg(feature = "dataurl")]
//! This is a high-performance implementation of the `dataurl` scheme. We avoid cloning the data, and we avoid unnecessary conversions.
//!
//! # Example
//!
//! ```rust
//! use mime_more::dataurl::Dataurl;
//! use std::path::Path;
//!
//! let content = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0];
//!
//! let dataurl = Dataurl::from_data(content.to_vec()).unwrap();
//! assert_eq!(dataurl.mime, "image/png");
//! let string = dataurl.to_string();
//! assert_eq!(string, "data:image/png;base64,iVBORw0KGgoRRRQZGYEA");
//! ```

use crate::magic::mime_type_by_content;
use crate::utils::get_extension;
use crate::{from_ext_and_content, Mime};
use base64::engine::{general_purpose, Engine as _};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use urlencoding::encode;

/// The `DataurlCharset` enum represents the charset of a `dataurl` scheme.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub enum DataurlCharset {
    Utf8,
    Base64,
}

impl DataurlCharset {
    pub fn to_texture_bool(self) -> bool {
        match self {
            Self::Utf8 => true,
            Self::Base64 => false,
        }
    }
}

impl FromStr for DataurlCharset {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "utf-8" => Ok(Self::Utf8),
            "base64" => Ok(Self::Base64),
            _ => Err(anyhow::anyhow!("Invalid charset")),
        }
    }
}

impl From<bool> for DataurlCharset {
    fn from(b: bool) -> Self {
        if b {
            Self::Utf8
        } else {
            Self::Base64
        }
    }
}

impl std::fmt::Display for DataurlCharset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8 => write!(f, "utf-8"),
            Self::Base64 => write!(f, "base64"),
        }
    }
}

/// The `Dataurl` struct represents a `dataurl` scheme. It contains the MIME type and the data.
#[cfg_attr(feature, feature = "dataurl")]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Dataurl {
    pub mime: Mime,
    pub data: Vec<u8>,
    pub charset: DataurlCharset,
}

impl Dataurl {
    pub fn new(mime: Mime, data: Vec<u8>) -> Self {
        Self {
            charset: mime.clone().is_texture(&data).into(),
            mime,
            data,
        }
    }

    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let mime = from_ext_and_content(get_extension(path).unwrap(), &data)?;
        let charset = mime.clone().is_texture(&data).into();
        Ok(Self {
            charset,
            mime,
            data,
        })
    }

    pub fn from_data(data: Vec<u8>) -> anyhow::Result<Self> {
        let mime = mime_type_by_content(&data)?;
        let mime = Mime::new(mime);
        let charset = mime.clone().is_texture(&data).into();
        Ok(Self {
            charset,
            mime,
            data,
        })
    }

    pub fn is_texture(&self) -> bool {
        self.charset.to_texture_bool()
    }
}

/// Implement the `Display` trait for `Dataurl`, so that we can directly use `to_string` to convert it to a `String`.
impl std::fmt::Display for Dataurl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_texture() {
            let text = encode(&std::str::from_utf8(&self.data).unwrap());
            write!(f, "data:{};charset=utf-8,{text}", self.mime)
        } else {
            let encoded = general_purpose::STANDARD.encode(&self.data);
            write!(f, "data:{};base64,{encoded}", self.mime)
        }
    }
}

impl FromStr for Dataurl {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("data:") {
            return Err(anyhow::anyhow!("Invalid dataurl"));
        }
        let s = s[5..].trim_start();
        let (mime, body) = s
            .split_once(';')
            .ok_or_else(|| anyhow::anyhow!("Invalid dataurl"))?;
        let (charset, data) = body
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("Invalid dataurl"))?;
        let mime = Mime::from_str(mime)?;
        let charset = DataurlCharset::from_str(charset)?;
        let data = if charset == DataurlCharset::Utf8 {
            let text = urlencoding::decode(data).unwrap();
            text.as_bytes().to_vec()
        } else {
            let data = general_purpose::STANDARD
                .decode(data.as_bytes())
                .unwrap();
            data
        };
        Ok(Self {
            charset,
            mime,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataurl() {
        let content = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x0,
        ];

        let dataurl = Dataurl::from_data(content.to_vec()).unwrap();
        assert_eq!(dataurl.mime, "image/png");
        let string = dataurl.to_string();
        assert_eq!(string, "data:image/png;base64,iVBORw0KGgoRRRQZGYEA");

        let dataurl = Dataurl::from_str("data:image/png;base64,iVBORw0KGgoRRRQZGYEA").unwrap();
        assert_eq!(dataurl.mime, "image/png");
        assert_eq!(dataurl.data, content.to_vec());
    }
}
