param($dayNumber)

$dayNumber = $dayNumber.ToString("00");
Write-Output $dayNumber

mkdir "day$dayNumber"
Copy-Item .\day0x\* -Recurse -Container -Exclude target, Cargo.lock "day$dayNumber"
Push-Location "day$dayNumber"

Get-ChildItem *day0x* -Recurse | ForEach-Object {
    Rename-Item $_ -NewName ($_ -replace "day0x", "day$dayNumber")
}

Get-ChildItem *.toml, *.rs -Recurse | ForEach-Object { 
    sd day0x "day$dayNumber" $_;
}

cargo build
cargo test
cargo bench --bench "bench_day$dayNumber" -- --quick