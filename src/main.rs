mod error;
mod fs;

use self::error::Result;
use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".");

    println!("{files:#?}");

    Ok(())
}
