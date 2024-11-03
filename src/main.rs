use std::fs::File;
use std::io::Read;
use std::io::{self, Write};

mod util;
use util::{get_cmd_args, get_output_filename};

fn main() -> io::Result<()> {
    let filenames = get_cmd_args();

    for filename in filenames {
        // open file
        let mut file = File::open(&filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // convert to HTML
        let html_content = markdown::to_html(&contents);

        // write to filename.html
        let output_filename = get_output_filename(filename);
        let mut output_file = File::create(output_filename)?;
        output_file.write_all(html_content.as_bytes())?;
    }

    Ok(())
}
