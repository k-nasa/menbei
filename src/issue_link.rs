pub struct IssueLink {
    repository: Option<String>,
    title: Option<String>,
    body: Option<String>,
    assignees: Vec<String>,
    labels: Vec<String>,
    projects: Vec<String>,
}

impl Default for IssueLink {
    fn default() -> Self {
        IssueLink {
            repository: None,
            title: None,
            body: None,
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
                repository: None,
                title: None,
                body: None,
                assignees: Vec::new(),
                labels: Vec::new(),
                projects: Vec::new(),
            }
        );
    }
}
