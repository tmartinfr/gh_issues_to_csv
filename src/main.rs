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
        let labels: Vec<String> = issue
            .labels
            .iter()
            .map(|label| label.name.to_string())
            .collect();
        // Gather CSV fields
        let output = vec![
            issue.title.to_string(),
            labels.join(", "),
            issue.created_at.to_string(),
            issue.html_url.to_string(),
            issue.body.to_string(),
        ];
        // Escape and add quotes around each field
        let output: Vec<String> = output
            .iter()
            .map(|field| field.to_string().replace("\"", "\"\""))
            .map(|field| ["\"", &field, "\""].concat())
            .collect();
        // Build final line
        let output = output.join(",") + "\n";

        // Don't use println! here because it fails under SIGPIPE
        let _ = io::stdout().write(output.as_bytes());
    }
}
