pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // for early development

mod fs;

use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".");

    println!("{files:#?}");

    Ok(())
}
