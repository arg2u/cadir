use cadir::run;
use std::path::PathBuf;
use structopt;
use structopt::StructOpt;
#[allow(unused)]
#[derive(Debug, StructOpt)]
#[structopt(
    name = "cadir",
    about = "This is a tool to create one or many directories. It provides recursive and multithreading modes."
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    directories: Vec<PathBuf>,
    /// Recursively create all paths
    #[structopt(short)]
    recursively: bool,
    /// Run task in multithreading mode
    #[structopt(short)]
    multithread: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.directories.len() > 0 {
        run(opt.directories, opt.recursively, opt.multithread);
    } else {
        println!("The directories weren't provided.\nPlease, use -h flag to show usage details.");
        std::process::exit(0);
    }
}
