use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    // get the first cmd argument
    let filename = match env::args().skip(1).next() {
        Some(v) => v,
        None => {
            println!("no file listed");
            std::process::exit(0)
        }
    };

    // create dist directory
    if let Err(err) = fs::create_dir("dist") {
        use io::ErrorKind;

        match err.kind() {
            ErrorKind::AlreadyExists => (),
            _ => {
                eprintln!("Error creating dist directory: {}", err);
                std::process::exit(0)
            }
        }
    }

    // create styles file
    let css_file = include_bytes!("./assets/styles.css");
    let css_path = "./dist/style.css";
    fs::write(css_path, css_file)?;

    // open file
    let mut md_file = File::open(&filename)?;
    let mut md_contents = String::new();
    md_file.read_to_string(&mut md_contents)?;

    // turn md_contents to html
    let html_head = String::from("<head><link rel=\"stylesheet\" href=\"style.css\"></head>");
    let html_body = markdown::to_html(&md_contents);
    let html_content = html_head + &html_body;

    // write to filename.html
    let output_path = get_output_path(filename);
    fs::write(output_path, html_content)?;

    Ok(())
}

pub fn get_output_path(mut filename: String) -> PathBuf {
    if filename.ends_with(".md") {
        filename.replace_range(filename.len() - 3.., ".html");
    } else {
        filename += ".html";
    }

    PathBuf::from("./dist").join(filename)
}
