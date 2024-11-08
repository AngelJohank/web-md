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

// get the path of the first element in env::args()
fn get_file_path() -> PathBuf {
    env::args()
        .skip(1)
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| exit("no files listed"))
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

/// TODO
fn export_html(file_path: PathBuf, html_content: String) -> io::Result<()> {
    let working_dir = file_path.parent().unwrap_or(Path::new("."));

    let file_name = file_path.file_stem().unwrap_or_default();
    let full_file_name = format!("{}.html", file_name.to_string_lossy());

    let mut export_path = create_dist_folder(working_dir)?;
    create_styles(&export_path)?;

    export_path.push(full_file_name);
    fs::write(export_path, html_content)?;

    Ok(())
}

fn create_dist_folder(path: &Path) -> io::Result<PathBuf> {
    let path = path.join("dist");

    use io::ErrorKind::AlreadyExists;

    match fs::create_dir(&path) {
        Ok(_) => Ok(path),
        Err(err) if err.kind() == AlreadyExists => Ok(path),
        Err(err) => {
            println!("Error creating dist directory");
            Err(err)
        }
    }
}

fn create_styles(dist_path: &Path) -> io::Result<()> {
    let styles = include_bytes!("./assets/styles.css");
    let styles_path = dist_path.join("style.css");

    fs::write(styles_path, styles)
}
