use std::io;
use std::path::Path;

use {Vfs, FileContents, Error};

pub fn racer_load_file(vfs: &Vfs, path: &Path) -> io::Result<String> {
    match vfs.0.load_file(path) {
        Ok(FileContents::Text(t)) => Ok(t),
        Ok(FileContents::Binary(_)) => Err(
            io::Error::new(io::ErrorKind::Other, Error::BadFileKind),
        ),
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
}

