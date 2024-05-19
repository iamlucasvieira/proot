use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;

/// Pr represents a pull request data parsed from the json response of GitHub CLI.
#[derive(Deserialize, Debug)]
pub struct Pr {
    id: String,
    number: u32,
    title: Option<String>,
    url: String,
    state: String,
    #[serde(rename = "isCrossRepository")]
    is_cross_repository: bool,
    #[serde(rename = "baseRefName")]
    base_ref_name: String,
    #[serde(rename = "headRefName")]
    head_ref_name: String,
    #[serde(rename = "headRepositoryOwner")]
    head_repository_owner: HeadRepositoryOwner,
}

#[derive(Deserialize, Debug)]
pub struct HeadRepositoryOwner {
    login: String,
}
/// Parse the json response from GitHub CLI to a list of Pr.
///
/// Returns a list of `Pr` if the json is successfully parsed, otherwise an error.
pub fn parse_pr_list(json: &str) -> Result<Vec<Pr>, serde_json::Error> {
    serde_json::from_str(json)
}

/// PrGraph represents a graph of pull requests.
pub struct PrGraph {
    adjacency_list: HashMap<String, Vec<String>>,
    prs: HashMap<String, Pr>,
}

impl PrGraph {
    /// Construct a new PrGraph from a list of Pr.
    ///
    /// Each `Pr` is indexed by a unique identifier constructed from the head and base ref names.
    pub fn new(prs: Vec<Pr>) -> Self {
        let mut adjacency_list = HashMap::new();
        let mut prs_map = HashMap::new();

        for pr in prs {
            let base_ref_name = pr.base_ref_name.clone();
            let mut head_ref_name = pr.head_ref_name.clone();

            if pr.is_cross_repository {
                head_ref_name = format!("{}/{}", pr.head_repository_owner.login, head_ref_name);
            }

            prs_map.insert(PrGraph::identifier(&head_ref_name, &base_ref_name), pr);

            adjacency_list
                .entry(base_ref_name)
                .or_insert_with(Vec::new)
                .push(head_ref_name);
        }

        Self {
            adjacency_list,
            prs: prs_map,
        }
    }

    /// Construct a unique identifier for a pull request.
    ///
    /// The identifier has the format `head_ref_name->base_ref_name`.
    fn identifier(head_ref_name: &str, base_ref_name: &str) -> String {
        format!("{}->{}", head_ref_name, base_ref_name)
    }

    /// Get a pull request by head and base ref names.
    ///
    /// Returns a reference to the `Pr` if it exists, otherwise `None`.
    fn get_pr(&self, head_ref_name: &str, base_ref_name: &str) -> Option<&Pr> {
        self.prs
            .get(&PrGraph::identifier(head_ref_name, base_ref_name))
    }

    /// Get the starting nodes of the graph.
    ///
    /// The starting nodes are the nodes that are not referenced by any other node.
    fn get_starting_nodes(&self) -> Vec<String> {
        self.adjacency_list
            .keys()
            .filter(|base_ref_name| {
                !self
                    .adjacency_list
                    .values()
                    .any(|head_ref_names| head_ref_names.contains(base_ref_name))
            })
            .cloned()
            .collect()
    }

    pub fn format(&self) -> String {
        let mut visited = HashMap::new();
        let starting_nodes = self.get_starting_nodes();
        let mut result = String::new();

        result.push_str("Pull Request Graph\n\n");

        for starting_node in starting_nodes {
            let connector = "┌";
            result.push_str(&format!(
                "{} {}\n",
                connector.blue().bold(),
                starting_node.blue().bold()
            ));
            self.dfs_print(&starting_node, &mut visited, "", &mut result);
            result.push('\n');
        }
        result
    }

