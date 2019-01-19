use crate::issue_link::IssueLink;
use regex::Regex;
use std::io::{stdin, stdout, Write};

pub fn exec() {
    print_message("Input repository name: ");
    let repository = read::<String>();

    if !validate_issue_link(&repository) {
        return println!("repository is invalid!");
    }

    print_message("Input title: ");
    let title = read::<String>();

    print_message("Input body: ");
    let body = read::<String>();

    print_message("Input assignees: ");
    let assignees = read::<String>().split(",").map(|x| x.to_string()).collect();

    print_message("Input labels: ");
    let labels = read::<String>().split(",").map(|x| x.to_string()).collect();

    print_message("Input projects: ");
    let projects = read::<String>().split(",").map(|x| x.to_string()).collect();

    let issue_link = IssueLink::new(repository, title, body, assignees, labels, projects);

    match issue_link.print_link() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn print_message(s: &str) {
    print!("{}", s);
    stdout().flush().expect("print! is faild");
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn validate_issue_link(repository: &str) -> bool {
    if repository.is_empty() {
        return false;
    }

    let repo_pattern = Regex::new(r".*/.*").unwrap();
    if !repo_pattern.is_match(repository) {
        return false;
    }

    true
}
