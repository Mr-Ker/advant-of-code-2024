use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("input_examples"))]
    input_folder: String,
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    println!("Running day {} with input folder <{}>!", args.day, args.input_folder);
}