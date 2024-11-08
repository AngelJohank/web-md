use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // get the first cmd argument
    let file_path = get_file_path();
    // open md_file
    let file_contents = get_file_contents(&file_path)?;
    // turn md_contents to html
    let html_content = md_to_html(file_contents);
    // write to filename.html
    export_html(file_path, html_content)?;

    Ok(())
}

fn get_file_path() -> PathBuf {
    // Get the first command-line argument
    let file_path = env::args()
        .skip(1)
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| exit("no file listed"));

    // Check if the file exists
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

fn get_file_contents(file_path: &Path) -> io::Result<String> {
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

fn export_html(file_path: PathBuf, html_content: String) -> io::Result<()> {
    let working_dir = file_path.parent().unwrap_or(Path::new("."));
    let filename = get_file_name(&file_path);

    // export_path = working_dir/build/
    let mut build_path = create_build_dir(working_dir)?;
    create_style_file(&build_path)?;

    // export_path = working_dir/build/filename.html
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

fn create_style_file(working_dir: &Path) -> io::Result<()> {
    let styles = include_bytes!("./assets/styles.css");
    let styles_path = working_dir.join("style.css");

    fs::write(styles_path, styles)
}

fn create_build_dir(working_dir: &Path) -> io::Result<PathBuf> {
    let build_path = working_dir.join("build");

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
