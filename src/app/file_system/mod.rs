use std::{
    cmp,
    cmp::Reverse,
    fs::{self, File},
    path::Path,
};
use std::{io, path::PathBuf};

use self::{
    file_system_item::FileSystemItem,
    functions::{create_link, map_dir_entry_to_file_system_item},
};

use super::config::tab_config::SortEnum;
use super::config::Config;

pub mod dir_item;
pub mod file_item;
pub mod file_system_item;
pub mod functions;
pub mod symlink_item;

pub trait FileSystem {
    fn exist<TPath: AsRef<Path>>(&self, path: TPath) -> bool;
    fn get_dir_info<TPath: AsRef<Path>>(&self, path: TPath) -> Option<DirInfo>;
    fn list_dir<TPath: AsRef<Path>>(&self, path: TPath, big_config: &Config)
        -> Vec<FileSystemItem>;
    fn read_to_string<TPath: AsRef<Path>>(&self, path: TPath) -> Option<String>;
    fn delete_file<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()>;
    fn delete_dir<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()>;
    fn delete_empty_dir<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()>;
    fn rename_item<TPath: AsRef<Path>>(&mut self, source: TPath, target: TPath) -> io::Result<()>;
    fn create_symlink<TPath: AsRef<Path>>(
        &mut self,
        source: TPath,
        target: TPath,
    ) -> io::Result<()>;
    fn create_file<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<File>;
    fn create_dir<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()>;
    fn copy_file<TPath: AsRef<Path>>(&mut self, source: TPath, target: TPath) -> io::Result<u64>;
    fn copy_dir<TPath: AsRef<Path>>(&mut self, source: TPath, target: TPath) -> io::Result<u64>;
}

#[derive(Clone, Debug, Default)]
pub struct PhysicalFileSystem;

impl FileSystem for PhysicalFileSystem {
    fn get_dir_info<TPath: AsRef<Path>>(&self, path: TPath) -> Option<DirInfo> {
        DirInfo::new(&path)
    }

    fn list_dir<TPath: AsRef<Path>>(
        &self,
        path: TPath,
        big_config: &Config,
    ) -> Vec<FileSystemItem> {
        match fs::read_dir(path) {
            Ok(mut iter) => {
                let mut result = Vec::new();
                while let Some(load_result) = iter.next() {
                    if let Ok(dir_entry) = load_result {
                        result.push(map_dir_entry_to_file_system_item(
                            dir_entry,
                            &big_config.icons,
                        ));
                    }
                }
                if big_config.tab_config.directory_first {
                    match big_config.tab_config.sort_by_name {
                        SortEnum::ASC => {
                            result.sort_by(|one, two| match one.is_file().cmp(&two.is_file()) {
                                cmp::Ordering::Equal => one.get_name().cmp(&two.get_name()),
                                other => other,
                            });
                            return result;
                        }
                        SortEnum::DESC => {
                            result.sort_by_key(|w| (w.is_file(), std::cmp::Reverse(w.get_name())));
                            return result;
                        }
                        SortEnum::NONE => {}
                    }
                    match big_config.tab_config.sort_by_attr {
                        SortEnum::ASC => {
                            result.sort_by(|one, two| match one.is_file().cmp(&two.is_file()) {
                                cmp::Ordering::Equal => one.get_size().cmp(&two.get_size()),
                                other => other,
                            });
                        }
                        SortEnum::DESC => {
                            result.sort_by_key(|w| (w.is_file(), std::cmp::Reverse(w.get_size())));
                            return result;
                        }
                        SortEnum::NONE => {}
                    }
                    match big_config.tab_config.sort_by_date {
                        SortEnum::ASC => {
                            result.sort_by(|one, two| match one.is_file().cmp(&two.is_file()) {
                                cmp::Ordering::Equal => one.get_modified().cmp(&two.get_modified()),
                                other => other,
                            });
                        }
                        SortEnum::DESC => {
                            result.sort_by_key(|w| {
                                (w.is_file(), std::cmp::Reverse(w.get_modified()))
                            });
                            return result;
                        }
                        SortEnum::NONE => {}
                    }
                } else {
                    match big_config.tab_config.sort_by_date {
                        SortEnum::ASC => {
                            result.sort_by(|one, two| one.get_modified().cmp(&two.get_modified()));
                        }
                        SortEnum::DESC => {
                            result.sort_by_key(|w| std::cmp::Reverse(w.get_modified()));
                        }
                        SortEnum::NONE => {}
                    }
                    match big_config.tab_config.sort_by_name {
                        SortEnum::ASC => {
                            result.sort_by(|one, two| one.get_name().cmp(&two.get_name()));
                        }
                        SortEnum::DESC => {
                            result.sort_by_key(|w| std::cmp::Reverse(w.get_name()));
                        }
                        SortEnum::NONE => {}
                    }
                    match big_config.tab_config.sort_by_attr {
                        SortEnum::ASC => {
                            result.sort_by(|one, two| one.get_size().cmp(&two.get_size()));
                        }
                        SortEnum::DESC => {
                            result.sort_by_key(|w| std::cmp::Reverse(w.get_size()));
                        }
                        SortEnum::NONE => {}
                    }
                }
                result
            }
            Err(_) => Vec::new(),
        }
    }

    fn read_to_string<TPath: AsRef<Path>>(&self, path: TPath) -> Option<String> {
        match fs::read_to_string(path) {
            Ok(content) => return Some(content.clone()),
            Err(_) => None,
        }
    }

    fn delete_file<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn delete_dir<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    fn rename_item<TPath: AsRef<Path>>(&mut self, source: TPath, target: TPath) -> io::Result<()> {
        fs::rename(source, target)
    }

    fn create_symlink<TPath: AsRef<Path>>(
        &mut self,
        source: TPath,
        target: TPath,
    ) -> io::Result<()> {
        create_link(target, source)
    }

    fn create_file<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<File> {
        File::create(path)
    }

    fn create_dir<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()> {
        fs::create_dir(path)
    }

    fn delete_empty_dir<TPath: AsRef<Path>>(&mut self, path: TPath) -> io::Result<()> {
        fs::remove_dir(path)
    }

    fn copy_file<TPath: AsRef<Path>>(&mut self, source: TPath, target: TPath) -> io::Result<u64> {
        fs::copy(source, target)
    }

    fn copy_dir<TPath: AsRef<Path>>(&mut self, source: TPath, target: TPath) -> io::Result<u64> {
        fs::create_dir_all(target.as_ref())?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                self.copy_dir(entry.path(), target.as_ref().join(entry.file_name()))?;
            } else {
                self.copy_file(entry.path(), target.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(0)
    }

    fn exist<TPath: AsRef<Path>>(&self, path: TPath) -> bool {
        let path = path.as_ref();
        if path.is_dir() || path.is_file() {
            return path.exists();
        }
        return true;
    }
}

#[derive(Clone, Debug)]
pub struct DirInfo {
    pub name: String,
    pub path: PathBuf,
}

impl DirInfo {
    pub fn new<TPath: AsRef<Path>>(path: &TPath) -> Option<Self> {
        if let Ok(path_buffer) = fs::canonicalize(path) {
            let name = if let Some(file_name) = path_buffer.file_name() {
                file_name.to_str().unwrap_or("")
            } else {
                ""
            };
            let path = path_buffer.as_path().to_str().unwrap_or("");
            return Some(DirInfo {
                name: name.to_string(),
                path: PathBuf::from(path),
            });
        }
        None
    }
}
