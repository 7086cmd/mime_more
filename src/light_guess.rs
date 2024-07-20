use mime::Mime;
use std::str::FromStr;

/// Adapted from:
/// - https://github.com/rolldown/rolldown/pull/1406/files#diff-4b612e077c82ae0e05e50eb0d419e02c05a04b83c6ac5440c0d0c9d0c38af942
/// - https://github.com/evanw/esbuild/blob/fc37c2fa9de2ad77476a6d4a8f1516196b90187e/internal/helpers/mime.go#L5
/// Thanks to @ikkz and @evanw for the inspiration.
pub fn mime_type_by_extension(ext: &str) -> Option<&'static str> {
    let mime = match ext.to_lowercase().as_str() {
        // Text
        "txt" => "text/plain",
        "css" => "text/css",
        "htm" | "html" => "text/html",
        "js" | "mjs" | "jsx" | "ecma" | "es" => "text/javascript",
        "json" => "application/json",
        "json5" => "application/json5",
        "yaml" | "yml" => "text/x-yaml",
        "toml" => "text/x-toml",
        "markdown" | "md" => "text/markdown",
        "xhtml" => "application/xhtml+xml",
        "xml" => "text/xml",
        "csv" => "text/csv",
        "tsv" => "text/tab-separated-values",
        // Images
        "bmp" => "image/bmp",
        "avif" => "image/avif",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        "jpeg" | "jpg" => "image/jpeg",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        // Fonts
        "otf" => "font/otf",
        "ttf" => "font/ttf",
        "ttc" => "font/collection",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        // Audios
        "aac" => "audio/aac",
        "midi" | "mid" => "audio/midi",
        "mp3" => "audio/mpeg",
        "oga" | "ogg" => "audio/ogg",
        "wav" => "audio/wav",
        "weba" => "audio/webm",
        "flac" => "audio/flac",
        "m3u" | "m3u8" => "audio/x-mpegurl",
        "m4a" => "audio/m4a",
        // Videos
        "avi" => "video/x-msvideo",
        "mpeg" => "video/mpeg",
        "ogv" => "video/ogg",
        "ivf" => "video/x-ivf",
        "webm" => "video/webm",
        "mp4" => "video/mp4",
        "flv" => "video/x-flv",
        "ts" => "audio/vnd.dlna.mpeg-tts", // Though I write TypeScript, this is not TypeScript
        "mov" => "video/quicktime",
        "wmv" => "video/x-ms-wmv",
        // Other
        "pdf" => "application/pdf",
        "wasm" => "application/wasm",
        "webmanifest" => "application/manifest+json",
        _ => "",
    };
    if mime.is_empty() {
        None
    } else {
        Some(mime)
    }
}

pub fn try_from_ext(ext: &str) -> anyhow::Result<Mime> {
    if let Some(mime) = mime_type_by_extension(ext) {
        Ok(Mime::from_str(mime)?)
    } else {
        anyhow::bail!("No mime type found for extension: {}", ext);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_extensions() {
        assert_eq!(mime_type_by_extension("txt").unwrap().0, "text/plain");
        assert_eq!(mime_type_by_extension("css").unwrap().0, "text/css");
        assert_eq!(mime_type_by_extension("html").unwrap().0, "text/html");
        assert_eq!(
            mime_type_by_extension("json").unwrap().0,
            "application/json"
        );
        assert_eq!(mime_type_by_extension("png").unwrap().0, "image/png");
        assert_eq!(mime_type_by_extension("svg").unwrap().0, "image/svg+xml");
        assert_eq!(mime_type_by_extension("woff2").unwrap().0, "font/woff2");
        assert_eq!(mime_type_by_extension("aac").unwrap().0, "audio/aac");
        assert_eq!(mime_type_by_extension("avi").unwrap().0, "video/x-msvideo");
        assert_eq!(mime_type_by_extension("pdf").unwrap().0, "application/pdf");
        assert_eq!(
            mime_type_by_extension("wasm").unwrap().0,
            "application/wasm"
        );
        assert_eq!(
            mime_type_by_extension("webmanifest").unwrap().0,
            "application/manifest+json"
        );
    }

    #[test]
    fn unknown_extensions() {
        assert!(mime_type_by_extension("unknown").is_none());
    }

    #[test]
    fn try_from_exts() {
        assert!(matches!(try_from_ext("png").unwrap().subtype(), mime::PNG));
        assert!(matches!(try_from_ext("svg").unwrap().subtype(), mime::SVG));
        assert!(matches!(try_from_ext("woff2").unwrap().type_(), mime::FONT));
    }
}
