#![deny(warnings)]
#![deny(missing_debug_implementations)]

//! The crate is a utility crate for MIME type handling. It provides `from_ext`, `from_path` with `extension` feature (implemented via `mime_guess` crate), and `from_content` with `magic` feature (implemented via `infer` crate). We also provide a lighter version of `from_ext` and `from_path` with common extensions.
//!
//! This crate was born out of a discussion of the behaviour of MIME inferencing in the [Rolldown](https://github.com/rolldown/rolldown) project Data URLs.

#[cfg(feature = "dataurl")]
pub mod dataurl;
mod light_guess;
mod magic;
pub mod texture;
mod utils;

#[cfg(feature = "dataurl")]
pub use crate::dataurl::Dataurl;

use mime::Mime as MimeType;
use std::str::FromStr;

/// A wrapper around the `mime` crate's `Mime` type, with additional functionality including `from_ext`, `from_path`, and `from_content`, etc.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Mime {
    pub mime: MimeType,
}

impl Mime {
    pub fn new(mime: MimeType) -> Self {
        Self { mime }
    }

    #[cfg(feature = "extension")]
    #[cfg_attr(
        feature = "extension",
        doc = "Guesses the MIME type from the content via [`mime_guess`](https://docs.rs/mime_guess/latest/mime_guess/). It is more accurate than `from_ext_light`, but is slower."
    )]
    pub fn from_path(path: &std::path::Path) -> anyhow::Result<Self> {
        use mime_guess::from_path;
        Ok(Self {
            mime: from_path(path).first().unwrap(),
        })
    }

    #[cfg(feature = "extension")]
    #[cfg_attr(
        feature = "extension",
        doc = "Guesses the MIME type from the extension via [`mime_guess`](https://docs.rs/mime_guess/latest/mime_guess/). It is more accurate than `from_ext_light`, but is slower."
    )]
    pub fn from_ext(ext: &str) -> anyhow::Result<Self> {
        use mime_guess::from_ext;
        Ok(Self {
            mime: from_ext(ext).first().unwrap(),
        })
    }

    #[cfg(feature = "extension-light")]
    #[cfg_attr(
        feature = "extension-light",
        doc = "Guesses the MIME type from the extension. It is more lightweight than `from_ext`, and is suitable for use in performance-critical code, especially bundlers."
    )]
    pub fn from_ext_light(ext: &str) -> anyhow::Result<Self> {
        Ok(Self {
            mime: light_guess::try_from_ext(ext)?,
        })
    }

    #[cfg(feature = "extension-light")]
    #[cfg_attr(
        feature = "extension-light",
        doc = "You can refer to `from_ext_light` for more information. This function is the same as `from_ext_light`, but it takes a `std::path::Path` instead of a `&str`."
    )]
    pub fn from_path_light(path: &std::path::Path) -> anyhow::Result<Self> {
        if let Some(ext) = utils::get_extension(path) {
            Ok(Self {
                mime: light_guess::try_from_ext(ext)?,
            })
        } else {
            anyhow::bail!("No extension found for path: {:?}", path);
        }
    }

    #[cfg(feature = "infer")]
    #[cfg_attr(
        feature = "infer",
        doc = "Guesses the MIME type from the content via [`infer`](https://docs.rs/infer/latest/infer/). It infers the MIME code via the Magic Number, which is useful in customized extensions."
    )]
    pub fn from_content(data: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            mime: magic::mime_type_by_content(data)?,
        })
    }

    pub fn to_mime(&self) -> MimeType {
        self.mime.clone()
    }

    #[cfg(feature = "texture")]
    #[cfg_attr(
        feature = "texture",
        doc = "Check the file is a texture or not. It is useful in the case of handling texture files."
    )]
    pub fn is_texture(self, data: &[u8]) -> bool {
        if data.len() == 0 {
            true
        } else if texture::is_texture_mime(&self.mime) {
            true
        } else if texture::is_texture_std(data) {
            true
        } else {
            false
        }
    }
}

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mime)
    }
}

impl FromStr for Mime {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            mime: MimeType::from_str(s)?,
        })
    }
}

impl<'a> PartialEq<&'a str> for Mime {
    fn eq(&self, other: &&'a str) -> bool {
        self.mime == MimeType::from_str(*other).unwrap()
    }

    fn ne(&self, other: &&'a str) -> bool {
        self.mime != MimeType::from_str(*other).unwrap()
    }
}

/// Guesses the MIME type from the extension and content. It is a combination of `from_ext` and `from_content`, and set the priority of `from_content` higher than `from_ext`.
/// If the function can't guess the MIME type, it will return `application/octet-stream` if the feature `texture` is disabled, otherwise it will return `text/plain` if the data is a texture or `application/octet-stream` if the data is not a texture.
pub fn from_ext_and_content(
    ext: &str,
    #[cfg(feature = "infer")] data: &[u8],
) -> anyhow::Result<Mime> {
    #[cfg(feature = "extension")]
    if let Ok(guessed) = Mime::from_ext(ext) {
        return Ok(guessed);
    }

    #[cfg(feature = "extension-light")]
    if let Ok(guessed_light) = Mime::from_ext_light(ext) {
        return Ok(guessed_light);
    }

    #[cfg(feature = "infer")]
    if let Ok(inferred) = Mime::from_content(data) {
        return Ok(inferred);
    }

    #[cfg(feature = "texture")]
    if texture::is_texture_std(data) {
        return Ok(Mime::new(mime::TEXT_PLAIN));
    }

    Ok(Mime::new(mime::APPLICATION_OCTET_STREAM))
}
