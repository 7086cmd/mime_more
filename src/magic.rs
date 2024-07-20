#[cfg(feature = "magic")]
pub fn mime_type_by_content(data: &[u8]) -> anyhow::Result<mime::Mime> {
    use infer::get;
    use mime::Mime;
    use std::str::FromStr;
    let inferred = get(data).ok_or_else(|| anyhow::anyhow!("Failed to infer mime type"))?;
    Ok(Mime::from_str(inferred.mime_type())?)
}
