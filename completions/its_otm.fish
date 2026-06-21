# Fish completion for its_otm — OTM public attestation CLI

complete -c its_otm -f

complete -c its_otm -n "__fish_use_subcommand" -a keygen -d "Create signer state file"
complete -c its_otm -n "__fish_use_subcommand" -a sign -d "Sign payload to attestation bundle"
complete -c its_otm -n "__fish_use_subcommand" -a verify -d "Verify attestation bundle"
complete -c its_otm -n "__fish_use_subcommand" -a demo -d "Print and verify demo bundle"
complete -c its_otm -n "__fish_use_subcommand" -a help -d "Show usage"

complete -c its_otm -n "__fish_seen_subcommand_from keygen" -l out -r -F -d "Signer state output path (- for stdout)"

complete -c its_otm -n "__fish_seen_subcommand_from sign" -l state -r -F -d "Signer state file"
complete -c its_otm -n "__fish_seen_subcommand_from sign" -l in -r -F -d "Payload to sign (- for stdin)"
complete -c its_otm -n "__fish_seen_subcommand_from sign" -l out -r -F -d "Attestation bundle output (- for stdout)"

complete -c its_otm -n "__fish_seen_subcommand_from verify" -l bundle -r -F -d "Attestation bundle file"
complete -c its_otm -n "__fish_seen_subcommand_from verify" -l payload -r -F -d "Optional payload to match message field"
