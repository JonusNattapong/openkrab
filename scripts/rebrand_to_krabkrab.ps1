param(
    [string]$SourceDir = "./krabkrab",
    [Parameter(Mandatory=$true)][string]$TargetDir,
    [switch]$DryRun
)

function Write-Log { param($m) Write-Host "[rebrand] $m" }

$src = Resolve-Path $SourceDir
$dst = Resolve-Path -LiteralPath $TargetDir -ErrorAction SilentlyContinue
if (-not $dst) { $dst = $TargetDir }

Write-Log "Source: $src"
Write-Log "Target: $dst"
if ($DryRun) { Write-Log "DRY RUN: no files will be modified." }

# Copy tree to new location (dry-run option)
if ($DryRun) {
    Write-Log "Would copy $src to $dst"
} else {
    Write-Log "Copying files..."
    if (Test-Path $dst) { Write-Log "Target already exists: $dst" }
    else { Copy-Item -Path $src -Destination $dst -Recurse -Force }
}

# Replacement pairs (use array to preserve case variants)
$replacements = @(
    @{ key = 'krabkrab'; value = 'krabkrab' }
    @{ key = 'krabkrab'; value = 'KrabKrab' }
)

# File extensions to skip (binary blobs)
$skipExt = @('.png','.jpg','.jpeg','.gif','.ico','.zip','.tar','.gz','.7z','.dll','.exe','.so','.dylib','.jar','.class','.webp')

# Process files in target folder
$files = Get-ChildItem -Path $dst -Recurse -File -ErrorAction SilentlyContinue
$changed = 0
$checked = 0

foreach ($f in $files) {
    $checked++
    if ($skipExt -contains $f.Extension.ToLower()) { continue }

    try {
        $raw = Get-Content -Raw -LiteralPath $f.FullName -ErrorAction Stop
    } catch {
        continue
    }

    $new = $raw
    foreach ($r in $replacements) {
        $k = $r.key
        $v = $r.value
        $new = $new -replace [regex]::Escape($k), $v
    }

    if ($new -ne $raw) {
        if ($DryRun) {
            Write-Log "Would update: $($f.FullName)"
            $changed++
        } else {
            Set-Content -LiteralPath $f.FullName -Value $new -Encoding utf8
            Write-Log "Updated: $($f.FullName)"
            $changed++
        }
    }
}

# Rename files and directories containing 'krabkrab' in their names
$items = Get-ChildItem -Path $dst -Recurse -Force | Sort-Object -Property FullName -Descending
foreach ($it in $items) {
    $name = $it.Name
    if ($name -match 'krabkrab') {
        $newName = $name -replace 'krabkrab','krabkrab'
        $oldPath = $it.FullName
        $newPath = Join-Path -Path $it.DirectoryName -ChildPath $newName
        if ($DryRun) {
            Write-Log "Would rename: $oldPath -> $newPath"
        } else {
            try { Rename-Item -LiteralPath $oldPath -NewName $newName -Force; Write-Log "Renamed: $oldPath -> $newPath" } catch { Write-Log "Rename failed: $oldPath -> $newPath : $_" }
        }
    }
}

Write-Log "Checked $checked text files; $changed files would be/are changed."
if ($DryRun) { Write-Log "Dry run complete. Remove -DryRun to execute changes." }
else { Write-Log "Rebrand complete - inspect and run tests/builds in $dst." }

