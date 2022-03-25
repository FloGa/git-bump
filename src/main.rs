use git_bump::bump;

fn main() {
    if let Err(err) = bump() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
