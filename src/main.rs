use std::{collections::HashMap, ffi::OsStr, fs, path::Path};

fn pack_dir(exe_name: &OsStr, dir: &Path, resource_storage: &mut HashMap<String, Vec<u8>>) {
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
            pack_dir(exe_name, entry.path().as_path(), resource_storage);
        } else {
            if resource_storage
                .insert(
                    entry.file_name().to_string_lossy().to_string(),
                    fs::read(entry.path()).unwrap(),
                )
                .is_some()
            {
                println!(
                    "Warning, duplicate name! {}",
                    entry.path().to_string_lossy()
                );
            } else {
                println!("Packed {}", entry.path().to_string_lossy());
            }
        }
    }
}

fn main() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_name = exe_path.file_name().unwrap();

    let mut resource_storage: HashMap<String, Vec<u8>> = HashMap::new();

    pack_dir(exe_name, "./".as_ref(), &mut resource_storage);

    fs::write(
        "./resources.pck",
        bincode::serialize(&resource_storage).unwrap(),
    )
    .unwrap();
}
