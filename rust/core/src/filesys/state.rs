use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use bytesize::ByteSize;
use chrono::{DateTime, Utc};

use crate::filesys::walker::MetadataWithSize;
#[derive(Debug, Clone)]
pub struct DirectoryStat {
    pub total_size: u64,
    pub file_count: u64,
    pub largest_file_size: u64,
    pub path: PathBuf,
    pub latest_created: Option<DateTime<Utc>>,
    pub latest_accessed: Option<DateTime<Utc>>,
    pub latest_modified: Option<DateTime<Utc>>,
}
impl Display for DirectoryStat{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "total_size:{};file_count:{};largest_file_size:{};path={}",
               ByteSize(self.total_size), self.file_count, ByteSize(self.largest_file_size), self.path.display())
    }
}

impl DirectoryStat {
    pub fn display(& self){
        println!("DirectoryStat# total_size:{};file_count:{};largest_file_size:{};path={};latest_created:{:?}",
                 ByteSize(self.total_size), self.file_count, ByteSize(self.largest_file_size), self.path.display(),self.latest_created.unwrap())
    }
    pub fn from_metadata(path: PathBuf, metadata: &MetadataWithSize) -> DirectoryStat {
        let file_count = if metadata.is_dir { 0 } else { 1 };
        let total_size = metadata.size;

        DirectoryStat {
            total_size,
            file_count,
            largest_file_size: total_size,
            path,
            latest_created: metadata.metadata.created().map(|f| f.into()).ok(),
            latest_accessed: metadata.metadata.accessed().map(|f| f.into()).ok(),
            latest_modified: metadata.metadata.modified().map(|f| f.into()).ok(),
        }
    }

    pub fn merge(&mut self, other: &DirectoryStat) {
        self.total_size += other.total_size;
        self.file_count += other.file_count;
        if other.largest_file_size > self.largest_file_size {
            self.largest_file_size = other.largest_file_size;
        }
        if let Some(created) = other.latest_created {
            self.update_latest_created(created);
        }
        if let Some(accessed) = other.latest_accessed {
            self.update_latest_accessed(accessed);
        }
        if let Some(modified) = other.latest_modified {
            self.update_latest_modified(modified);
        }
    }

    // Please oh god tell me this can be generalized somehow.
    pub fn update_latest_created(&mut self, created: DateTime<Utc>) {
        match self.latest_created {
            None => {
                self.latest_created.replace(created);
            }
            Some(dt) if dt < created => {
                self.latest_created.replace(created);
            }
            _ => {}
        }
    }

    pub fn update_latest_accessed(&mut self, accessed: DateTime<Utc>) {
        match self.latest_accessed {
            None => {
                self.latest_accessed.replace(accessed);
            }
            Some(dt) if dt < accessed => {
                self.latest_accessed.replace(accessed);
            }
            _ => {}
        }
    }

    pub fn update_latest_modified(&mut self, modified: DateTime<Utc>) {
        match self.latest_modified {
            None => {
                self.latest_modified.replace(modified);
            }
            Some(dt) if dt < modified => {
                self.latest_modified.replace(modified);
            }
            _ => {}
        }
    }
}

pub struct WalkState {
    current: Option<DirectoryStat>,
    depth: Option<usize>,
}

impl WalkState {
    pub fn new( depth: Option<usize>) -> WalkState {
        WalkState {
            current: None,
            depth,
        }
    }

    fn is_equivalent_path(root: &PathBuf, target: &PathBuf, depth: Option<usize>) -> bool {
        // Are these two directory paths the same, or given a depth are the first N
        // components the same?
        match depth {
            None => root == target,
            Some(depth) => root
                .components()
                .take(depth)
                .eq(target.components().take(depth)),
        }
    }

    pub fn add_path(&mut self, path: PathBuf, metadata: &MetadataWithSize) {
        // println!("{} - {}", path.display(), path.is_dir());
        match &mut self.current {
            None => {
                self.current = Some(DirectoryStat::from_metadata(path, metadata));
            }
            Some(stat) if WalkState::is_equivalent_path(&stat.path, &path, self.depth) => {
                // Same directory, update in place
                if !metadata.is_dir {
                    stat.total_size += metadata.size;
                    stat.file_count += 1;
                    if metadata.size > stat.largest_file_size {
                        stat.largest_file_size = metadata.size
                    }
                    if let Ok(created) = metadata.metadata.created() {
                        stat.update_latest_created(created.into());
                    }
                    if let Ok(accessed) = metadata.metadata.accessed() {
                        stat.update_latest_accessed(accessed.into());
                    }
                    if let Ok(modified) = metadata.metadata.modified() {
                        stat.update_latest_modified(modified.into());
                    }
                }
            }
            Some(stat) => {
                stat.display();
                self.current = Some(DirectoryStat::from_metadata(path, metadata));
            }
        }
    }
}

impl Drop for WalkState {
    fn drop(&mut self) {
        if let Some(stat) = &self.current {
            stat.display();
        }
    }
}
