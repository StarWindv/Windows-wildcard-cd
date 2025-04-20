function Invoke-rscd {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Pattern
    )
    
    $target_dir = rcd $Pattern 2>&1
    
    if ($LASTEXITCODE -eq 0) {
      
        Set-Location $target_dir
    } else {
        Write-Error $target_dir
    }
}
New-Alias -Name rscd -Value Invoke-rscd -Force
Set-Alias -Name rscd -Value Invoke-rscd -Option AllScope
