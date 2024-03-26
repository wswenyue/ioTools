
pub mod filesys;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::filesys::progress::WalkProgress;
    use crate::filesys::state::WalkState;
    use crate::filesys::walker::Walker;

    #[test]
    fn test_filesys_walker() {
        let root: PathBuf = PathBuf::from(home::home_dir().unwrap().join("temp/ioTools"));
        println!("Root:{:?}", root);
        let mut walk_state = WalkState::new( None);
        let mut walk_progress = WalkProgress::new(root.clone());
        for dir in &mut Walker::new(8, true, true, true, true).walk_dir(&root) {
            walk_progress.record_progress(&dir);
            if walk_progress.should_update() {
                walk_progress.update();
            }

            let dir_entry = dir.unwrap();

            if let Some(metadata) = &dir_entry.client_state {
                if dir_entry.file_type.is_dir() {
                    walk_state.add_path(dir_entry.path(), metadata);
                } else {
                    walk_state.add_path(dir_entry.parent_path.to_path_buf(), metadata);
                };
            }
        }
        eprintln!("{}", walk_progress);
    }

}
