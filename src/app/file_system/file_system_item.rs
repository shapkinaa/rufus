use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use tui::{layout::Rect, text::Spans};

use crate::core::ToSpans;

use super::{dir_item::DirItem, file_item::FileItem, symlink_item::SymlinkItem};

#[derive(Clone, Debug)]
pub enum FileSystemItem {
    Directory(DirItem),
    File(FileItem),
    Symlink(SymlinkItem),
    Unknown,
}

impl FileSystemItem {
    pub fn get_path(&self) -> PathBuf {
        match self {
            FileSystemItem::Directory(dir) => dir.get_path(),
            FileSystemItem::File(file) => file.get_path(),
            FileSystemItem::Symlink(symlink) => symlink.get_path(),
            FileSystemItem::Unknown => PathBuf::new(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            FileSystemItem::Directory(dir) => dir.get_name(),
            FileSystemItem::File(file) => file.get_name(),
            FileSystemItem::Symlink(symlink) => symlink.get_name(),
            FileSystemItem::Unknown => "".to_string(),
        }
    }

    pub fn is_symlink(&self) -> bool {
        match self {
            FileSystemItem::Directory(_) => false,
            FileSystemItem::File(_) => false,
            FileSystemItem::Symlink(_) => true,
            FileSystemItem::Unknown => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileSystemItem::Directory(_) => false,
            FileSystemItem::File(_) => true,
            FileSystemItem::Symlink(_) => false,
            FileSystemItem::Unknown => false,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            FileSystemItem::Directory(_) => true,
            FileSystemItem::File(_) => false,
            FileSystemItem::Symlink(_) => false,
            FileSystemItem::Unknown => false,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            FileSystemItem::Directory(dir) => dir.is_visible(),
            FileSystemItem::File(file) => file.is_visible(),
            FileSystemItem::Symlink(_) => true,
            FileSystemItem::Unknown => false,
        }
    }

    //        created: DateTime<Local>,
    pub fn get_created(&self) -> DateTime<Local> {
        match self {
            FileSystemItem::Directory(dir) => dir.created,
            FileSystemItem::File(file) => file.created,
            FileSystemItem::Symlink(symlink) => symlink.created,
            FileSystemItem::Unknown => Local
                .datetime_from_str("1980-01-01 00:00:01", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        }
    }
    //        modified: DateTime<Local>,
    pub fn get_modified(&self) -> DateTime<Local> {
        match self {
            FileSystemItem::Directory(dir) => dir.modified,
            FileSystemItem::File(file) => file.modified,
            FileSystemItem::Symlink(symlink) => symlink.modified,
            FileSystemItem::Unknown => Local
                .datetime_from_str("1980-01-01 00:00:01", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        }
    }
    //        accessed: DateTime<Local>,
    pub fn get_accessed(&self) -> DateTime<Local> {
        match self {
            FileSystemItem::Directory(dir) => dir.accessed,
            FileSystemItem::File(file) => file.accessed,
            FileSystemItem::Symlink(symlink) => symlink.accessed,
            FileSystemItem::Unknown => Local
                .datetime_from_str("1980-01-01 00:00:01", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        }
    }
    //        size: u64,
    pub fn get_size(&self) -> u64 {
        match self {
            FileSystemItem::Directory(dir) => dir.size,
            FileSystemItem::File(file) => file.size,
            FileSystemItem::Symlink(symlink) => symlink.size,
            FileSystemItem::Unknown => 0,
        }
    }
    //        mode: u32,
    pub fn get_mode(&self) -> u32 {
        match self {
            FileSystemItem::Directory(dir) => dir.mode,
            FileSystemItem::File(file) => file.mode,
            FileSystemItem::Symlink(symlink) => symlink.mode,
            FileSystemItem::Unknown => 0,
        }
    }
    //        inode: u64,
    pub fn get_inode(&self) -> u64 {
        match self {
            FileSystemItem::Directory(dir) => dir.inode,
            FileSystemItem::File(file) => file.inode,
            FileSystemItem::Symlink(symlink) => symlink.inode,
            FileSystemItem::Unknown => 0,
        }
    }
    //        nlink: u64,
    pub fn get_nlink(&self) -> u64 {
        match self {
            FileSystemItem::Directory(dir) => dir.nlink,
            FileSystemItem::File(file) => file.nlink,
            FileSystemItem::Symlink(symlink) => symlink.nlink,
            FileSystemItem::Unknown => 0,
        }
    }
    //        username: String,
    pub fn get_username(&self) -> String {
        match self {
            FileSystemItem::Directory(dir) => dir.username.clone(),
            FileSystemItem::File(file) => file.username.clone(),
            FileSystemItem::Symlink(symlink) => symlink.username.clone(),
            FileSystemItem::Unknown => "".to_string(),
        }
    }
    //        groupname: String,
    pub fn get_groupname(&self) -> String {
        match self {
            FileSystemItem::Directory(dir) => dir.groupname.clone(),
            FileSystemItem::File(file) => file.groupname.clone(),
            FileSystemItem::Symlink(symlink) => symlink.groupname.clone(),
            FileSystemItem::Unknown => "".to_string(),
        }
    }
    //        blocksize: u64,
    pub fn get_blocksize(&self) -> u64 {
        match self {
            FileSystemItem::Directory(dir) => dir.blocksize,
            FileSystemItem::File(file) => file.blocksize,
            FileSystemItem::Symlink(symlink) => symlink.blocksize,
            FileSystemItem::Unknown => 0,
        }
    }
    //        blocks: u64,
    pub fn get_blocks(&self) -> u64 {
        match self {
            FileSystemItem::Directory(dir) => dir.blocks,
            FileSystemItem::File(file) => file.blocks,
            FileSystemItem::Symlink(symlink) => symlink.blocks,
            FileSystemItem::Unknown => 0,
        }
    }
}

impl ToSpans for FileSystemItem {
    fn to_spans(&self, area: Rect, show_icons: bool) -> Spans {
        match self {
            FileSystemItem::Directory(dir) => dir.to_spans(area, show_icons),
            FileSystemItem::File(file) => file.to_spans(area, show_icons),
            FileSystemItem::Symlink(symlink) => symlink.to_spans(area, show_icons),
            FileSystemItem::Unknown => Spans::default(),
        }
    }
}
