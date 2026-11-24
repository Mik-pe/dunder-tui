# Läs ändrade celler ur två råa buffertar

På macOS ARM64 sjönk mediantiden för en oförändrad gles bild från 40 till 23 µs. För en tät bild sjönk den från 126 till 50 µs. Ratatui 0.30.2 tog 13,1 respektive 16,2 µs. Föregående Dunder-version och den nya kördes med samma mätprogram. [Råvärden före och efter](results/raw-diff-scan-0.12.1.tsv)

Båda programmen jämför färdigritade buffertar med 96 × 24 celler. I den glesa bilden har varje rad en färgad CJK-etikett med ett kombinerande tecken och en status till höger. Den täta bilden fyller varje rad med åtta textstycken och växlar mellan indexerad färg med fetstil och RGB-färger för text, bakgrund och understrykning med kursiv stil. Vid rullning flyttas raderna ett steg. Vid storleksändring jämförs de första 20 raderna med en bild på 24 rader och en ändring på sista gemensamma raden.

Dunder använder `BufferPair.diff_iter()` med `advance()`. Ratatui använder `Buffer::diff_iter` med `underline-color`. Mätningen omfattar hela genomgången och läsning av varje ändrad cells koordinater och attribut. Båda skriver noll byte till terminalen under genomgången. I en separat körning jämförs båda indatabilderna och varje lämnad cell inklusive symbol, färger och attribut byte för byte. [Dunder](raw_diff.ddr) · [Ratatui](reference/examples/raw_diff.rs)

Tabellen visar median / p95 i mikrosekunder per genomgång.

| Arbetslast | Ändrade celler | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Översatt Linux x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 0 | 23 / 32 | 13,1 / 21,7 | 24,7 / 29,9 | 12,3 / 20,4 | 147 / 209,4 | 18,9 / 41,6 |
| En cell | 1 | 24 / 35 | 12,8 / 13,5 | 25,2 / 33 | 11,8 / 12,5 | 147,1 / 332 | 19,4 / 23,4 |
| Rullning | 24 | 26 / 52 | 13,3 / 14,3 | 26,3 / 30,5 | 12,2 / 13,7 | 151,6 / 313,1 | 19,1 / 31 |
| Storleksändring | 1 | 21 / 23 | 10,8 / 11,6 | 22,2 / 27 | 10,3 / 12,8 | 122,2 / 175,8 | 16,7 / 27 |
| Omritning | 2280 | 42 / 88 | 19,5 / 31,1 | 47,6 / 65,6 | 20,7 / 37,4 | 297,2 / 375,9 | 36,5 / 54,6 |
| Tät, oförändrad | 0 | 50 / 129 | 16,2 / 28,2 | 54,2 / 99,4 | 15,1 / 15,6 | 401,2 / 531,8 | 23,8 / 50,7 |
| Tät, en cell | 1 | 51 / 139 | 16,3 / 17,3 | 55,4 / 97,2 | 15,2 / 17,6 | 406,5 / 658,2 | 24,5 / 35,3 |
| Tät, rullning | 2112 | 61 / 120 | 23 / 37,5 | 68,1 / 116 | 24,3 / 37,7 | 511,2 / 967,2 | 35,6 / 53 |
| Tät, omritning | 2112 | 81 / 154 | 24,5 / 40,7 | 90,8 / 166,4 | 24,4 / 27,4 | 656 / 989,2 | 40,1 / 79,9 |

Varje process gjorde tio uppvärmningar och 100 mätningar per arbetslast. På varje plattform kördes två uppvärmningsomgångar och sedan sju mätomgångar med roterande ordning mellan den tidigare Dunder-koden, den nya koden och Ratatui. Tabellen visar medianen av processernas p50 respektive p95. För tät, oförändrad bild på Macen låg processmedianerna mellan 49 och 54 µs för Dunder och mellan 16,0 och 16,7 µs för Ratatui. Maskinen var en Apple M5 med macOS 27.2.1 och en lokal Linux 7.0.14-VM. Linux x64 kördes med Rosetta. Nativa tider på Windows x64 och Linux x64 återstår. [Samtliga 567 processvärden för p50 och p95](results/raw-diff-scan-0.12.1.tsv)

Över nio arbetslaster och 990 genomgångar begärde Dunder 174 110 allokeringar inklusive uppbyggnad. En process som bara byggde buffertarna begärde 170 150. Skillnaden sjönk från 146 080 till 3 960 mellan Dunder-versionerna. Räknaren inkluderar återanvända objekt i Dunders pool. Rust begärde 26 allokeringar med och utan genomgångarna på Macen och 24 på Linux. Medianen för högsta RSS över fem oprofilerade processer var 2 512 / 2 288 KiB på macOS, 768 / 1 840 KiB på Linux ARM64 och 6 204 / 5 264 KiB på översatt Linux x64, med Dunder först. [Allokeringsräknare och minnesprov](results/raw-diff-scan-resources-0.12.1.tsv)

De första fem arbetslasterna finns också uppmätta före ändringen med det ursprungliga glesa mätprogrammet. [Tidigare tider](results/raw-diff-dunder-0.12.1.tsv) · [Tidigare resursvärden](results/raw-diff-resources-0.12.1.tsv)

## Kör lokalt

Mätningarna använder Dunder 0.12.1 och Rust 1.99.1. Lägg den låsta referensens Rust-paket i Cargo-cachen före offlinekörningen.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/raw_diff.ddr -o build/raw
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak
cargo build --release --locked --offline --no-default-features --features theme-oracle \
  --manifest-path benchmarks/reference/Cargo.toml --example raw_diff --target-dir build/reference
ref="$PWD/build/reference/release/examples/raw_diff"
build/raw --cells >build/dunder-cells
"$ref" --cells >build/rust-cells
cmp build/dunder-cells build/rust-cells

baseline=$(mktemp -d)
git archive bf2ccc57aa919cd85cb1ad418c6b1364632dbce9 | tar -x -C "$baseline"
cp benchmarks/raw_diff.ddr "$baseline/benchmarks/raw_diff.ddr"
output="$PWD/build/raw-before"
(cd "$baseline" && dunder build --locked --offline benchmarks/raw_diff.ddr -o "$output")
for run in 1 2 3 4 5 6 7 8 9; do
    case $((run % 3)) in
        0) set -- "$output" build/raw "$ref" ;;
        1) set -- build/raw "$ref" "$output" ;;
        2) set -- "$ref" "$output" build/raw ;;
    esac
    for exe do "$exe"; done
done
build/peak "$PWD/build/raw" --allocations
build/peak "$ref" --allocations
dunder build --locked --offline --runtime-profile benchmarks/raw_diff.ddr -o build/profile
for mode in --setup --allocations; do
    build/profile "$mode"
    cargo run --release --locked --offline --no-default-features --features theme-oracle,allocation-counts \
      --manifest-path benchmarks/reference/Cargo.toml --example raw_diff -- "$mode"
done
```
