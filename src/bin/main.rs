use structopt::StructOpt;

mod cli;

fn main() {
    let context = cli::arguments::Arguments::from_args();

    println!("cli options: {:?}", context);
}
