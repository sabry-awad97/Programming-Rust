use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from("nonexistent.txt");
    let file = File::open(&file_path).map_err(|e| MyError::from((e, file_path.clone())))?;
    Ok(())
}

#[derive(Debug)]
struct MyError {
    source: Option<Box<dyn Error>>,
    path: Option<PathBuf>,
}

impl MyError {
    fn from((error, path): (io::Error, PathBuf)) -> MyError {
        MyError {
            source: Some(Box::new(error)),
            path: Some(path),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error opening file")?;
        if let Some(path) = &self.path {
            write!(f, ": {:?}", path)?;
        }
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}
