use clap::Parser;
use entity_scope_generator::generate_rust;

#[derive(Parser)]
#[clap(name = "entityscope")]
enum Cli {
    GenRust {
        package_name: String,
        file_path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli {
        Cli::GenRust {
            package_name,
            file_path,
        } => {
            generate_rust(&package_name, &file_path)?;
        }
    }

    Ok(())
}
