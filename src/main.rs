use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// A Cli tool for pattern matching from files
struct Cli {
    // A statement you'd like to convert to mockCase (i.e. SoMeThInG LiKe ThIs)
    #[arg(short, long)]
    string: String,

    // An ASCII art depiction of SpongeBob mocking your target
    // Argument is optional, if not present, the string will be mocked and returned
    #[arg(short, long)]
    picture: bool,

    // An optional argument to call a Generative AI model to mock the string
    #[arg(short, long)]
    generate: Model,
}

// Provide options and configs for different model providers
struct Model {
    // The name of the model
    model: String,

    // The model's API key - pulled from environment variables
    api_key: String,

    // The model's endpoint - implicitly defined by the function call
    endpoint: String,
}

fn main() {
    let args = Cli::parse();

    // Access string for mocking
    let string_to_mock = &args.string;

    // if bedrock arg present, call Amazon Bedrock to mock the string
    // TODO implement a case for different model providers
    if args.bedrock {
        let result: String = mock_with_bedrock(&string_to_mock);
    } else {
        let result: String = mock(&string_to_mock);
    }

    // If picture arg present, draw the meme - otherwise return mocked string.
    if args.picture {
        draw_meme();
        println!("{}", result);
    } else {
        println!("{}", result);
    }
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

fn draw_meme() {
    // draw mocking spongebob meme if arg present
    println!(r"⠀⠀⠀⠀⠀⠀⠀⠀⣠⣶⣖⡲⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⡏⠀⠀⠑⣄⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢧⠀⣄⠀⠈⣆⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠈⢧⠹⡆⠀⠈⢧⢧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣠⠤⢤⣨⣧⡾⠦⠃⠀⢳⣷⠦⣤⣤⡖⠒⠒⠲⢴⣶⣯⣝⠒⠒⢋⡹⠳⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⣠⣤⠞⠁⢻⡿⡿⠿⠿⢤⣀⣤⣾⣿⣦⠈⣿⣿⣦⣤⣤⡀⠉⠙⠉⠀⡰⠋⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣀⣤⡞⠁⠀⢀⣀⡀⠿⠿⢆⣠⡼⠟⠋⢉⣾⠏⠀⠛⢻⣿⣛⡛⠁⠀⣀⡤⠚⠀⠀⣀⡀⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⣟⠀⣽⣭⡍⠉⠉⠉⠉⠱⣿⣁⣀⣠⣴⣿⠋⠀⠉⠉⠉⢀⣴⣮⡉⠉⠁⢹⠀⢠⣾⣿⣿⡌⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠈⠑⣭⠝⣿⠀⢠⠀⠀⠀⠈⠛⠛⠛⠉⠀⠀⠀⠀⡴⠊⣿⣀⣸⡉⠑⠂⢸⠀⢸⣿⣿⡿⠃⣏⠀⠀⠀⠀⠀⠀⠀⣠⡴⠻⡆⠀⠀⠀
⢠⣞⡿⠚⠋⠉⠙⠳⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠟⠊⠉⠉⠉⠙⠲⡄⠈⢆⠀⠉⠉⢀⣀⠘⢦⠀⢀⡤⠤⣤⡞⣩⣶⢠⠇⠀⠀⠀
⠀⡟⣿⣾⡓⣆⠀⠀⠘⡆⠀⠀⠀⠀⠀⠀⠀⢠⡏⠀⠀⠀⣰⣶⣖⣆⢹⠀⠀⣳⡀⠀⢺⡿⠂⢸⡶⠋⠀⠀⠀⢹⠁⡇⢸⢠⣶⢶⣄
⠀⢧⡙⠿⠟⠃⠀⠀⡰⠃⡀⠀⠀⠀⠀⠀⠀⠈⢧⡀⠀⠀⠘⠿⣿⣿⠏⠀⠀⢸⡇⠀⠀⠀⢠⣾⠁⠀⠀⠀⣠⠞⢰⡇⣸⣿⢏⡏⣽
⠀⠈⠙⢦⣤⣤⠴⠊⣡⠞⢡⡟⠀⠀⠀⠀⠀⠀⠀⠙⣖⣒⡒⣚⣩⡵⠂⠀⠀⠀⠳⣄⠀⣰⣿⣇⠀⢀⣠⣶⣿⣶⣿⡇⢟⡤⠘⣰⠃
⠀⠀⠀⠀⣨⠷⠖⠋⠁⢀⡞⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠀⠘⡆⢹⣿⡿⠋⠉⡀⠙⣿⠛⢻⣧⣌⣤⣾⡇⠀
⠀⠀⠀⠸⣇⠀⠀⣀⡴⠋⠀⠿⠁⠀⠀⠀⢀⣀⣤⣤⣤⣤⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠧⣀⠉⠀⠀⢾⣿⠀⣯⠀⠈⣿⣿⣿⣿⣿⡄
⠀⠀⠀⠀⠈⠉⢹⠏⠀⠀⠀⠀⠀⢀⣠⣶⣿⣿⡿⠿⠛⠛⠋⣁⡀⠀⠀⣴⣶⣶⣶⣤⠀⠀⢱⡀⠀⠀⠀⠀⢸⠀⠀⢸⣿⣿⡿⠟⠋
⠀⠀⠀⠀⠀⠀⢸⡄⠀⠀⠀⠀⣰⣟⡿⡟⢋⣁⣤⠤⠶⠛⠛⠉⠀⠀⠀⠛⠛⠛⠛⢉⣠⣄⡀⠳⠶⢤⡀⠀⠸⡆⠀⣠⠿⠋⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢧⠀⠀⠀⡾⠙⠧⠔⠚⠛⠳⠤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠋⠁⠀⢀⣀⣉⣭⡷⠟⠉⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠈⢧⡀⡼⠁⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⣿⣿⡾⠛⠒⠒⠒⠒⠒⠚⠛⢻⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠁⠀⠀⠀⠀⠀⠀⢀⡸⢋⡽⠿⠿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠈⢻⠇⣿⠿⠿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣴⣿⡦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡟⢰⣇⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣷⣦⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⢿⣏⠓⢿⣿⣦⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣿⣿⠿⠿⠿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣤⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣿⣿⠿⠋⠀⠀⠀⠀⠀⠀⠀⠀");
}