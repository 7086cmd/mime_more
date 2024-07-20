pub fn get_extension(path: &std::path::Path) -> Option<&str> {
    path.extension().and_then(|ext| ext.to_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension(std::path::Path::new("test.txt")), Some("txt"));
        assert_eq!(get_extension(std::path::Path::new("test")), None);
    }
}
