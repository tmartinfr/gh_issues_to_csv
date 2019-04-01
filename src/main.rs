use serde::Deserialize;
use std::io::{self, Write};

#[derive(Deserialize)]
struct Label {
    name: String,
}

#[derive(Deserialize)]
struct Issue {
    title: String,
    created_at: String,
    html_url: String,
    body: String,
    labels: Vec<Label>,
}

fn main() {
    let issues: Vec<Issue> = serde_json::from_reader(io::stdin()).unwrap();

    for issue in issues.iter() {
        // Extract issue's labels
        let labels = issue
            .labels
            .iter()
            .map(|label| label.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        // Gather CSV fields
        let output = vec![
            &issue.title,
            &labels,
            &issue.created_at,
            &issue.html_url,
            &issue.body,
        ];
        // Escape and add quotes around each field
        let output: Vec<String> = output
            .iter()
            .map(|field| field.replace("\"", "\"\""))
            .map(|field| format!("\"{}\"", &field))
            .collect();
        // Build final line
        let output = output.join(",") + "\n";

        // Don't use println! here because it fails under SIGPIPE
        let _ = io::stdout().write(output.as_bytes());
    }
}
