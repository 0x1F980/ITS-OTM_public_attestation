#compdef its_otm

_its_otm() {
    local line

    _arguments -C \
        '1:command:->cmds' \
        '*::args:->args'

    case $state in
        cmds)
            _values "its_otm command" \
                'keygen[Create signer state file]' \
                'sign[Sign payload to attestation bundle]' \
                'verify[Verify attestation bundle]' \
                'demo[Print and verify demo bundle]' \
                'help[Show usage]'
            ;;
        args)
            case $line[1] in
                keygen)
                    _arguments '--out[Signer state output path]:file:_files'
                    ;;
                sign)
                    _arguments \
                        '--state[Signer state file]:file:_files' \
                        '--in[Payload to sign]:file:_files' \
                        '--out[Attestation bundle output]:file:_files'
                    ;;
                verify)
                    _arguments \
                        '--bundle[Attestation bundle file]:file:_files' \
                        '--payload[Optional payload to match]:file:_files'
                    ;;
            esac
            ;;
    esac
}

_its_otm "$@"
