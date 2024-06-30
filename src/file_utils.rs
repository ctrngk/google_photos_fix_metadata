use std::fs;
use filetime::{FileTime, set_file_times};
use std::str;
use std::path::{Path, PathBuf};
use std::io;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::ffi::OsStr;


pub(crate) fn restore_file_modification_time(file_path: &str, metadata: fs::Metadata) -> std::io::Result<()> {
    // usage:
    // let metadata = fs::metadata(src_path)?;
    // restore_file_modification_time(file_path, metadata);
    //
    let accessed = FileTime::from_last_access_time(&metadata);
    let modified = FileTime::from_last_modification_time(&metadata);
    set_file_times(file_path, accessed, modified)?;
    Ok(())
}

pub fn copy_file_preserving_metadata(src: &Path, dest: &Path) -> io::Result<()> {
    println!("copying src_path to dest_path: {:?} {:?}", src, dest);
    let final_dest = if dest.exists() {
        let unique_path = generate_unique_path(dest);
        println!("generate_unique_path -> final_dest: {:?}", unique_path);
        unique_path
    } else {
        dest.to_path_buf()
    };

    fs::copy(src, &final_dest)?;
    let metadata = fs::metadata(src)?;
    let accessed = FileTime::from_last_access_time(&metadata);
    let modified = FileTime::from_last_modification_time(&metadata);
    set_file_times(&final_dest, accessed, modified)?;
    Ok(())
}

fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn generate_unique_path(dest: &Path) -> PathBuf {
    let mut new_dest = dest.to_path_buf();
    let file_stem = dest.file_stem().unwrap_or_else(|| OsStr::new("")).to_str().unwrap_or("");
    let extension = dest.extension().unwrap_or_else(|| OsStr::new("")).to_str().unwrap_or("");

    while new_dest.exists() {
        let random_string = generate_random_string(6); // Generates a 6-character random string
        let new_file_name = format!("{}-{}.{}", file_stem, random_string, extension);
        new_dest = dest.with_file_name(new_file_name);
    }

    new_dest
}

pub struct PathComponents {
    pub parent_path: PathBuf,
    pub file_name: String,
}

pub fn split_path_components(path: &Path) -> PathComponents {
    let parent_path = path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
    let file_name = path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("")).to_string_lossy().to_string();

    PathComponents {
        parent_path,
        file_name,
    }
}

pub fn get_extension(image_file_path: &str) -> Option<&str> {
    let path = Path::new(image_file_path);
    path.extension().and_then(OsStr::to_str)
}