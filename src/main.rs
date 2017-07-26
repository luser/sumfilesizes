extern crate number_prefix;
extern crate rayon;
extern crate walkdir;

use number_prefix::{binary_prefix, Standalone, Prefixed};
use rayon::prelude::*;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;


fn total_size<P: AsRef<Path>>(path: P) -> u64 {
    let path = path.as_ref();
    let m = match fs::metadata(path) {
        Err(e) => {
            println!("Error reading file size for {:?}: {}", path, e);
            return 0;
        }
        Ok(m) => m,
    };
    if m.is_file() {
        m.len()
    } else if m.is_dir() {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok()
                        .and_then(|f| {
                            // Only look at files
                            if f.file_type().is_file() {
                                // Just grab the file size out.
                                f.metadata().map(|m| m.len()).ok()
                            } else {
                                None
                            }
                        }))
            .sum()
    } else {
        // not a regular file or directory, just skip it?
        0
    }
}

fn lines<R: BufRead>(reader: R) -> io::Result<Vec<String>> {
    reader.lines().collect()
}

fn main() {
    let stdin = io::stdin();
    let files: Vec<_> = if env::args().count() > 1 {
        env::args().skip(1).collect()
    } else {
        // If no args are given, read filenames from stdin.
        lines(stdin.lock()).expect("Error reading from stdin")
    };
    let size: u64 = files.into_par_iter()
        .map(total_size)
        .sum();
    let (val, suffix) = match binary_prefix(size as f64) {
        Standalone(bytes) => (bytes.to_string(), "bytes".to_string()),
        Prefixed(prefix, n) => (format!("{:.0}", n), format!("{}B", prefix)),
    };
    println!("Total size: {} {}", val, suffix)
}
