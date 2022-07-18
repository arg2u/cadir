//! # Cadir
//!
//! `cadir` is a tool to create one or many directories.
//! It provides recursive and multithreading modes.

use std::fs;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

/// # Creates directories
///
/// If you have a directory path "/a/b/c" and "b"
/// doesn't exist, use `recursively = true`
/// 
/// If you need to create a lof of dirs, set `mthread = true`
///
/// # Examples
/// ```
/// use std::path::PathBuf;
///
/// let path:PathBuf = PathBuf::from("e");
/// let path2:PathBuf = PathBuf::from("b");
/// let vec = vec![path, path2];
///
/// cadir::run(vec, false, false);
/// ```
pub fn run(directories: Vec<PathBuf>, recursively: bool, mthread: bool) {
    if mthread {
        let handles = multi_create(directories, recursively);
        for handle in handles {
            handle.join().unwrap();
        }
    } else {
        sync_create(directories, recursively);
    }
}

/// Create directories one by one in the main thread
/// 
/// If you have dir path "/a/b/c" and "b"
/// doesn't exist, use `recursively = true`
/// 
/// # Examples
/// ```
/// use std::path::PathBuf;
///
/// let path:PathBuf = PathBuf::from("e");
/// let path2:PathBuf = PathBuf::from("b");
/// let vec = vec![path, path2];
///
/// cadir::sync_create(vec, false);
/// ```
pub fn sync_create(directories: Vec<PathBuf>, recursively: bool) {
    validate(&directories, recursively);
    for dir in directories {
        create(&dir, recursively);
    }
}

/// Create directories in the multithreading mode
/// 
/// If you have dir path "/a/b/c" and "b"
/// doesn't exist, use `recursively = true`
/// 
/// # Examples
/// ```
/// use std::path::PathBuf;
///
/// let path:PathBuf = PathBuf::from("e");
/// let path2:PathBuf = PathBuf::from("b");
/// let vec = vec![path, path2];
///
/// cadir::sync_create(vec, false);
/// ```
pub fn multi_create(directories: Vec<PathBuf>, recursively: bool) -> Vec<JoinHandle<()>> {
    validate(&directories, recursively);
    let mut handles = vec![];
    for dir in directories {
        let handle = thread::spawn(move || {
            create(&dir, recursively);
        });
        handles.push(handle);
    }
    handles
}

fn validate(opt_dirs: &Vec<PathBuf>, recursively: bool) {
    let directories = opt_dirs.clone();
    let mut valid: bool = true;
    for mut dir in directories {
        if !dir.exists() {
            dir.pop();
            if !recursively && !dir.exists() && dir.iter().count() > 1 {
                println!("Use -r flag for the create dir recurvively.");
                valid = false;
                break;
            }
        } else {
            println!("This directory is already exists.");
            valid = false;
            break;
        }
    }
    if !valid {
        std::process::exit(0);
    }
}

fn create(dir: &PathBuf, recursively: bool) {
    if recursively {
        fs::create_dir_all(&dir).unwrap();
    } else {
        fs::create_dir(&dir).unwrap();
    }
}
