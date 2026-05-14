use std::fs::File;
use std::io;
use std::path::Path;

fn classify_open_error(path: &Path) -> String {
    if path.is_dir() {
        return "is a directory".to_string();
    }

    match File::open(path) {
        Ok(_) => "opened".to_string(),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => "not found".to_string(),
            io::ErrorKind::PermissionDenied => "permission denied".to_string(),
            other => format!("other error: {other:?}"),
        },
    }
}

fn main() {
    for path in ["START_HERE.md", "missing-file.txt", "."] {
        println!("{path}: {}", classify_open_error(Path::new(path)));
    }
}
