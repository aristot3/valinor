use tokio::fs as tokio_fs;
use crate::internals;

pub async fn process_files_in_folder(folder_path: &str) {
    let mut read_dir = match tokio_fs::read_dir(folder_path).await {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("Error opening folder: {}", e);
            return;
        }
    };

    while let Some(entry) = read_dir.next_entry().await.unwrap() {
        let file_path = entry.path();
        let file_description = internals::filetype_identifier::identify_filetype(&file_path);
        log::info!("Processing file: {}", file_path.display());
        log::info!("File identified as: {}", file_description);

        if let Err(e) = tokio_fs::remove_file(&file_path).await {
            log::error!("Error deleting file: {}", e);
        } else {
            log::info!("File deleted: {}", file_path.display());
        }
    }
}
