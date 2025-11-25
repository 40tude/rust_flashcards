param(
    [Parameter(Mandatory=$true)]
    [string]$FilePath
)

if (-not (Test-Path $FilePath)) {
    Write-Error "File not found: $FilePath"
    exit 1
}

$content = Get-Content $FilePath -Encoding UTF8
$lineNumber = 0
$nonCompliantQuestions = @()

foreach ($line in $content) {
    $lineNumber++
    
    # Check if line starts with "Question :"
    if ($line -match '^Question\s*:') {
        # Check if it follows the pattern: Question : CATEGORY - SUBCATEGORY - ...
        # Pattern: "Question :" followed by text, then " - " (space-dash-space), then text, then " - " (space-dash-space)
        if ($line -notmatch '^Question\s*:\s*.+\s-\s.+\s-\s.+') {
            $nonCompliantQuestions += [PSCustomObject]@{
                Line = $lineNumber
                Content = $line.Trim()
            }
        }
    }
}

if ($nonCompliantQuestions.Count -eq 0) {
    Write-Host "✓ All questions follow the pattern 'Question : CATEGORY - SUBCATEGORY - ...'" -ForegroundColor Green
} else {
    Write-Host "Found $($nonCompliantQuestions.Count) non-compliant questions:" -ForegroundColor Yellow
    Write-Host ""
    foreach ($q in $nonCompliantQuestions) {
        Write-Host "Line $($q.Line):" -ForegroundColor Cyan
        Write-Host "  $($q.Content)" -ForegroundColor White
        Write-Host ""
    }
}
