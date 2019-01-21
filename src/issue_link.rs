use failure::Error;
use regex::Regex;
use serde_derive::*;
use url::percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct IssueLink {
    repository: String,
    title: String,
    body: String,
    assignees: Vec<String>,
    labels: Vec<String>,
    projects: Vec<String>,
}

impl IssueLink {
    pub fn new<T: ToString>(
        repository: T,
        title: T,
        body: T,
        assignees: Vec<T>,
        labels: Vec<T>,
        projects: Vec<T>,
    ) -> Self {
        let repository = repository.to_string();
        let title = title.to_string();
        let body = body.to_string();
        let assignees = assignees.iter().map(|x| x.to_string()).collect();
        let labels = labels.iter().map(|x| x.to_string()).collect();
        let projects = projects.iter().map(|x| x.to_string()).collect();

        IssueLink {
            repository,
            title,
            body,
            assignees,
            labels,
            projects,
        }
    }

    pub fn print_link(&self) -> Result<(), Error> {
        let link = self.generate_link()?;
        let link = utf8_percent_encode(&link, QUERY_ENCODE_SET).to_string();
        println!("{}", link);

        Ok(())
    }

    pub fn generate_link(&self) -> Result<String, Error> {
        self.validate_issue_link()?;

        let prefix = format!("https://github.com/{}/issues/new", self.repository);
        let title = format!("title={}", self.title);
        let body = format!("body={}", self.body);

        let assignees = format!("assignees={}", self.assignees.join(","));
        let labels = format!("labels={}", self.labels.join(","));
        let projects = format!("projects={}", self.projects.join(","));

        Ok(format!(
            "{}?{}&{}&{}&{}&{}",
            prefix, title, body, assignees, labels, projects
        ))
    }

    fn validate_issue_link(&self) -> Result<(), Error> {
        if self.repository.is_empty() {
            failure::bail!("repository is required!");
        }

        let repo_pattern = Regex::new(r".*/.*").unwrap();
        if !repo_pattern.is_match(&self.repository) {
            failure::bail!("repository is invalid!");
        }

        Ok(())
    }
}

impl Default for IssueLink {
    fn default() -> Self {
        IssueLink {
            repository: String::new(),
            title: String::new(),
            body: String::new(),
            assignees: Vec::new(),
            labels: Vec::new(),
            projects: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_issue_link() {
        let repository = "k-nasa/menbei";
        let title = "title";
        let body = "hogehoge";
        let assignees = vec!["k-nasa", "hoge"];
        let labels = vec!["bug", "question"];
        let projects = vec!["k-nasa/menbei/1"];

        let issue_link = IssueLink::new(repository, title, body, assignees, labels, projects);
        let link = issue_link.generate_link();

        assert_eq!(link, Ok("https://github.com/k-nasa/menbei/issues/new?title=title&body=hogehoge&assignees=k-nasa,hoge&labels=bug,question&projects=k-nasa/menbei/1".to_string()))
    }

    #[test]
    fn generator_issue_link_repository_is_must() {
        let empty_repository = "";

        let title = "title";
        let body = "nogenoge";
        let assignees = vec!["k-nasa", "hoge"];
        let labels = vec!["bug", "question"];
        let projects = vec!["k-nasa/menbei/1"];

        let issue_link = IssueLink::new(empty_repository, title, body, assignees, labels, projects);
        let link = issue_link.generate_link();

        assert_eq!(link, Err("repository is required!".to_string()));
    }

    #[test]
    fn generator_issue_link_repository_is_invalid() {
        let invalid_repository = "hoge";

        let title = "title";
        let body = "nogenoge";
        let assignees = vec!["k-nasa", "hoge"];
        let labels = vec!["bug", "question"];
        let projects = vec!["k-nasa/menbei/1"];

        let issue_link =
            IssueLink::new(invalid_repository, title, body, assignees, labels, projects);
        let link = issue_link.generate_link();

        assert_eq!(link, Err("repository is invalid!".to_string()));
    }

    #[test]
    fn issue_link_default() {
        let issue_link = IssueLink::default();

        assert_eq!(
            issue_link,
            IssueLink {
                repository: String::new(),
                title: String::new(),
                body: String::new(),
                assignees: Vec::new(),
                labels: Vec::new(),
                projects: Vec::new(),
            }
        );
    }

    #[test]
    fn from_yaml() {
        let yaml = r#"
            repository: k-nasa/menbei
            title: title
            body: hogehoge
            assignees:
                - k-nasa
                - hoge
            labels:
                - bug
                - question
            projects:
                - k-nasa/menbei/1
        "#;

        let issue_link: IssueLink = serde_yaml::from_str(&yaml).unwrap();
        let link = issue_link.generate_link();

        assert_eq!(link, Ok("https://github.com/k-nasa/menbei/issues/new?title=title&body=hogehoge&assignees=k-nasa,hoge&labels=bug,question&projects=k-nasa/menbei/1".to_string()))
    }

    #[test]
    fn from_toml() {
        let yaml = r#"
            repository = "k-nasa/menbei"
            title = "title"
            body = "hogehoge"
            assignees = ["k-nasa", "hoge"]
            labels = ["bug", "question"]
            projects = ["k-nasa/menbei/1"]
        "#;

        let issue_link: IssueLink = toml::from_str(&yaml).unwrap();
        let link = issue_link.generate_link();

        assert_eq!(link, Ok("https://github.com/k-nasa/menbei/issues/new?title=title&body=hogehoge&assignees=k-nasa,hoge&labels=bug,question&projects=k-nasa/menbei/1".to_string()))
    }
}
