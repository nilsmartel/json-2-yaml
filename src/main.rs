use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Convert JSON to YAML")]
struct Args {
    /// Input JSON file
    input: PathBuf,

    /// Output YAML file (optional)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Read JSON file
    let json_content = fs::read_to_string(&args.input)?;

    // Parse JSON
    let value: serde_json::Value = serde_json::from_str(&json_content)?;

    // Convert to YAML
    let yaml_output = serde_yaml::to_string(&value)?;

    // Determine output path
    let output_path = match args.output {
        Some(path) => path,
        None => {
            let mut path = args.input.clone();
            path.set_extension("yaml");
            path
        }
    };

    // Write YAML to file
    fs::write(&output_path, yaml_output)?;

    println!("Converted {:?} to {:?}", args.input, output_path);
    Ok(())
}
