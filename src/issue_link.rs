use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
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

    pub fn generate_link(&self) -> Result<String, String> {
        if self.repository.is_empty() {
            return Err("repository is required!".to_string());
        }

        let repo_pattern = Regex::new(r".*/.*").unwrap();
        if !repo_pattern.is_match(&self.repository) {
            return Err("repository is invalid!".to_string());
        }

        Ok("https://github.com/k-nasa/menbei/issues/new?title=title&body=hogehoge&assignees=k-nasa,hoge&labels=bug,question&projects=k-nasa/menbei/1".to_string())
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
}
