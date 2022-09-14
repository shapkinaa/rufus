#[cfg(unix)]
use std::os::unix::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

#[cfg(windows)]
use std::os::windows::fs;

use std::{
    fs::{read_link, DirEntry, Metadata},
    io,
    path::{Path, PathBuf},
    time::SystemTime,
};
use users::{get_group_by_gid, get_user_by_uid};

use chrono::{DateTime, Local};

use crate::app::config::icon_cfg::IconsConfig;

use super::{
    dir_item::DirItem, file_item::FileItem, file_system_item::FileSystemItem,
    symlink_item::SymlinkItem,
};

struct FileSystemItemProps {
    name: String,
    path: PathBuf,
    created: DateTime<Local>,
    modified: DateTime<Local>,
    accessed: DateTime<Local>,
    size: u64,
    mode: u32,
    inode: u64,
    nlink: u64,
    username: String,
    groupname: String,
    blocksize: u64,
    blocks: u64,
}

#[cfg(unix)]
pub fn create_link<TPath: AsRef<Path>>(symlink_path: TPath, item_path: TPath) -> io::Result<()> {
    let symlink_path = expand_if_contains_tilde(symlink_path).unwrap();
    fs::symlink(item_path, symlink_path)
}

#[cfg(windows)]
pub fn create_link<TPath: AsRef<Path>>(symlink_path: TPath, item_path: TPath) -> io::Result<()> {
    let symlink_path = expand_if_contains_tilde(symlink_path).unwrap();
    if item_path.is_dir() {
        fs::symlink_dir(item_path, symlink_path)
    } else {
        fs::symlink_file(item_path, symlink_path)
    }
}

//From: https://stackoverflow.com/questions/54267608/expand-tilde-in-rust-path-idiomatically
pub fn expand_if_contains_tilde<TPath: AsRef<Path>>(input: TPath) -> Option<PathBuf> {
    let path = input.as_ref();
    if path.starts_with("~") == false {
        return Some(path.to_path_buf());
    }
    if path == Path::new("~") {
        return dirs::home_dir();
    }

    dirs::home_dir().map(|mut home_path| {
        if home_path == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            path.strip_prefix("~").unwrap().to_path_buf()
        } else {
            home_path.push(path.strip_prefix("~/").unwrap());
            home_path
        }
    })
}

pub fn map_dir_entry_to_file_system_item(
    dir_entry: DirEntry,
    icons: &IconsConfig,
) -> FileSystemItem {
    if let Ok(metadata) = dir_entry.metadata() {
        // let (name, path, modified) = get_file_system_item_props(dir_entry.clone(), &metadata);
        let file_system_item_props = get_file_system_item_props_struct(dir_entry, &metadata);

        let file_type = metadata.file_type();
        if file_type.is_file() {
            let file_extensions = file_system_item_props.name.split('.').last().unwrap_or("");
            return FileSystemItem::File(FileItem::new(
                file_system_item_props.name.to_string(),
                file_system_item_props.path,
                file_system_item_props.modified,
                icons.get_file_icon(file_extensions.to_string()),
                file_system_item_props.created,
                file_system_item_props.modified,
                file_system_item_props.accessed,
                file_system_item_props.size,
                file_system_item_props.mode,
                file_system_item_props.inode,
                file_system_item_props.nlink,
                file_system_item_props.username,
                file_system_item_props.groupname,
                file_system_item_props.blocksize,
                file_system_item_props.blocks,
            ));
        }

        if file_type.is_dir() {
            return FileSystemItem::Directory(DirItem::new(
                file_system_item_props.name.to_string(),
                file_system_item_props.path.clone(),
                file_system_item_props.modified,
                icons.get_dir_icon(file_system_item_props.name),
                file_system_item_props
                    .path
                    .read_dir()
                    .map(|mut i| i.next().is_none())
                    .unwrap_or(false),
                file_system_item_props.created,
                file_system_item_props.modified,
                file_system_item_props.accessed,
                file_system_item_props.size,
                file_system_item_props.mode,
                file_system_item_props.inode,
                file_system_item_props.nlink,
                file_system_item_props.username,
                file_system_item_props.groupname,
                file_system_item_props.blocksize,
                file_system_item_props.blocks,
            ));
        }

        if file_type.is_symlink() {
            let file_extensions = file_system_item_props.name.split('.').last().unwrap_or("");
            match read_link(file_system_item_props.path.clone()) {
                Ok(target) => {
                    return FileSystemItem::Symlink(SymlinkItem::new(
                        file_system_item_props.name.to_string(),
                        file_system_item_props.path,
                        target.clone(),
                        file_system_item_props.modified,
                        if target.is_file() {
                            icons.get_file_icon(file_extensions.to_string())
                        } else {
                            icons.get_dir_icon(file_system_item_props.name)
                        },
                        file_system_item_props.created,
                        file_system_item_props.modified,
                        file_system_item_props.accessed,
                        file_system_item_props.size,
                        file_system_item_props.mode,
                        file_system_item_props.inode,
                        file_system_item_props.nlink,
                        file_system_item_props.username,
                        file_system_item_props.groupname,
                        file_system_item_props.blocksize,
                        file_system_item_props.blocks,
                    ))
                }
                Err(_) => {
                    return FileSystemItem::Symlink(SymlinkItem::new(
                        file_system_item_props.name.to_string(),
                        file_system_item_props.path.clone(),
                        file_system_item_props.path,
                        file_system_item_props.modified,
                        icons.get_file_icon(file_extensions.to_string()),
                        file_system_item_props.created,
                        file_system_item_props.modified,
                        file_system_item_props.accessed,
                        file_system_item_props.size,
                        file_system_item_props.mode,
                        file_system_item_props.inode,
                        file_system_item_props.nlink,
                        file_system_item_props.username,
                        file_system_item_props.groupname,
                        file_system_item_props.blocksize,
                        file_system_item_props.blocks,
                    ))
                }
            }
        }

        FileSystemItem::Unknown
    } else {
        FileSystemItem::Unknown
    }
}

