pub struct IssueLink {
    organization: Option<String>,
    repository: Option<String>,
    title: Option<String>,
    body: Option<String>,
    assignees: Vec<String>,
    labels: Vec<String>,
    projects: Vec<String>,
}
