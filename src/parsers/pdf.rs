use std::path::Path;

pub fn parse<P: AsRef<Path>>(_path: P) -> Result<String, String> {
    Ok("MOCK_PDF_PARSER".to_string())
}