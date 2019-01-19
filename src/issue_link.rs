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
