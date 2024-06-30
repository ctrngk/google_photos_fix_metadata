use std::ffi::OsStr;
use regex::Regex;
use std::path::{Path, PathBuf};
use crate::file_utils; // Import the file_utils module

// This is a stupid bug in Takeout from google photos

//  IMG_0743.PNG(1).json -> IMG_0743(1).PNG
//  Stitch1714280447.png(3).json -> Stitch1714280447(3).png
// camphoto_1804928587.jpg(4).json -> camphoto_1804928587(4).jpg
// IMG_0253.HEIC(1).json -> IMG_0253(1).HEIC
// sample.gif.json -> sample.gif
// sample.png.json -> sample.png




// pub fn swap_position(json_file_path: &Path) -> Option<PathBuf> {
//     let regex = Regex::new(r"(?P<filename>.+?)\.(?P<extension>[^.]+)\((?P<number>\d+)\)\.json").unwrap();
//
//     // Get the string representation of the path
//     let path_str = json_file_path.to_str()?;
//
//     // Capture the groups using regex
//     regex.captures(path_str).map(|caps| {
//         // Create a new PathBuf from the original path
//         let mut new_path = json_file_path.to_path_buf();
//
//         // Build new file name with swapped order
//         let new_file_name = format!("{filename}({number}).{extension}.json",
//                                     filename = caps.name("filename").unwrap().as_str(),
//                                     extension = caps.name("extension").unwrap().as_str(),
//                                     number = caps.name("number").unwrap().as_str().parse::<u32>().unwrap());
//
//         // Change to the new file name by applying regex
//         new_path.set_file_name(new_file_name);
//
//         // Return the new path
//         new_path
//     })
// }


pub fn swap_file_name_str_position(file_name: &str) -> String {
    let regex = Regex::new(r"(?P<filename>.+?)\.(?P<extension>[^.]+)\((?P<number>\d+)\)\.json").unwrap();

    if let Some(caps) = regex.captures(file_name) {
        format!("{filename}({number}).{extension}.json",
                filename = caps.name("filename").unwrap().as_str(),
                extension = caps.name("extension").unwrap().as_str(),
                number = caps.name("number").unwrap().as_str().parse::<u32>().unwrap())
    } else {
        file_name.to_string()  // Return the original file name if it doesn't match the regex
    }
}



pub fn swap_position(json_file_path: &Path) -> Option<PathBuf> {
    // Split the path into components
    let components = file_utils::split_path_components(json_file_path);

    // Handle the file name
    let new_file_name = swap_file_name_str_position(&components.file_name);

    // Concatenate old parent path and new_file_name
    let mut new_path = components.parent_path;
    new_path.push(new_file_name);

    Some(new_path)
}

// pub fn swap_position(json_file_path: &Path) -> Option<PathBuf> {
//     let regex = Regex::new(r"(?P<filename>.+?)\.(?P<extension>[^.]+)\((?P<number>\d+)\)\.json").unwrap();
//
//     // Get the string representation of the path
//     let path_str = json_file_path.to_str()?;
//
//     // Capture the groups using regex
//     regex.captures(path_str).map(|caps| {
//         // Create a new PathBuf from the parent directory of the original path
//         let parent_dir = json_file_path.parent().unwrap_or_else(|| Path::new(""));
//         let mut new_path = parent_dir.to_path_buf();
//
//         // Build new file name with swapped order
//         let new_file_name = format!("{filename}({number}).{extension}.json",
//                                     filename = caps.name("filename").unwrap().as_str(),
//                                     extension = caps.name("extension").unwrap().as_str(),
//                                     number = caps.name("number").unwrap().as_str().parse::<u32>().unwrap());
//
//         // Change to the new file name by applying regex
//         new_path.push(new_file_name);
//
//         // Return the new path
//         new_path
//     })
// }




fn strip_json_suffix(json_file_path: &Path) -> PathBuf {
    // This code makes use of the with_extension method
    // to remove the .json suffix.
    // If the file path doesn't have a .json suffix,
    // it will simply return the original path.
    json_file_path.with_extension("")
}

pub fn get_new_image_file_path_by_swap_position(json_file_path: &Path) -> PathBuf {

    let swapped_path = match swap_position(json_file_path) {
        Some(path) => path,
        None => json_file_path.to_path_buf(), // Return original path if swap_position fails
    };

    // Call strip_json_suffix to remove the .json suffix
    strip_json_suffix(&swapped_path)
}


// BAAC2A4F-AF2C-44EE-B4BF-5FCB1FC0EE38-5325-00000.png
// BAAC2A4F-AF2C-44EE-B4BF-5FCB1FC0EE38-5325-0000.json
pub fn fix_image_file_path_by_fix_0(json_file_path: &Path) -> PathBuf {

    let mut new_dest = json_file_path.to_path_buf();
    let file_stem = json_file_path.file_stem().unwrap_or_else(|| OsStr::new("")).to_str().unwrap_or("");
    let extension = json_file_path.extension().unwrap_or_else(|| OsStr::new("")).to_str().unwrap_or("");

    let new_file_name = format!("{}0.png", file_stem);
    new_dest = json_file_path.with_file_name(new_file_name);

    new_dest

}
