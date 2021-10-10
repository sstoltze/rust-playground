use std::{env, error::Error, fs};
fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("Usage: elk FILE");
    let input = fs::read(&input_path)?;
    let file = match delf::File::parse_or_print_error(&input[..]) {
        Some(f) => f,
        None => std::process::exit(1),
    };
    println!("{:#?}", file);

    Ok(())
}
