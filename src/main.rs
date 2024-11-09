use std::fs;
use std::io;
use std::path::{Path, PathBuf};

mod webmd;

fn main() -> io::Result<()> {
    let file_path = webmd::get_file_path();
    let file_contents = webmd::read_file_to_string(&file_path)?;
    let html_content = webmd::md_to_html(file_contents);

    // write to: build/filename.html
    export_html(file_path, html_content)?;

    Ok(())
}

fn export_html(file_path: PathBuf, html_content: String) -> io::Result<()> {
    // create build_path
    let default_path = Path::new(".");
    let parent_folder = file_path.parent().unwrap_or(default_path);
    let mut build_path = webmd::create_build_dir(parent_folder)?;

    // write styles
    webmd::create_style_file(&build_path)?;

    // write html
    let filename = webmd::get_file_name(&file_path);
    build_path.push(filename);
    fs::write(build_path, html_content)?;

    Ok(())
}
