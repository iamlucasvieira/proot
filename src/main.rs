use clap::Parser;
use proot::gh;
use proot::parse;
use std::ffi::OsString;

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

    /// Number of the PR to open on the web. Optional.
    #[arg(value_name = "PR_NUMBER")]
    number: Option<OsString>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    if args.check {
        if let Err(e) = gh::check_health() {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
        println!("✨ gh cli is installed.");
    }

    if let Some(pr_number) = args.number {
        let number = pr_number
            .into_string()
            .unwrap_or(String::new())
            .parse::<u64>()
            .unwrap_or_else(|_| {
                eprintln!("❌ PR number must be a number.");
                std::process::exit(1);
            });

        if let Err(e) = gh::open_pr_on_web(number) {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }

        println!("✨ Opened PR {} on the web.", number);
        return;
    }

    let pr_string = gh::get_pr_string().unwrap();
    let pr_list = parse::parse_pr_list(&pr_string).unwrap();
    let pr_graph = parse::PrGraph::new(pr_list);
    println!("{}", pr_graph.format());
}
