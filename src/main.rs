use std::fs::File;
use std::io;
use std::io::Read;

mod util;
use util::get_cmd_args;

// then you have to parse it propperly
// then you gotta convert it to html or something else

fn main() -> io::Result<()> {
    let filenames = get_cmd_args();

    for filename in filenames {
        let mut file = File::open(&filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        println!("file - {filename}:\n\n{contents}")
    }

    Ok(())
}
