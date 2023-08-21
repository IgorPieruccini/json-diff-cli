use std::env;

pub fn get() -> String {
    let mut base_path = String::new();
    match env::current_exe() {
        Ok(exe_path) => {
            let root: String = String::from(&exe_path.display().to_string());
            let mut path_parts: Vec<&str> = root.split("/").collect();

            path_parts.pop();

            let mut url = path_parts.join("/");
            url.push_str("/");

            base_path.push_str(&url);
        }
        _ => {}
    }
    base_path
}
