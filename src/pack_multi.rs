use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use crate::utils::clean_path;

pub fn pack_multi() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_name = exe_path.file_name().unwrap();

    let resource_paths = read_dir(Path::new("./"), exe_name);

    let _ = fs::remove_dir_all("./resources");
    fs::create_dir("./resources").unwrap();

    let mut index_file: HashMap<PathBuf, u64> = HashMap::new();

    for (file_index, path) in resource_paths.iter().enumerate() {
        index_file.insert(path.to_path_buf(), file_index as u64);

        fs::copy(
            Path::new("./").join(&path),
            format!("./resources/resource_{file_index}.pck"),
        )
        .unwrap();

        println!(
            "Packed {} in resource_{file_index}.pck",
            path.to_string_lossy()
        );
    }

    fs::write(
        "./resources/resource_index.pck",
        bincode::serialize(&index_file).unwrap(),
    )
    .unwrap();
}

fn read_dir(dir: &Path, exe_name: &OsStr) -> Vec<PathBuf> {
    let mut resource_paths = Vec::new();

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
            resource_paths.extend(read_dir(&entry.path(), exe_name))
        } else {
            let cleaned_path = clean_path(&entry.path());
            resource_paths.push(cleaned_path);
        }
    }

    resource_paths
}
