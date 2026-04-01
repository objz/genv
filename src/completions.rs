use std::io::{self, Write};

pub fn generate_bash(out: &mut impl Write) -> io::Result<()> {
    write!(
        out,
        r#"_envm() {{
    local cur prev
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"

    if [[ ${{COMP_CWORD}} -eq 1 ]]; then
        COMPREPLY=($(compgen -W "add edit remove list export completions help" -- "${{cur}}"))
        return
    fi

    case "${{COMP_WORDS[1]}}" in
        edit|remove)
            if [[ ${{COMP_CWORD}} -eq 2 ]]; then
                local keys
                keys=$(grep '=' "$HOME/.config/envm/env" 2>/dev/null | cut -d= -f1)
                COMPREPLY=($(compgen -W "${{keys}}" -- "${{cur}}"))
            fi
            ;;
        export)
            if [[ "${{prev}}" == "--shell" ]]; then
                COMPREPLY=($(compgen -W "posix fish" -- "${{cur}}"))
            else
                COMPREPLY=($(compgen -W "--shell" -- "${{cur}}"))
            fi
            ;;
        completions)
            if [[ ${{COMP_CWORD}} -eq 2 ]]; then
                COMPREPLY=($(compgen -W "bash zsh fish" -- "${{cur}}"))
            fi
            ;;
    esac
}}
complete -F _envm envm
"#
    )
}

pub fn generate_zsh(out: &mut impl Write) -> io::Result<()> {
    write!(
        out,
        r#"#compdef envm

_envm_keys() {{
    local -a keys
    keys=(${{(f)"$(grep '=' "$HOME/.config/envm/env" 2>/dev/null | cut -d= -f1)"}})
    compadd -a keys
}}

_envm() {{
    local -a commands
    commands=(
        'add:Add a new variable'
        'edit:Edit an existing variable'
        'remove:Remove a variable'
        'list:List variables managed by this file'
        'export:Print exports for the current shell'
        'completions:Generate shell completions'
        'help:Print help information'
    )

    _arguments -C \
        '--file=[Override the managed env file]:file:_files' \
        '--help[Print help]' \
        '--version[Print version]' \
        '1:command:->cmd' \
        '*::arg:->args'

    case $state in
        cmd)
            _describe -t commands 'envm command' commands
            ;;
        args)
            case $words[1] in
                add)
                    _arguments '1:key:' '2:value:'
                    ;;
                edit)
                    _arguments '1:key:_envm_keys' '2:value:'
                    ;;
                remove)
                    _arguments '1:key:_envm_keys'
                    ;;
                export)
                    _arguments '--shell=[Shell style]:shell:(posix fish)'
                    ;;
                completions)
                    _arguments '1:shell:(bash zsh fish)'
                    ;;
            esac
            ;;
    esac
}}

_envm "$@"
"#
    )
}

pub fn generate_fish(out: &mut impl Write) -> io::Result<()> {
    write!(
        out,
        r#"complete -c envm -e

complete -c envm -n '__fish_use_subcommand' -l file -d 'Override the managed env file' -r -F
complete -c envm -n '__fish_use_subcommand' -l help -d 'Print help'
complete -c envm -n '__fish_use_subcommand' -l version -d 'Print version'

complete -c envm -n '__fish_use_subcommand' -a add -d 'Add a new variable'
complete -c envm -n '__fish_use_subcommand' -a edit -d 'Edit an existing variable'
complete -c envm -n '__fish_use_subcommand' -a remove -d 'Remove a variable'
complete -c envm -n '__fish_use_subcommand' -a list -d 'List variables'
complete -c envm -n '__fish_use_subcommand' -a export -d 'Print exports for the current shell'
complete -c envm -n '__fish_use_subcommand' -a completions -d 'Generate shell completions'
complete -c envm -n '__fish_use_subcommand' -a help -d 'Print help information'

complete -c envm -n '__fish_seen_subcommand_from edit remove' -xa '(grep "=" "$HOME/.config/envm/env" 2>/dev/null | string split -f1 =)'

complete -c envm -n '__fish_seen_subcommand_from export' -l shell -xa 'posix fish'

complete -c envm -n '__fish_seen_subcommand_from completions' -xa 'bash zsh fish'
"#
    )
}
