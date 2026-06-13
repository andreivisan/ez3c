use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // consider if let Some here instead
    match env::home_dir() {
        Some(home_path) => {
            let path = home_path.join(".claude").join("projects");
            println!("Your Claude directory, probably: {}", path.display());
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                println!("{}", path.display());
            }
        }
        None => println!("Impossible to get your home dir"),
    }
    Ok(())
}
