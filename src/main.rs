use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// A Cli tool for pattern matching from files
struct Cli {
    // A statement you'd like to convert to mockCase (i.e. SoMeThInG LiKe ThIs)
    #[arg(short, long)]
    string: String,
}

fn main() {
    let args = Cli::parse();

    let content = std::io::read_to_string(&args.string).expect("DiD yOu MeAn To NoT MoCk SoMeThInG? (You didn't provide a string)");
    let mut new_vec: Vec<char>;

    let mut count: u32 = 0;

    // Loop over the vector chars, if not alphabetic then skip, otherwise increment count by 1
    // If count is divisible by two then capitalize the alphabetic character
    for c in content.to_lowercase().chars() {
        if !c.is_alphabetic() {

        }
    }
}
