extern crate dirs;
extern crate chrono;
extern crate clap;
use std::{env, fs, path::{PathBuf, Path}, io::{Write, self, BufRead}};
use chrono::{Local, Datelike};
use clap::Parser;

#[derive(Parser)]
#[command(name = "note")]
#[command(author = "Morgan Hester <morganhasebehester@gmail.com>")]
#[command(about = "CLI note taking and orgainizing app", long_about = "Takes the given input note and creates a file containing the date and time the note was taken and the contents of the note. If the supplied argument is a file, it will give the full path to the file and a sample of the contents, if it is a string it will contain the full contents. This will be organized in a file tree with the format \"$HOME/notes/$YEAR/$MONTH/$DAY\".")]
struct Cli {
    ///Mandatory argument for the note to take. Can be a file or string surounded by ""
    note: String,

    ///Set a different output path
    #[arg(short,long)]
    output: Option<String>,

    ///Set a catagory to add notes to, rather than the standard date
    #[arg(short,long)]
    category: Option<String>,
}

fn main() {
    //Get arguments
    let cli = Cli::parse();
    //Get home dir
    let home = match dirs::home_dir() {
        Some(path) => path,
        None => match env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("Could not find Home dir or Current path."),
        },
    };

    //Get the date
    let date = Local::now();
    //Initialize path var and set it to date organization or given output if --output is set
    let path: PathBuf;
    if let Some(string) = cli.output {
        path = PathBuf::from(string);
    } else if let Some(string) = cli.category {
        path = [home, PathBuf::from("notes"), PathBuf::from(string), PathBuf::from(date.format("%d-%m-%Y").to_string())].iter().collect();
    } else {
        path = [home, PathBuf::from("notes"), PathBuf::from(date.year().to_string()), PathBuf::from(date.format("%b").to_string()), PathBuf::from(date.format("%e").to_string())].iter().collect();
    }

    //Get the parent dir and create the file tree if necessary
    let parent_dir = match path.parent() {
        Some(thing) => thing,
        None => panic!("Could not get parent directory.")
    };
    if !parent_dir.exists(){
        fs::create_dir_all(parent_dir).expect("Could not create path");
    }

    //Create the contents of the note, tagging the time and date first, then the note
    let mut contents = date.format("Note from: %H:%M:%S %Y-%m-%d\n").to_string();
    //Tests if the passed argument is a file
    if Path::new(&cli.note).is_file() {
        //If it is get the absolute path of it and read the first 5 lines in to contents
        let abs_path = fs::canonicalize(PathBuf::from(cli.note)).expect("Could not get absolute path to file.");
        let abs_path_str = abs_path.clone(); //This seems unfortunately necessary because abs_path gets consumed so I can't use it in printing later
        let file = fs::File::open(abs_path).expect("Could not open file.");
        let reader = io::BufReader::new(file);
        let lines = reader.lines();

        contents.push_str("Exerpt from: ");
        contents.push_str(format!("{:?}\n", abs_path_str).as_str());

        //Only taking 5 lines because that file should live on the computer somewhere, so why copy all of it
        //Some context should be all that is needed
        let mut count = 0;
        for line in lines {
            contents.push_str(&line.expect("Could not read file."));
            contents.push('\n');
            count += 1;
            if count > 5 {
                break
            }
        }
        contents.push_str("...");
        contents.push_str("\n\n");
    } else {
        //If not a file, just put the argument in contents as written
        contents.push_str(cli.note.as_str());
        contents.push_str("\n\n");
    }

    //Actually create the file (if needed) and write the contents to it
    let mut note_file = fs::OpenOptions::new().append(true).create(true).open(path).expect("Could not create or open file.");
    write!(note_file, "{}", contents).expect("Could not write to file.");
}
