use std::path::Path;
use std::collections::HashMap;
use mime_guess::from_path;

pub fn identify_filetype<P: AsRef<Path>>(path: P) -> &'static str {
    let mime_type = from_path(path).first_or_octet_stream().to_string();

    let lookup: HashMap<&str, &str> = [
        ("application/vnd.openxmlformats-officedocument.wordprocessingml.document", "Word document"),
        ("application/vnd.ms-excel", "Excel document"),
        ("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "Excel sheet"),
        ("application/vnd.ms-powerpoint", "PowerPoint document"),
        ("application/vnd.openxmlformats-officedocument.presentationml.presentation", "PowerPoint presentation"),
        ("application/pdf", "PDF"),
        ("image/jpeg", "JPEG image"),
        ("image/gif", "GIF image"),
        ("image/bmp", "BMP image"),
        ("text/plain", "Text file"),
        ("application/x-elf", "ELF binary"),
        ("application/x-dosexec", "Windows PE binary"),
        ("application/zip", "ZIP archive"),
        ("application/x-rar-compressed", "RAR archive"),
        ("application/gzip", "GZIP archive"),
        ("application/x-tar", "TAR archive"),
        ("image/png", "PNG image"),
        ("image/tiff", "TIFF image"),
        ("text/html", "HTML document"),
        ("text/css", "CSS file"),
        ("application/javascript", "JavaScript file"),
        ("text/xml", "XML document"),
        ("video/mp4", "MP4 video"),
        ("video/x-matroska", "MKV video"),
        ("audio/mpeg", "MP3 audio"),
        ("audio/x-wav", "WAV audio"),
        ("audio/flac", "FLAC audio"),
        ("video/quicktime", "QuickTime video"),
        ("video/webm", "WebM video"),
        ("application/octet-stream", "Binary data"),
        ("application/x-shockwave-flash", "SWF Flash file"),
        ("application/rtf", "Rich Text Format document"),
        ("application/x-sqlite3", "SQLite database file"),
        ("application/vnd.ms-cab-compressed", "Microsoft Cabinet archive"),
        ("application/x-7z-compressed", "7z archive"),
        ("application/x-deb", "Debian package"),
        ("application/vnd.ms-fontobject", "Embedded OpenType font"),
        ("application/font-woff", "WOFF font"),
        ("application/x-font-ttf", "TrueType font"),
        ("image/webp", "WebP image"),
        ("image/x-icon", "Icon image"),
        ("text/csv", "CSV file"),
    ].iter().cloned().collect();

    lookup.get(mime_type.as_str()).unwrap_or(&"Unknown file type")
}
