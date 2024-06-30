use google_photos_fix_metadata::{swap_position, get_new_image_file_path_by_swap_position};
use std::path::{Path, PathBuf};
use google_photos_fix_metadata::fix_stupid_google_photos_takeout_naming_bug::swap_file_name_str_position;

#[test]
fn test_swap_file_name_str_position() {
    assert_eq!(
        swap_file_name_str_position("IMG_0253.HEIC(2).json"),
        "IMG_0253(2).HEIC.json"
    );
    assert_eq!(
        swap_file_name_str_position("IMG_0743.PNG(1).json"),
        "IMG_0743(1).PNG.json"
    );
    assert_eq!(
        swap_file_name_str_position("Stitch1714280447.png(3).json"),
        "Stitch1714280447(3).png.json"
    );
    assert_eq!(
        swap_file_name_str_position("Stitch1714280447.png.json"),
        "Stitch1714280447.png.json"
    );
    assert_eq!(
        swap_file_name_str_position("example.png.json"),
        "example.png.json"
    );
}

#[test]
fn test_swap() {
    assert_eq!(
        swap_position(Path::new("IMG_0743.PNG(1).json")),
        Some(PathBuf::from("IMG_0743(1).PNG.json"))
    );
    assert_eq!(
        swap_position(Path::new("Stitch1714280447.png(3).json")),
        Some(PathBuf::from("Stitch1714280447(3).png.json"))
    );
    assert_eq!(
        swap_position(Path::new("/home/camphoto_1804928587.jpg(4).json")),
        Some(PathBuf::from("/home/camphoto_1804928587(4).jpg.json"))
    );
    assert_eq!(
        swap_position(Path::new("./IMG_0253.HEIC(1).json")),
        Some(PathBuf::from("./IMG_0253(1).HEIC.json"))
    );
    assert_eq!(
        swap_position(Path::new("./test/IMG_0253.HEIC(2).json")),
        Some(PathBuf::from("./test/IMG_0253(2).HEIC.json"))
    );
    assert_eq!(
        swap_position(Path::new("Stitch1714280447.png.json")),
        Some(PathBuf::from("Stitch1714280447.png.json")),
    );
    assert_eq!(
        swap_position(Path::new("./example.png.json")),
        Some(PathBuf::from("./example.png.json")),
    );
}




#[test]
fn test_get_image_file_path() {
    assert_eq!(
        get_new_image_file_path_by_swap_position(Path::new("IMG_0743.PNG(1).json")),
        PathBuf::from("IMG_0743(1).PNG")
    );

    assert_eq!(
        get_new_image_file_path_by_swap_position(Path::new("IMG_0777.png.json")),
        PathBuf::from("IMG_0777.png")
    );

    assert_eq!(
        get_new_image_file_path_by_swap_position(Path::new("/home/IMG_0743.mp4(2).json")),
        PathBuf::from("/home/IMG_0743(2).mp4")
    );

    assert_eq!(
        get_new_image_file_path_by_swap_position(Path::new("./test/IMG_0743.PNG(2).json")),
        PathBuf::from("./test/IMG_0743(2).PNG")
    );

    assert_eq!(
        get_new_image_file_path_by_swap_position(Path::new("IMG_0894.JPG(1).json")),
        PathBuf::from("IMG_0894(1).JPG")
    );
    
    assert_eq!(
        get_new_image_file_path_by_swap_position(Path::new("/home/fedora/test/ipad_google_photos_2014-2017/takeout-20240625T042623Z-001/Takeout/Google Photos/Photos from 2015/IMG_0894.JPG(1).json")),
        PathBuf::from("/home/fedora/test/ipad_google_photos_2014-2017/takeout-20240625T042623Z-001/Takeout/Google Photos/Photos from 2015/IMG_0894(1).JPG")
    );

}