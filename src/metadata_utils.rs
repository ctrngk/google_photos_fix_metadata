use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::str;
use crate::file_utils::{get_extension, restore_file_modification_time};
use crate::update_media_metadata_from_json;

pub fn add_metadata_wrapper(image_file_path: &str, value: &str) -> std::io::Result<()>  {
    let actual_extension = get_media_file_type(image_file_path);

    match actual_extension.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "heic" | "tiff" | "tif" | "webp" | "mp4" | "mov" => {

            let original_extension = get_original_extension(image_file_path);
            let original_metadata = fs::metadata(image_file_path)?;

            // change png to jpg, for example
            let new_image_file_path = rename_file(image_file_path, &actual_extension);
            add_metadata_with_exiftool(&new_image_file_path, value);


            // Rename back to original extension
            // change jpg to png, for example
            rename_file(&new_image_file_path, &original_extension);

            // Restore back to original modification time
            restore_file_modification_time(image_file_path, original_metadata);

            // Finally, sync the modification time with the creation tag.
            // Google Photos surprisingly ignores the DateTimeOriginal and CreateDate tags of GIFs,
            // but respects the modification time
            sync_metadata_modification_from_DateTimeOriginal_and_CreateDate(image_file_path);

            Ok(())
        },
        "unknown" => {

            let extension = get_extension(image_file_path);

            match extension {
                Some(ext) => {
                    if ext == "AAE" || ext == "aae" { // Check for both "AAE" and "aae"
                        println!("Extension: {}", ext);
                        println!("do not modify apple *.AAE file");
                        Ok(())
                    } else {
                        panic!("Unsupported or misidentified file format: {}, image_file_path: {}", actual_extension, image_file_path);
                    }
                },
                None => {
                    println!("No extension found");
                    panic!("Unsupported or misidentified file format: {}, image_file_path: {}", actual_extension, image_file_path);
                },
            }
        },
        _ => panic!("Unsupported or misidentified file format: {}, image_file_path: {}", actual_extension, image_file_path),
    }
}

fn get_media_file_type(file_path: &str) -> String {
    let output = Command::new("file")
        .arg("--mime-type")
        .arg("-b")
        .arg(file_path)
        .output()
        .expect("Failed to execute file command");

    if output.status.success() {
        let mime_type = str::from_utf8(&output.stdout).unwrap_or("").trim();
        match mime_type {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            "image/gif" => "gif",
            "image/heic" => "heic",
            "image/tiff" => "tiff",
            "image/webp" => "webp",
            "video/mp4" => "mp4",
            "video/quicktime" => "mov",
            _ => "unknown",
        }.to_string()
    } else {
        panic!("Failed to determine file type: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn get_original_extension(file_path: &str) -> String {
    Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string()
}

fn rename_file(file_path: &str, new_extension: &str) -> String {
    let path = Path::new(file_path);
    let new_file_path = path.with_extension(new_extension);
    fs::rename(file_path, &new_file_path).expect("Failed to rename file");
    new_file_path.to_str().unwrap().to_string()
}


fn sync_metadata_modification_from_DateTimeOriginal_and_CreateDate(image_path: &str) -> Result<(), String> {

    // Check if the file exists
    if !Path::new(image_path).exists() {
        panic!("File {} does not exist", image_path);
    }

    // // Check if either "Date/Time Original" or "Create Date" tag exists
    // let output = Command::new("exiftool")
    //     .arg("-DateTimeOriginal")
    //     .arg("-CreateDate")
    //     .arg("-s")
    //     .arg(file_path)
    //     .output()
    //     .expect("Failed to execute exiftool");
    //
    // let output_str = String::from_utf8_lossy(&output.stdout);

    // if output_str.contains("DateTimeOriginal") || output_str.contains("CreateDate") {
    //     return Ok(());  // If either tag exists, skip the rest of the code
    // }

    // https://exiftool.org/forum/index.php?topic=7843.0
    let status = Command::new("exiftool")
        .arg("-filemodifydate<createdate")
        .arg("-filecreatedate<createdate")
        .arg("-filemodifydate<datetimeoriginal")
        .arg("-filecreatedate<datetimeoriginal")
        .arg("-overwrite_original")
        .arg(image_path)
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => panic!("sync_modification error"),
    }


}



fn add_metadata_with_exiftool(file_path: &str, value: &str) -> Result<(), String> {

    // Check if the file exists
    if !Path::new(file_path).exists() {
        panic!("File {} does not exist", file_path);
    }

    // Check if either "Date/Time Original" or "Create Date" tag exists
    let output = Command::new("exiftool")
        .arg("-DateTimeOriginal")
        .arg("-CreateDate")
        .arg("-s")
        .arg(file_path)
        .output()
        .expect("Failed to execute exiftool");

    let output_str = String::from_utf8_lossy(&output.stdout);

    if output_str.contains("DateTimeOriginal") || output_str.contains("CreateDate") {
        return Ok(());  // If either tag exists, skip the rest of the code
    }

    let original_metadata = fs::metadata(file_path).expect("Failed to get file metadata");

    match add_date_time_tags(file_path, value) {
        Ok(_) => println!("Date/Time tags added successfully"),
        Err(e) => println!("Error: {}", e),
    }


    Ok(())

}

fn generate_metadata_xml_name(file_path: &str) -> String {
    let path = Path::new(file_path);
    let xml_path = path.with_extension("xml");
    xml_path.to_str().unwrap().to_string()
}



fn add_date_time_tags(file_path: &str, value: &str) -> Result<ExitStatus, String> {
    // Attempt to add the new Date/Time tags
    let status = Command::new("exiftool")
        .arg(format!("-DateTimeOriginal={}", value))
        .arg(format!("-CreateDate={}", value))
        .arg("-ignoreMinorErrors")
        .arg("-overwrite_original")
        .arg("-preserve")
        .arg(file_path)
        .status();

    match status {
        Ok(s) if s.success() => Ok(s),
        _ => {
            // Generate the metadata filename based on the file path name
            let metadata_xml_filename = generate_metadata_xml_name(file_path);

            // If it fails, extract and re-add metadata, then add new Date/Time tags
            let exported_status = Command::new("exiftool")
                .arg("-ignoreMinorErrors")
                .arg("-X") // Extract metadata as XML
                .arg(file_path)
                .arg("-o")
                .arg(&metadata_xml_filename)
                .status()
                .expect("Failed to extract metadata");

            println!("metadata successfully exported to: {}", metadata_xml_filename);

            let delete_status = Command::new("exiftool")
                .arg("-all=")
                .arg("-ignoreMinorErrors")
                .arg("-overwrite_original")
                .arg("-preserve")
                .arg(file_path)
                .status()
                .expect("Failed to delete metadata");

            println!("file_path {}'s metadata successfully deleted", file_path);

            let add_status = Command::new("exiftool")
                .arg("-ignoreMinorErrors")
                .arg("-overwrite_original")
                .arg("-preserve")
                .arg("-tagsfromfile")
                .arg(&metadata_xml_filename)
                .arg(file_path)
                .status()
                .expect("Failed to add metadata back");

            println!("file_path {}'s metadata successfully add it back", file_path);

            let retry_status = Command::new("exiftool")
                .arg(format!("-DateTimeOriginal={}", value))
                .arg(format!("-CreateDate={}", value))
                .arg("-ignoreMinorErrors")
                .arg("-overwrite_original")
                .arg("-preserve")
                .arg(file_path)
                .status()
                .expect("Failed to execute exiftool on retry");

            if retry_status.success() {
                Ok(retry_status)
            } else {
                Err("Failed to add Date/Time tags on retry".to_string())
            }
        }
    }
}