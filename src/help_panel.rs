pub fn main() {
    println!(
        r#"
DevStash
Simple CLI snippet manager

USAGE:
    devstash <command> [options]

COMMANDS:
    add [amount]           Add one or more snippets interactively
    list [limit]           List snippets. Default limit: 10
    show <id>              Show a snippet by ID
    delete <id>            Delete a snippet by ID
    search <query>         Search snippets by title or content
    tag <tag1 tag2 ...>    Search snippets by tags
    export <path>          Export the json snippets file
    import <path>          Import the json snippets file (experimental)
    help                   Show this help panel

SNIPPET TYPES:
    command                Terminal commands
    json                   JSON payloads
    note                   Free text notes

EXAMPLES:
    devstash add
    devstash add 3
    devstash list
    devstash list 20
    devstash show 1
    devstash delete 1
    devstash search cooking recipe
    devstash tag vscode 3d
    devstash export C:\Users\username\backups\exported_snippets.json
    devstash import C:\Users\username\backups\exported_snippets_02.json

TAGS:
    When adding a snippet, write tags separated by commas:
        rust,cli,json

DATA:
    snippets.json
"#
    );
}
