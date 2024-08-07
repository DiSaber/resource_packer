use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::utils::clean_path;

pub fn pack_single() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_name = exe_path.file_name().unwrap();

    let resources = read_dir(Path::new("./"), exe_name);

    fs::write("./resources.pck", bincode::serialize(&resources).unwrap()).unwrap();
}

fn read_dir(dir: &Path, exe_name: &OsStr) -> HashMap<PathBuf, Vec<u8>> {
    let mut resources = HashMap::new();

    for entry in fs::read_dir(dir).unwrap() {
        let Ok(entry) = entry else {
            continue;
        };

        if entry.file_name() == exe_name {
            continue;
        }

        if let Some(extension) = entry.path().extension() {
            if extension == "pck" {
                continue;
            }
        }

        if entry.path().is_dir() {
            resources.extend(read_dir(&entry.path(), exe_name))
        } else {
            let cleaned_path = clean_path(&entry.path());
            resources.insert(cleaned_path.clone(), fs::read(entry.path()).unwrap());
            println!("Packed {}", cleaned_path.to_string_lossy());
        }
    }

    resources
}
