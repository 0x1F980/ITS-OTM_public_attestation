# PowerShell completion for its_otm — OTM public attestation CLI
# Usage: . ./completions/its_otm.ps1

using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'its_otm' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $tokens = @(
        foreach ($element in $commandAst.CommandElements) {
            if ($element -is [StringConstantExpressionAst] -and
                $element.StringConstantType -eq [StringConstantType]::BareWord) {
                $element.Value
            }
        }
    )

    $command = ($tokens -join ';')

    $completions = @(switch -Regex ($command) {
        '^its_otm$' {
            'keygen', 'sign', 'verify', 'demo', 'help', '-h', '--help' | ForEach-Object {
                [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterValue, $_)
            }
            break
        }
        '^its_otm;keygen' {
            '--out' | ForEach-Object {
                [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterName, 'Signer state output path')
            }
            break
        }
        '^its_otm;sign' {
            '--state', '--in', '--out' | ForEach-Object {
                [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterName, $_)
            }
            break
        }
        '^its_otm;verify' {
            '--bundle', '--payload' | ForEach-Object {
                [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterName, $_)
            }
            break
        }
    })

    $completions | Where-Object { $_.CompletionText -like "$wordToComplete*" }
}
