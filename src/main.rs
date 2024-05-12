use proot::gh;

fn main() {
    if let Err(e) = gh::check_health() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    println!("gh cli is installed");

    let pr_string = gh::get_pr_string().unwrap();

    let pr_list = proot::parse::parse_pr_list(&pr_string).unwrap();

    for pr in &pr_list {
        println!("{:?}", pr);
    }

    println!("Total PRs: {}", pr_list.len());
}
