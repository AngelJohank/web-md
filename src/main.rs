use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // get the first cmd arguments
    let file_path = get_file_path();

    // open md_file
    let file_contents = get_file_contents(&file_path)?;

    // turn md_contents to html
    let html_content = md_to_html(file_contents);

    // write to filename.html
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

/// Exit the program with a custom message and exit code of 0
fn exit(msg: &str) -> ! {
    println!("{}", msg);
    std::process::exit(0);
}

/// Attempts to open a file and read it's contents to a String
fn get_file_contents(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(&file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

/// Takes a markdown String and turns it into an html String
fn md_to_html(value: String) -> String {
    let html_head = "<head><link rel=\"stylesheet\" href=\"style.css\"></head>";
    let html_body = markdown::to_html(&value);

    format!("{html_head}{html_body}")
}

/// With the given `file_path` and `html_content` creates a build folder inside the
/// `file_path` parent folder. The build folder looks like this: `parent_folder/build`
///
/// Then, inside the build folder creates a `style.css` file
/// and writes the html contents into `filename.html`
fn export_html(file_path: PathBuf, html_content: String) -> io::Result<()> {
    let root = Path::new(".");
    let parent_folder = file_path.parent().unwrap_or(root);

    // create build folder inside parent_folder
    let mut build_path = create_build_dir(parent_folder)?;

    // write style.css inside build folder
    create_style_file(&build_path)?;

    // write filename.html inside build folder
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
