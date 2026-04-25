use std::fs::{copy, create_dir_all, read_dir, DirEntry};
use std::io;
use std::path::Path;

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>, copy_if: impl Fn(&DirEntry) -> bool + Clone) -> io::Result<()> {
    create_dir_all(&dst)?;
    for entry in read_dir(src)? {
        let entry = entry?;
        
        if !copy_if(&entry) {
            continue;
        }
        
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()), copy_if.clone())?;
        } else {
            copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
