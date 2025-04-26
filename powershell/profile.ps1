function Invoke-Rcd {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Pattern
    )
    
    $output = rcd $Pattern 2>&1 | Out-String
    
    if ($LASTEXITCODE -eq 0) {
        $target_dir = [System.Text.Encoding]::UTF8.GetString(
            [System.Text.Encoding]::GetEncoding('GBK').GetBytes($output)
        ).Trim()
        
        Set-Location $target_dir
    } else {
        Write-Error $output
    }
}

New-Alias -Name rscd -Value Invoke-Rcd -Force

Set-Alias -Name rscd -Value Invoke-Rcd -Option AllScope
