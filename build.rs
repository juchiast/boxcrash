/*
The MIT License (MIT)

Copyright (c) 2015 Adolfo Ochagavía

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/
use std::env;
use std::fs::{copy, create_dir_all, read_dir};
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let res_dir_source = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("resources/");
    let res_dir_target = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../resources/");

    //copies all resource files to "target/NAME/resources". Prints out any errors if failed.
    if let Err(io_error) = add_resources(&res_dir_source, &res_dir_target) {
        println!("OS Error: {}", io_error);
    }
}

///Recursively copy all files in dir given by source_path to dir given by target path
///WARNING! Overwrites files with same name
fn add_resources(source_path: &PathBuf, target_path: &PathBuf) -> io::Result<()> {
    match read_dir(source_path) {
        Ok(entry_iter) => {
            create_dir_all(target_path)?;
            for entry in entry_iter {
                let entry = entry?;
                let source_path = entry.path();
                let target_path = target_path.join(entry.file_name());
                add_resources(&source_path, &target_path)?;
            }
        }
        Err(_) => {
            copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}
