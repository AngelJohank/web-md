use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let file_path = get_file_path();
    let file_contents = read_file_to_string(&file_path)?;
    let html_content = md_to_html(file_contents);

    // write to: build/filename.html
    export_html(file_path, html_content)?;

    Ok(())
}

/// Gets the first command-line argument and turns it into a `PathBuf`
/// if `env::args` is empty or the file does not exists, exits the program
fn get_file_path() -> PathBuf {
    let file_path = env::args()
        .skip(1)
        .next()
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

fn exit(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(0);
}

fn read_file_to_string(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn md_to_html(value: String) -> String {
    let html_head = "<head><link rel=\"stylesheet\" href=\"style.css\"></head>";
    let html_body = markdown::to_html(&value);

    format!("{html_head}{html_body}")
}

/// Creates a build folder inside the `file_path` parent folder. Then, inside
/// the build folder creates a `style.css` file and writes the html contents
/// into `filename.html`
fn export_html(file_path: PathBuf, html_content: String) -> io::Result<()> {
    let parent_folder = file_path.parent().unwrap_or(Path::new("."));
    let mut build_path = create_build_dir(parent_folder)?;

    // write styles
    create_style_file(&build_path)?;

    // write html
    let filename = get_file_name(&file_path);
    build_path.push(filename);
    fs::write(build_path, html_content)?;

    Ok(())
}

fn get_file_name(file_path: &Path) -> String {
    let file_stem = match file_path.file_stem() {
        Some(stem) => stem.to_string_lossy().to_string(),
        None => String::from("result"),
    };

    format!("{}.html", file_stem)
}

fn create_style_file(path: &Path) -> io::Result<()> {
    let styles = include_bytes!("./assets/styles.css");
    let styles_path = path.join("style.css");

    fs::write(styles_path, styles)
}

fn create_build_dir(path: &Path) -> io::Result<PathBuf> {
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