fn get_file_system_item_props(
    dir_entry: DirEntry,
    metadata: &Metadata,
) -> (String, PathBuf, DateTime<Local>) {
    let modified: DateTime<Local> = if let Ok(last_modified) = metadata.modified() {
        last_modified.into()
    } else {
        SystemTime::now().into()
    };

    let entry_name = dir_entry.file_name();
    let name = if let Some(name) = entry_name.to_str() {
        name
    } else {
        ""
    };
    let path_buffer = dir_entry.path();

    (name.to_string(), path_buffer, modified)
}

fn get_file_system_item_props_struct(
    dir_entry: DirEntry,
    metadata: &Metadata,
) -> FileSystemItemProps {
    let modified: DateTime<Local> = if let Ok(last_modified) = metadata.modified() {
        last_modified.into()
    } else {
        SystemTime::now().into()
    };

    let entry_name = dir_entry.file_name();
    let name = if let Some(name) = entry_name.to_str() {
        name
    } else {
        ""
    };
    let path_buffer = dir_entry.path();

    // (name.to_string(), path_buffer, modified)
    let size = metadata.len();
    let mode = metadata.permissions().mode();

    let inode = metadata.ino();
    let nlink = metadata.nlink();
    let username = get_user_by_uid(metadata.uid())
        .unwrap()
        .name()
        .to_string_lossy()
        .to_string();
    let groupname = get_group_by_gid(metadata.gid())
        .unwrap()
        .name()
        .to_string_lossy()
        .to_string();
    let blocksize = metadata.blksize();
    let blocks = metadata.blocks();

    let created: DateTime<Local> = if let Ok(last_modified) = metadata.created() {
        last_modified.into()
    } else {
        SystemTime::now().into()
    };

    let modified: DateTime<Local> = if let Ok(last_modified) = metadata.modified() {
        last_modified.into()
    } else {
        SystemTime::now().into()
    };

    let accessed: DateTime<Local> = if let Ok(last_modified) = metadata.accessed() {
        last_modified.into()
    } else {
        SystemTime::now().into()
    };

    FileSystemItemProps {
        name: name.to_string(),
        path: path_buffer,
        created,
        modified,
        accessed,
        size,
        mode,
        inode,
        nlink,
        username,
        groupname,
        blocksize,
        blocks,
    }

    ////////////////////////////////////////////
}
