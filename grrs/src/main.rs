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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("Provided arguments: {} {}", args.pattern, args.path.display());
    
    let args = Cli::from_args();
    
    // let file = File::open(&args.path).expect("Could not read file");
    // let reader = BufReader::new(file);
    // for result in reader.lines() {
    //     match result {
    //         Ok(line) => {
    //             if line.find(&args.pattern).is_some() {
    //                 println!("{}", line);
    //             }
    //         }
    //         Err(error) => { println!("Oh no, {}", error) }
    //     }
    // }
    let content = std::fs::read_to_string(&args.path)?;
    println!("file content: {}", content);
    Ok(())
}
