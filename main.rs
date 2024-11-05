use clap::Parser;
use ksubst::substitute;
use std::collections::HashMap;
use std::env;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about = "Variable substitution tool")]
struct Args {
    /// Path to .env file
    #[arg(short = 'e', long = "env")]
    env_file: Option<String>,

    /// Recursively process files in input directory
    #[arg(short = 'r', long = "recursive", requires_all = ["input_dir", "output_dir"])]
    recursive: bool,

    /// Input directory (required if -r is specified)
    #[arg()]
    input_dir: Option<String>,

    /// Output directory (required if -r is specified)
    #[arg()]
    output_dir: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

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

    if args.recursive {
        // Process directory recursively
        let input_dir = args.input_dir.unwrap();
        let output_dir = args.output_dir.unwrap();

        process_directory_recursively(&input_dir, &output_dir, &variables)?;
    } else {
        // Read from stdin
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;

        // Perform substitution
        let output = substitute(input, &variables)?;

        // Write to stdout
        println!("{}", output);
    }

    Ok(())
}

fn process_directory_recursively(
    input_dir: &str,
    output_dir: &str,
    variables: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(input_dir) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Read file content
            let input_content = std::fs::read_to_string(path)?;

            // Perform substitution
            let output_content = substitute(&input_content, variables)?;

            // Compute output path
            let relative_path = path.strip_prefix(input_dir)?;
            let output_path = Path::new(output_dir).join(relative_path);

            // Create parent directories if needed
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write output file
            std::fs::write(output_path, output_content)?;
        }
    }

    Ok(())
}