    fn dfs_print(
        &self,
        node: &str,
        visited: &mut HashMap<String, bool>,
        ident: &str,
        result: &mut String,
    ) {
        if visited.contains_key(node) {
            return;
        }

        visited.insert(node.to_string(), true);

        if let Some(head_ref_names) = self.adjacency_list.get(node) {
            let count = head_ref_names.len();
            for (index, head_ref_name) in head_ref_names.iter().enumerate() {
                let is_last = index == count - 1;
                let connector = if index < count - 1 {
                    "├──○"
                } else {
                    "└──○"
                };
                let parent_connector = if is_last { " " } else { "│" };

                match self.get_pr(head_ref_name, node) {
                    Some(pr) => {
                        let title = pr.title.clone().unwrap_or("".to_string());
                        let number = format!("[#{}]", pr.number);
                        result.push_str(&format!(
                            "{}{} {} {} - {}\n",
                            ident,
                            connector.blue().bold(),
                            number.dimmed(),
                            head_ref_name,
                            title.dimmed()
                        ));
                    }
                    None => {
                        result.push_str(&format!("{}  {}\n", ident, head_ref_name));
                    }
                }
                self.dfs_print(
                    head_ref_name,
                    visited,
                    &format!("{}{}  ", ident, parent_connector.blue().bold()),
                    result,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRS_DATA: &str = r#"
        [

            {
                "id": "1",
                "number": 1,
                "title": "Add feature",
                "url": "www.example.com",
                "state": "OPEN",
                "isCrossRepository": false,
                "baseRefName": "main",
                "headRefName": "feature",
                "headRepositoryOwner": {
                    "login": "owner"
                }
            },
            {
                "id": "2",
                "number": 2,
                "title": "Fix bug",
                "url": "www.example.com",
                "state": "CLOSED",
                "isCrossRepository": false,
                "baseRefName": "main",
                "headRefName": "bugfix",
                "headRepositoryOwner": {
                    "login": "owner"
                }
            },
            {
                "id": "3",
                "number": 3,
                "title": "Refactor code",
                "url": "www.example.com",
                "state": "OPEN",
                "isCrossRepository": false,
                "baseRefName": "main",
                "headRefName": "refactor",
                "headRepositoryOwner": {
                    "login": "owner"
                }
            },
            {
                "id": "4",
                "number": 4,
                "title": "Add tests",
                "url": "www.example.com",
                "state": "OPEN",
                "isCrossRepository": false,
                "baseRefName": "feature",
                "headRefName": "tests",
                "headRepositoryOwner": {
                    "login": "owner"
                }
            },
            {
                "id": "5",
                "number": 5,
                "title": "Fix tests",
                "url": "www.example.com",
                "state": "OPEN",
                "isCrossRepository": false,
                "baseRefName": "bugfix",
                "headRefName": "fix-tests",
                "headRepositoryOwner": {
                    "login": "owner"
                }
            },
            {
                "id": "6",
                "number": 6,
                "title": "Refactor tests",
                "url": "www.example.com",
                "state": "OPEN",
                "isCrossRepository": false,
                "baseRefName": "refactor",
                "headRefName": "refactor-tests",
                "headRepositoryOwner": {
                    "login": "owner"
                }
            },
            {
                "id": "7",
                "number": 7,
                "title": "Add feature",
                "url": "www.example.com",
                "state": "OPEN",
                "isCrossRepository": false,
                "baseRefName": "dev",
                "headRefName": "feature",
                "headRepositoryOwner": {
                    "login": "owner"
                }
            }
        ]
    "#;

    #[test]
    fn test_parse_pr_list() {
        let pr_list = parse_pr_list(PRS_DATA).expect("Prs should be parsed without error");
        assert_eq!(pr_list.len(), 7);
    }

    #[test]
    fn test_pr_missing_field() {
        let test_cases = vec![
            r#"
                [
                    {
                        "id": "1",
                        "number": 1,
                        "title": "Add feature",
                        "url": "www.example.com",
                        "state": "OPEN",
                        isCrossRepository": false,
                        "baseRefName": "main"
                    }
                ]
            "#,
            r#"
                [
                    {
                        "id": "1",
                        "number": 1,
                        "title": "Add feature",
                        "url": "www.example.com",
                        "state": "OPEN",
                        "headRefName": "feature"
                    }
                ]
            "#,
            r#"
                [
                    {
                        "id": "1",
                        "number": 1,
                        "title": "Add feature",
                        "url": "www.example.com",
                        "baseRefName": "main",
                        "headRefName": "feature"
                    }
                ]
            "#,
        ];

        test_cases.iter().for_each(|json| {
            let pr_list = parse_pr_list(json);
            assert!(pr_list.is_err());
        });
    }

    #[test]
    fn test_allow_missing_title() {
        let json = r#"
            [
                {
                    "id": "1",
                    "number": 1,
                    "url": "www.example.com",
                    "state": "OPEN",
                    "isCrossRepository": false,
                    "baseRefName": "main",
                    "headRefName": "feature",
                    "headRepositoryOwner": {
                        "login": "owner"
                    }
                }
            ]
        "#;

        let pr_list = parse_pr_list(json).expect("Prs should be parsed without error");
        assert_eq!(pr_list.len(), 1);
        assert_eq!(pr_list[0].title, None);
        assert_eq!(pr_list[0].state, "OPEN");
        assert_eq!(pr_list[0].base_ref_name, "main");
        assert_eq!(pr_list[0].head_ref_name, "feature");
    }

    #[test]
    fn test_pr_graph() {
        let pr_list = parse_pr_list(PRS_DATA).expect("Prs should be parsed without error");
        let pr_graph = PrGraph::new(pr_list);

        assert_eq!(pr_graph.adjacency_list.len(), 5);
        assert_eq!(pr_graph.prs.len(), 7);
    }

    #[test]
    fn test_pr_grph_all_ajdacents_in_prs() {
        let pr_list = parse_pr_list(PRS_DATA).expect("Prs should be parsed without error");
        let pr_graph = PrGraph::new(pr_list);

        for (base_ref_name, head_ref_names) in &pr_graph.adjacency_list {
            for head_ref_name in head_ref_names {
                assert!(pr_graph.get_pr(head_ref_name, base_ref_name).is_some());
            }
        }
    }

    #[test]
    fn test_pr_graph_identifier() {
        let head_ref_name = "feature";
        let base_ref_name = "main";

        let identifier = PrGraph::identifier(head_ref_name, base_ref_name);
        assert_eq!(identifier, "feature->main");
    }

    #[test]
    fn test_pr_graph_get_pr() {
        let pr_list = parse_pr_list(PRS_DATA).expect("Prs should be parsed without error");
        let pr_graph = PrGraph::new(pr_list);

        let pr = pr_graph.get_pr("feature", "main").unwrap();
        assert_eq!(pr.id, "1");
    }

    #[test]
    fn test_pr_graph_get_starting_nodes() {
        let pr_list = parse_pr_list(PRS_DATA).expect("Prs should be parsed without error");
        let pr_graph = PrGraph::new(pr_list);

        let starting_nodes = pr_graph.get_starting_nodes();
        assert_eq!(starting_nodes.len(), 2);
        assert!(starting_nodes.contains(&"main".to_string()));
        assert!(starting_nodes.contains(&"dev".to_string()));
    }
}
