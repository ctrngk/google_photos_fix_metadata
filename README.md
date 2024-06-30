# Google Photos and iPhone Photos Metadata Fixer 

## Overview

This Rust project addresses common metadata issues in Google Photos. Sometimes, Google Photos does not respect the original capture time or `photoTakenTime`, resulting in incorrect date ordering and photos improperly appearing as if they were taken "today". After using the Google Takeout service, you can obtain JSON files for each photo, which contain important time information.

This tool fixes the following issues:

1. **Google Photos Metadata Issues**:
    - Adds `DateTimeOriginal` and `CreateDate` metadata to photos if missing, using the information extracted from the corresponding JSON files.

2. **iPhone Photos Metadata Issues**:
    - iPhone photos transferred to a PC via USB may lack `DateTimeOriginal`, `CreateDate`, or `photoTakenTime` metadata. These photos only have modification times, which are unreliable and easily lost if the photo is modified, copied, or renamed.
    - This tool adds `DateTimeOriginal` and `CreateDate` metadata to these photos using their modification times if no other time information is available.

## Prerequisites
1. exiftool command line tool installed beforehand.
2. Tested on Linux environment (macOS should work, Windows is not guaranteed).

## Usage

```
./google_photos_fix_metadata --help
Patches metadata of photos based on JSON files

Usage: google_photos_fix_metadata [OPTIONS]

Options:
--src-google-photos <src-google-photos>   Source directories for Google Photos
--src-iphone-photos <src-iphone-photos>   Source directories for iPhone Photos
-h, --help                                Print help
-V, --version                             Print version
```

### Example Commands

```bash
# The original source files are edited, so please make a backup before using this tool.

./google_photos_fix_metadata --src-google-photos /home/fedora/test/iphone-google-photos-20240416-20240624
./google_photos_fix_metadata --src-google-photos /home/fedora/test/ipad_google_photos_2014-2017/
./google_photos_fix_metadata --src-iphone-photos /home/fedora/test/iphone_direct_photos_20240415_from_about20240117
```

### Output

- The results are saved to the current output directory relative to the `google_photos_fix_metadata` command line directory.

### Important Notes

- This tool modifies the original source files. Please ensure you have backups of your photos before using it.
- When copying the output directory, make sure to preserve the modification time. For example, use the `-p` option in the `cp` command. This is important because Google Photos does not respect the DateCreate and DateTimeOriginal metadata for GIFs, but it does use the modification time.

## Handling Large Photos

If you encounter issues with Google Photos not uploading your photos because they are too large, note that Google Photos suggests images be less than 50 MB in size. Requests to create media items in an album that exceed this limit will fail. Google Photos also compresses photos to save space and resizes photos larger than 16 MP to 16 MP.

I had a very large screenshot created by scrolling, and the upload failed. I used the following command line to resize it to 16 MP:

```bash
convert input_image.jpg -resize 1000x16000 output_image.jpg
```

For 16 MP, this translates to roughly 4000 x 4000 pixels for a 1:1 aspect ratio. If your photos have a different aspect ratio (e.g., 4:3, 16:9), maintain the ratio while keeping the maximum dimension around 4000 pixels. For example, for a 4:3 photo, you could use:

```bash
-resize 3200x2400
```

## Features

- **Filename Preservation**: The tool retains the original filenames whenever possible. If there are duplicate filenames, a random 6-character suffix is added to one of them (e.g., `IMG_0328.JPG` becomes `IMG_0328-IxSMqO.JPG`).
- **Extension Consistency**: The tool preserves the original file extension even if it conflicts with the actual media type (e.g., a file named `IMG_0328.JPG` might actually be a PNG, but the extension remains JPG).
- **Sync Modification Time From DateCreate and DateTimeOriginal**: Google Photos ignores the DateTimeOriginal and CreateDate metadata of GIFs but respects the modification time.
- **Modification Time Preservation**: The tool respects and restores the original modification times of the files as much as possible.

You can find all the processed photos in the output directory relative to where you run the `google_photos_fix_metadata` command.
