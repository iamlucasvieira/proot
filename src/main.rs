use proot::gh;

fn main() {
    if let Err(e) = gh::check_health() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("gh cli is installed");
}
