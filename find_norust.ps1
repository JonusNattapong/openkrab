$files = Get-ChildItem -Path "D:\Projects\Github\openkrab\docs" -Recurse -Filter "*.md"
$noRust = @()
foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw -ErrorAction SilentlyContinue
    if ($content -notmatch "rust|Rust") {
        $noRust += $file.FullName
    }
}
$noRust | ForEach-Object { $_ }
