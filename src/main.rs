use anyhow::Result;
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
struct Cli {
    /// Number of the PR to open on the web. Optional.
    #[arg(value_name = "PR_NUMBER")]
    number: Option<u64>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Check if gh cli is installed
    Check,

    /// Filters PRs
    Filter {
        #[arg(short, long)]
        /// Only show PRs you created
        me: bool,

        // custom
        #[arg(short, long)]
        /// Custom filter. (Check `gh pr list --help` for more info)
        custom: Vec<OsString>,
    },
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    let gh_cli = gh::GhCli::new();

    if let Some(pr_number) = cli.number {
        let response = gh_cli.open_pr_on_web(pr_number);
        handle_response(
            response,
            Some(format!("Opened PR {} on the web", pr_number)),
        );
        std::process::exit(0);
    }

    let mut filters: Option<String> = None;

    match &cli.command {
        Some(Commands::Check) => {
            let response = gh_cli.check_health();
            handle_response(response, Some("gh cli is installed".to_string()));
            std::process::exit(0);
        }

        Some(Commands::Filter { me, custom }) => {
            let mut filter_str = String::new();

            if *me {
                filter_str.push_str("author:@me ");
            }

            for filter in custom {
                filter_str.push_str(&filter.to_string_lossy());
                filter_str.push(' ');
            }

            filters = Some(filter_str);
        }

        None => {}
    }

    let pr_string = match filters {
        Some(filters) => {
            let response = gh_cli.get_filtered_prs(filters);
            handle_response(response, None)
        }
        None => {
            let response = gh_cli.get_all_prs();
            handle_response(response, None)
        }
    };

    let pr_list = parse::parse_pr_list(&pr_string).unwrap_or_else(|e| {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    });

    if pr_list.is_empty() {
        println!("No PRs found");
        std::process::exit(0);
    }

    let pr_graph = parse::PrGraph::new(pr_list);
    println!("{}", pr_graph.format());
}

fn handle_response(response: Result<String>, success_message: Option<String>) -> String {
    match response {
        Ok(m) => {
            if let Some(success_message) = success_message {
                println!("✨ {}", success_message);
            }
            m
        }
        Err(e) => {
            eprintln!("❌ {}", e);
            std::process::exit(1);
        }
    }
}
