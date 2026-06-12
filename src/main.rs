fn main() {
    match std::env::home_dir() {
        Some(path) => println!("Your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir"),
    }
}
