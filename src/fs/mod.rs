use crate::{Error, Result};

pub fn list_files(path: &str) -> Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    if files.is_empty() {
        return Err(Error::CantListEmptyFolder);
    }
    Ok(files)
}

// 1tm -> 10x-test-module-01
// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    // use super::*;

    // 1tf -> 10x-test-fn-with-comment-01
    #[test]
    fn test_name() -> Result<()> {
        // -- Setup & Fixtures

        // -- Exec

        // -- Check

        Ok(())
    }
}

// endregion: --- Tests
