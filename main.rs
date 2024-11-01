use clap::Parser;
use ksubst::substitute;
use std::collections::HashMap;
use std::env;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about = "Variable substitution tool")]
struct Args {
    /// Path to .env file
    #[arg(short = 'e', long = "env")]
    env_file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Load variables
    let variables = if let Some(env_file) = args.env_file {
        // Load variables from .env file
        let mut vars = HashMap::new();
        dotenvy::from_path_iter(env_file)?
            .filter_map(Result::ok)
            .for_each(|(key, value)| {
                vars.insert(key, value);
            });
        vars
    } else {
        // Use environment variables
        env::vars().collect::<HashMap<String, String>>()
    };

    // Perform substitution
    let output = substitute(input, &variables)?;

    // Write to stdout
    println!("{}", output);

    Ok(())
}
