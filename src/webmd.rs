use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn exit(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(0);
}

/// Gets the first command-line argument and turns it into a `PathBuf`
/// if `env::args` is empty or the file does not exists, exits the program
pub fn get_file_path() -> PathBuf {
    let file_path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| exit("no file listed"));

    if !file_path.is_file() {
        let msg = format!(
            "Path: {}\ndoes not point to a file",
            file_path.to_string_lossy()
        );
        exit(&msg)
    }

    file_path
}

pub fn get_file_name(file_path: &Path) -> String {
    let file_stem = match file_path.file_stem() {
        Some(stem) => stem.to_string_lossy().to_string(),
        None => String::from("result"),
    };

    format!("{}.html", file_stem)
}

pub fn read_file_to_string(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn create_build_dir(path: &Path) -> io::Result<PathBuf> {
    let build_path = path.join("build");

    if build_path.is_dir() {
        return Ok(build_path);
    }

    match fs::create_dir(&build_path) {
        Ok(_) => Ok(build_path),
        Err(err) => {
            println!("Error creating dist directory");
            Err(err)
        }
    }
}

/// Turn a markdown string to html with custom style.css link
pub fn md_to_html(value: String) -> String {
    let html_head = "<head><link rel=\"stylesheet\" href=\"style.css\"></head>";
    let html_body = markdown::to_html(&value);

    format!("{html_head}{html_body}")
}

pub fn create_style_file(path: &Path) -> io::Result<()> {
    let styles = include_bytes!("./assets/styles.css");
    let styles_path = path.join("style.css");

    fs::write(styles_path, styles)
}
