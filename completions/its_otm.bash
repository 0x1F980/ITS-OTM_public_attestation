# Bash completion for its_otm — OTM public attestation CLI

_its_otm_completions() {
    local cur prev
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    if [[ ${COMP_CWORD} -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "keygen sign verify demo help -h --help" -- "$cur") )
        return 0
    fi

    case "${COMP_WORDS[1]}" in
        keygen)
            case "$prev" in
                --out)
                    COMPREPLY=( $(compgen -f -- "$cur") $(compgen -W "-" -- "$cur") )
                    ;;
                *)
                    COMPREPLY=( $(compgen -W "--out" -- "$cur") )
                    ;;
            esac
            ;;
        sign)
            case "$prev" in
                --state|--in|--out)
                    COMPREPLY=( $(compgen -f -- "$cur") $(compgen -W "-" -- "$cur") )
                    ;;
                *)
                    COMPREPLY=( $(compgen -W "--state --in --out" -- "$cur") )
                    ;;
            esac
            ;;
        verify)
            case "$prev" in
                --bundle|--payload)
                    COMPREPLY=( $(compgen -f -- "$cur") )
                    ;;
                *)
                    COMPREPLY=( $(compgen -W "--bundle --payload" -- "$cur") )
                    ;;
            esac
            ;;
        *)
            ;;
    esac
}

complete -F _its_otm_completions its_otm
