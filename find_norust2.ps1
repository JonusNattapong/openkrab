$files = Get-ChildItem -Path "D:\Projects\Github\openkrab\docs" -Recurse -Filter "*.md"
$noRust = @()
foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
    if ($content -notmatch "rust|Rust") {
        $relPath = $file.FullName.Replace("D:\Projects\Github\openkrab\docs\", "")
        $noRust += $relPath
    }
}
Write-Host "Total files without 'rust' or 'Rust': $($noRust.Count)"
Write-Host ""
$noRust | ForEach-Object { Write-Host $_ }
