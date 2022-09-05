use std::path::PathBuf;

use chrono::{DateTime, Local};
use tui::{
    layout::Rect,
    text::{Span, Spans},
};

use crate::core::ToSpans;

#[derive(Clone, Debug)]
pub struct FileItem {
    name: String,
    path: PathBuf,
    last_modification: DateTime<Local>,
    icon: String,

    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
    pub accessed: DateTime<Local>,
    pub size: u64,
    pub mode: u32,
    pub inode: u64,
    pub nlink: u64,
    pub username: String,
    pub groupname: String,
    pub blocksize: u64,
    pub blocks: u64,
}

impl FileItem {
    pub fn new(
        name: String,
        path: PathBuf,
        last_modification: DateTime<Local>,
        icon: String,

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
    ) -> Self {
        FileItem {
            name,
            path,
            last_modification,
            icon,

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
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn is_visible(&self) -> bool {
        self.name.starts_with('.')
    }
}

impl ToSpans for FileItem {
    fn to_spans(&self, _area: Rect, show_icons: bool) -> Spans {
        /*
        if show_icons {
            Spans::from(vec![
                Span::from("  "),
                Span::from(self.icon.clone()),
                Span::from("  "),
                Span::from(
                    self.last_modification
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                ),
                Span::from("  "),
                Span::from(self.name.clone()),
            ])
        } else {
            Spans::from(vec![
                Span::from("  "),
                Span::from(
                    self.last_modification
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(),
                ),
                Span::from("  "),
                Span::from(self.name.clone()),
            ])
        }
        */
        Spans::from(vec![Span::from("   "), Span::from(self.name.clone())])
    }
}
