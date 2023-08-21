mod exe_root;
mod prompt_options;
mod select_file;
use std::{env, fs::DirEntry};

fn main() {
    let dir = env::current_dir().unwrap();

    fn call_back<'a>(selected: &'a DirEntry) {
        println!("{}", selected.path().display().to_string());
    }

    select_file::visit_dirs(dir.as_path(), &call_back);
}
