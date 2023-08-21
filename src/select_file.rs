use dialoguer::{console::Term, theme::ColorfulTheme, Select};

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        let mut options: Vec<String> = Vec::new();
        let mut paths: Vec<String> = Vec::new();

        for entry in fs::read_dir(dir)? {
            let val = entry.unwrap();
            options.push(val.file_name().to_str().unwrap().to_owned().to_string());
            paths.push(val.path().to_owned().display().to_string())
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .expect("error");

        if paths.len() >= selection.unwrap() {
            let ref selected = paths[selection.unwrap()];
            let selected_path = Path::new(&selected);
            if selected_path.is_dir() {
                println!("is Dir");
                visit_dirs(&selected_path, cb)?;
            } else {
                println!("finally selected a file");
                // cb(&entry);
            }
        }
    }
    Ok(())
}
