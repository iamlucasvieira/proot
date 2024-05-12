use serde::Deserialize;

/// Parses the input of gh pr list json output
#[derive(Deserialize)]
pub struct Pr {
    //title is optional
    title: Option<String>,
    state: String,
    base_ref_name: String,
    head_ref_name: String,
}

pub fn parse_pr_list(json: &str) -> Result<Vec<Pr>, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pr_list() {
        let json = r#"
            [
                    {
                        "title": "Add feature",
                        "state": "OPEN",
                        "base_ref_name": "main",
                        "head_ref_name": "feature"
                    },
                    {
                        "title": "Fix bug",
                        "state": "CLOSED",
                        "base_ref_name": "main",
                        "head_ref_name": "bugfix"
                    }
            ]
        "#;

        let pr_list = parse_pr_list(json).unwrap();
        assert_eq!(pr_list.len(), 2);
        assert_eq!(pr_list[0].title, Some("Add feature".to_string()));
        assert_eq!(pr_list[0].state, "OPEN");
        assert_eq!(pr_list[0].base_ref_name, "main");
        assert_eq!(pr_list[0].head_ref_name, "feature");
        assert_eq!(pr_list[1].title, Some("Fix bug".to_string()));
        assert_eq!(pr_list[1].state, "CLOSED");
        assert_eq!(pr_list[1].base_ref_name, "main");
    }

    #[test]
    fn test_pr_missing_field() {
        let test_cases = vec![
            r#"
                [
                    {
                        "title": "Add feature",
                        "state": "OPEN",
                        "base_ref_name": "main"
                    }
                ]
            "#,
            r#"
                [
                    {
                        "title": "Add feature",
                        "state": "OPEN",
                        "head_ref_name": "feature"
                    }
                ]
            "#,
            r#"
                [
                    {
                        "title": "Add feature",
                        "base_ref_name": "main",
                        "head_ref_name": "feature"
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
                    "state": "OPEN",
                    "base_ref_name": "main",
                    "head_ref_name": "feature"
                }
            ]
        "#;

        let pr_list = parse_pr_list(json).unwrap();
        assert_eq!(pr_list.len(), 1);
        assert_eq!(pr_list[0].title, None);
        assert_eq!(pr_list[0].state, "OPEN");
        assert_eq!(pr_list[0].base_ref_name, "main");
        assert_eq!(pr_list[0].head_ref_name, "feature");
    }
}
