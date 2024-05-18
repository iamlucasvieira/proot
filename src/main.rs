use clap::Parser;
use proot::gh;
use proot::parse;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "A tool to visualize the PR graph of a GitHub repository"
)]
struct Args {
    #[clap(short, long)]
    /// Check if gh cli is installed
    check: bool,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    if args.check {
        if let Err(e) = gh::check_health() {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        println!("✨ gh cli is installed.");
        return;
    }

    let pr_string = gh::get_pr_string().unwrap();
    let pr_list = parse::parse_pr_list(&pr_string).unwrap();
    let pr_graph = parse::PrGraph::new(pr_list);
    println!("{}", pr_graph.format());
}
