#[derive(Eq, PartialEq, Debug)]
pub struct IssueLink {
    repository: String,
    title: String,
    body: String,
    assignees: Vec<String>,
    labels: Vec<String>,
    projects: Vec<String>,
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
