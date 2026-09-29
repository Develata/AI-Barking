param([string]$Name,[string]$Url,[string[]]$CliArgs)
$start=[DateTimeOffset]::UtcNow.ToOffset([TimeSpan]::FromHours(8)).ToString('yyyy-MM-dd HH:mm:ss zzz')
$env:OPENCLI_BROWSER_COMMAND_TIMEOUT='150'
$out=& opencli @CliArgs 2>&1 | Out-String
$code=$LASTEXITCODE
$out | Set-Content -LiteralPath "docs/0929/sources/$Name" -Encoding utf8
[pscustomobject]@{time_bjt=$start;url=$Url;tool=('opencli '+($CliArgs -join ' '));exit=$code;file=$Name} | ConvertTo-Json -Compress | Add-Content docs/0929/sources/capture-records.jsonl
Write-Output "$Name exit=$code"
Write-Output $out
