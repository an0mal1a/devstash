pub fn main() {
    println!(
        r#"
DevStash - simple CLI snippet manager

USAGE:
    devstash <command> [options]

COMMANDS:
    add [amount]        Add one or more snippets interactively
    list [limit]        List snippets. Default limit: 10
    show <id>           Show a snippet by ID
    delete <id>         Delete a snippet by ID
    help                Show this help panel

SNIPPET TYPES:
    command             Terminal commands
    json                JSON payloads
    note                Free text notes

EXAMPLES:
    devstash add
    devstash add 3
    devstash list
    devstash list 20
    devstash show 1
    devstash delete 1

TAGS:
    When adding a snippet, write tags separated by commas:
    rust,cli,json

DATA:
    Snippets are stored in:
    snippets.json
"#
    );
}