use std::fs;
use std::io;
use std::path::Path;

pub fn move_files_to_new_folder(dir_location: &str) -> io::Result<()> {
    // Create a new Path from the directory location
    let dir_path = Path::new(dir_location);

    // Create a new directory in the specified location
    let new_dir = dir_path.join("new_folder");
    fs::create_dir_all(&new_dir)?;

    // Iterate over the entries in the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        // Check if the entry is a file (not a directory)
        if path.is_file() {
            // Move the file to the new directory
            let new_path = new_dir.join(path.file_name().unwrap());
            fs::rename(path, new_path)?;
        }
    }

    Ok(())
}
