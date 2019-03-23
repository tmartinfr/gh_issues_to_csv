Simple Rust program which reads a `GitHub Issues API <https://developer.github.com/v3/issues/>`_ JSON payload and outputs CSV.

Usage : ::

    cargo build
    curl "https://api.github.com/issues?per_page=100" -u $USER:$TOKEN | ./target/debug/gh_issues_to_csv
