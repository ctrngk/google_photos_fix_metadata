mod metadata_utils;
mod file_utils;
mod fix_stupid_google_photos_takeout_naming_bug;

use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use chrono::{DateTime, NaiveDateTime, Utc};
use clap::{Arg, Command};
use walkdir::{DirEntry, WalkDir};
use google_photos_fix_metadata::get_new_image_file_path_by_swap_position;
use crate::fix_stupid_google_photos_takeout_naming_bug::fix_image_file_path_by_fix_0;
use crate::metadata_utils::add_metadata_wrapper;

fn get_recursive_file_list(path: &str) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file())
        .collect()
}

fn filter_excluded_files(files: Vec<DirEntry>, excluded_files: &[&str]) -> Vec<DirEntry> {
    files
        .into_iter()
        .filter(|entry| {
            if let Some(file_name) = entry.path().file_name().and_then(|f| f.to_str()) {
                !excluded_files.iter().any(|&excluded| file_name.contains(excluded))
            } else {
                false
            }
        })
        .collect()
}

fn get_metadata_from_json(file_path: &DirEntry) -> Option<String> {
    let file = File::open(file_path.path()).ok()?;
    let reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader).ok()?;

    let timestamp_str = json
        .get("photoTakenTime")?
        .get("timestamp")?
        .as_str()?;

    let timestamp = timestamp_str.parse::<i64>().ok()?;

    let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime_utc: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    // %H is 24-hr, verified. Do not change to %I 12-hr
    let formatted_str = datetime_utc.format("%Y:%m:%d %H:%M:%S%.3f%:z").to_string();

    Some(formatted_str)
}

fn get_all_json_files(all_files: Vec<DirEntry>) -> Vec<DirEntry> {
    // Filter out only the files that have the ".json" extension
    all_files
        .into_iter()
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext_str| ext_str.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
        .collect()
}

fn check_if_every_json_has_media_file_beforehand(filtered_json_files: &[DirEntry]) {

    let mut panic = false;
    for file in filtered_json_files {
        if let Some(metadata_str) = get_metadata_from_json(&file) {

            // let image_file_path = file.path().to_str().map(|path_str| fix_image_file_path_by_swap_position(path_str));
            let image_file_path = get_new_image_file_path_by_swap_position(&file.path());

            if !fs::metadata(&image_file_path).is_ok() {
                //  Try to find it by fixing 0
                let json_file_path = file.path();
                let image_file_path = fix_image_file_path_by_fix_0(json_file_path);
                if !image_file_path.exists() {
                    println!("Relevant image file not found: {:?}", file);
                    panic = true;
                }
            }
        }
    }


    if panic {
        panic!("Relevant image file not found");
    }

}

fn update_media_metadata_from_json(json_file: &DirEntry) {
    if let Some(metadata_str) = get_metadata_from_json(json_file) {
        let mut image_file_path = get_new_image_file_path_by_swap_position(&json_file.path());

        if fs::metadata(&image_file_path).is_err() {
            image_file_path = fix_image_file_path_by_fix_0(&json_file.path());

            if fs::metadata(&image_file_path).is_err() {
                panic!("cannot find associated image file");
            }
        }

        if let Some(image_file_path_str) = image_file_path.to_str() {
            add_metadata_wrapper(image_file_path_str, &metadata_str)
                .expect("Failed to add metadata");
        } else {
            eprintln!("Invalid UTF-8 path");
            panic!("Invalid UTF-8 path");
        }
    }
}

fn patch_google_photos_image(directories: Vec<&str>) {
    let excluded_files = vec![
        "print-subscriptions.json",
        "shared_album_comments.json",
        "user-generated-memory-titles.json",
    ];

    for path in directories {
        let all_files = get_recursive_file_list(path);

        let all_json_files = get_all_json_files(all_files);

        let filtered_json_files = filter_excluded_files(all_json_files, &excluded_files);


        check_if_every_json_has_media_file_beforehand(&filtered_json_files);

        for json_file in &filtered_json_files {
            println!("Filtered file: {:?}", json_file.path());
        }

        for json_file in &filtered_json_files {
            update_media_metadata_from_json(json_file)
        }

    }
}

fn copy_files_to_output(directories: Vec<&str>, output_dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(output_dir)?;

    for path in directories {
        let all_files = get_recursive_file_list(path);

        for file in all_files {
            if let Some(file_name) = file.path().file_name() {
                let dest_path = output_dir.join(file_name);
                let file_extension = file.path().extension().and_then(|ext| ext.to_str()).unwrap_or("");

                if file_extension != "json" && file_extension != "html" && file_extension != "xml" && file_extension != "zip" {
                    if let Err(e) = file_utils::copy_file_preserving_metadata(file.path(), &dest_path) {
                        eprintln!("Failed to copy file: {:?} to {:?} due to {:?}", file.path(), dest_path, e);
                        panic!("panic!");
                    } else {
                        println!("Copied {:?} to {:?}", file.path(), dest_path);
                    }
                }
            }
        }
    }
    Ok(())
}

fn process_iphone_photos(directories: Vec<&str>) {
    for path in directories {
        let all_files = get_recursive_file_list(path);

        for file in all_files {
            if let Ok(metadata) = fs::metadata(file.path()) {
                if let Ok(modified_time) = metadata.modified() {
                    let datetime: DateTime<Utc> = DateTime::from(modified_time);
                    // %H is 24-hr, and is verified here. Do not change to %I, 12-hr.
                    let formatted_str = datetime.format("%Y:%m:%d %H:%M:%S%.3f%:z").to_string();
                    add_metadata_wrapper(file.path().to_str().unwrap(), &formatted_str);
                }
            }
        }
    }
}

fn main() {
    let matches = Command::new("Photo Metadata Patcher")
        .version("1.0")
        .about("Patches metadata of photos based on JSON files")
        .arg(
            Arg::new("src-google-photos")
                .long("src-google-photos")
                .action(clap::ArgAction::Append)
                .help("Source directories for Google Photos"),
        )
        .arg(
            Arg::new("src-iphone-photos")
                .long("src-iphone-photos")
                .action(clap::ArgAction::Append)
                .help("Source directories for iPhone Photos"),
        )
        .get_matches();

    if let Some(directories) = matches.get_many::<String>("src-google-photos") {
        let directories: Vec<&str> = directories.map(|s| s.as_str()).collect();
        patch_google_photos_image(directories.clone());

        let output_dir = Path::new("output");

        if let Err(e) = copy_files_to_output(directories, output_dir) {
            eprintln!("Failed to copy files to output directory: {:?}", e);
        }
    }

    if let Some(directories) = matches.get_many::<String>("src-iphone-photos") {
        let directories: Vec<&str> = directories.map(|s| s.as_str()).collect();
        process_iphone_photos(directories.clone());

        let output_dir = Path::new("output");

        if let Err(e) = copy_files_to_output(directories, output_dir) {
            eprintln!("Failed to copy files to output directory: {:?}", e);
        }
    }
}
