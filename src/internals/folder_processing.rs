use tokio::fs as tokio_fs;
use crate::internals::filetype_identifier;
use crate::parsers;

pub async fn process_files_in_folder(folder_path: &str) {
    let mut read_dir = match tokio_fs::read_dir(folder_path).await {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("Error opening folder: {}", e);
            return;
        }
    };

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let file_path = entry.path();
        let file_description = filetype_identifier::identify_filetype(&file_path);
        log::info!("Processing file: {}", file_path.display());
        log::info!("File identified as: {}", file_description);
        
        let parsed_content = match file_description {
            "PDF" => parsers::pdf::parse(&file_path),
            "Word document" => parsers::docx::parse(&file_path),
            _ => {
                log::warn!("No parser available for file type: {}", file_description);
                // I may implement a generic parser that does a basic "strings" as a fallback - unsure yet
                Ok(String::new())
            }
        };
        match &parsed_content {
            Ok(content) => log::info!("Parsed content: {}", content),
            Err(e) => log::error!("Error while parsing: {}", e),
        }

        if let Err(e) = tokio_fs::remove_file(&file_path).await {
            log::error!("Error deleting file: {}", e);
        } else {
            log::info!("File deleted: {}", file_path.display());
        }
    }
}
