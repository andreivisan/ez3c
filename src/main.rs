use std::env;

fn main() {
    // consider if let Some here instead
    match env::home_dir() {
        Some(home_path) => {
            let path = home_path.join(".claude").join("projects");
            println!("Your Claude directory, probably: {}", path.display());
        }
        None => println!("Impossible to get your home dir"),
    }
}
