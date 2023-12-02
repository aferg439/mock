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
    let string_to_mock = &args.string;
    let result: String = mock(&string_to_mock);

    println!("{}", result)
}

fn mock(s: &str) -> String {
    let mut count: u32 = 0;
    let mut result: String = String::new();

    // iterate over characters, alternating upper and lower case letters and skipping
    // non-alphabetic characters
    for c in s.to_lowercase().chars() {
        if !c.is_alphabetic() {
            result.push(c);
        } else {
            if count % 2 == 0 {
                let new_string: String = c.to_string().to_uppercase();
                result.push(new_string.chars().next().expect("String is empty"));
                count += 1;
            } else {
                let new_string: String = c.to_string().to_lowercase();
                result.push(new_string.chars().next().expect("String is empty."));
                count += 1;
            }
        }
    }
    result
}
