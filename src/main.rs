use clap::Parser;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Component, Path, PathBuf},
};

// From: https://github.com/danreeves/path-clean
pub fn clean_path(path: &Path) -> PathBuf {
    let mut out = Vec::new();

    for comp in path.components() {
        match comp {
            Component::CurDir => (),
            Component::ParentDir => match out.last() {
                Some(Component::RootDir) => (),
                Some(Component::Normal(_)) => {
                    out.pop();
                }
                None
                | Some(Component::CurDir)
                | Some(Component::ParentDir)
                | Some(Component::Prefix(_)) => out.push(comp),
            },
            comp => out.push(comp),
        }
    }

    if !out.is_empty() {
        out.iter().collect()
    } else {
        PathBuf::from(".")
    }
}

fn read_dir(dir: &Path, exe_name: &OsStr) -> Vec<(PathBuf, u64)> {
    let mut resource_sizes = Vec::new();

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
            resource_sizes.append(&mut read_dir(&entry.path(), exe_name));
        } else {
            let cleaned_path = clean_path(&entry.path());
            resource_sizes.push((
                cleaned_path.clone(),
                fs::metadata(entry.path()).unwrap().len(),
            ));
        }
    }

    resource_sizes
}

fn main() {
    let max_file_size = Args::parse().max_file_size;

    let exe_path = std::env::current_exe().unwrap();
    let exe_name = exe_path.file_name().unwrap();

    let resource_sizes = read_dir(Path::new("./"), exe_name);

    let _ = fs::remove_dir_all("./resources");
    fs::create_dir("./resources").unwrap();

    let mut index_file: HashMap<PathBuf, u64> = HashMap::new();
    let mut current_file_size = 0_u64;
    let mut current_resource_file: HashMap<PathBuf, Vec<u8>> = HashMap::new();
    let mut current_file_index = 0_u64;

    for (i, (path, file_size)) in resource_sizes.iter().enumerate() {
        index_file.insert(path.to_path_buf(), current_file_index);
        current_resource_file.insert(
            path.to_path_buf(),
            fs::read(Path::new("./").join(&path)).unwrap(),
        );
        current_file_size += file_size;

        println!(
            "Packed {} in resource_{current_file_index}.pck",
            path.to_string_lossy()
        );

        // Write the resources if the file size exceeds the max file size or when the last iteration of the loop is reached
        if current_file_size > max_file_size || i == resource_sizes.len() - 1 {
            fs::write(
                format!("./resources/resource_{current_file_index}.pck"),
                bincode::serialize(&current_resource_file).unwrap(),
            )
            .unwrap();
            current_file_size = 0;
            current_resource_file.clear();
            current_file_index += 1;
        }
    }

    fs::write(
        format!("./resources/resource_index.pck"),
        bincode::serialize(&index_file).unwrap(),
    )
    .unwrap();
}

#[derive(Parser, Debug)]
struct Args {
    /// The file size threshold, in bytes, for writing resource files. Defaults to 1MB
    #[arg(short, long, default_value_t = 1_000_000)]
    max_file_size: u64,
}
