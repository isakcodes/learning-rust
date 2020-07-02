use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    //println!("Provided arguments: {} {}", args.pattern, args.path.display());
    let file = File::open(&args.path).expect("Could not read file");
    let reader = BufReader::new(file);
    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.find(&args.pattern).is_some() {
            println!("{}", line);
        }
    }
}
