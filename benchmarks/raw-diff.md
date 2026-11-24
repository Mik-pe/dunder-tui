# Läs ändrade celler ur två råa buffertar

På macOS ARM64 sjönk mediantiden för en oförändrad tät bild från 44,8 till 38,2 µs och för en tät omritning från 72,3 till 61,1 µs. Ratatui 0.30.2 tog 14,6 respektive 21,8 µs. Buffertdiffen lånar nu båda buffertarna direkt och beräknar föregående symbolbredd först när en tidigare stil kan synas på en tom cell. Av 1 088 Unicode-sidor har 1 031 samma egenskaper i hela sidan. Dessa läses med ett uppslag, liksom de första 1 024 kodpunkterna. [Råvärden före och efter](results/raw-diff-pages-0.12.1.tsv)

Båda programmen jämför färdigritade bilder med 96 × 24 celler. Den glesa bilden har en färgad CJK-etikett med ett kombinerande tecken till vänster och en status till höger. Den täta bilden växlar mellan indexerad färg med fetstil och RGB-färger för text, bakgrund och understrykning med kursiv stil. Rullning flyttar innehållet en rad. Vid storleksändring har den förra bilden 20 rader och den nya 24, med en ändring på sista gemensamma raden. [Dunder](raw_diff.ddr) · [Ratatui](reference/examples/raw_diff.rs)

Dunder använder `BufferPair.diff_iter().advance()` och Ratatui använder `Buffer::diff_iter` med understrykningsfärg. Båda läser varje ändrad cells koordinater och attribut. De skriver noll terminalbyte under genomgången. Ett separat prov jämför båda indatabilderna och varje ändrad cell i ordning inklusive symbol, färger och attribut. Tabellen visar median / p95 i mikrosekunder per genomgång.

| Arbetslast | Ändrade celler | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Översatt Linux x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 0 | 19,2 / 20,1 | 11,8 / 12,6 | 21,0 / 21,6 | 11,5 / 12,2 | 119,5 / 125,8 | 17,8 / 19,4 |
| En cell | 1 | 20,4 / 21,4 | 10,7 / 11,2 | 21,4 / 21,9 | 10,4 / 10,5 | 119,6 / 125,7 | 17,2 / 17,5 |
| Rullning | 24 | 21,2 / 22,4 | 11,0 / 12,1 | 21,7 / 22,1 | 10,8 / 10,9 | 122,0 / 127,6 | 17,6 / 17,7 |
| Storleksändring | 1 | 17,2 / 18,1 | 9,0 / 9,4 | 17,9 / 18,5 | 8,7 / 9,1 | 99,8 / 105,1 | 14,5 / 14,9 |
| Omritning | 2280 | 32,2 / 33,5 | 17,8 / 18,1 | 36,5 / 38,0 | 18,5 / 18,6 | 236,5 / 241,8 | 32,0 / 33,0 |
| Tät, oförändrad | 0 | 38,2 / 40,2 | 14,6 / 16,1 | 41,5 / 42,5 | 12,8 / 13,7 | 302,1 / 306,4 | 21,8 / 24,5 |
| Tät, en cell | 1 | 38,8 / 41,2 | 14,6 / 16,4 | 41,7 / 42,7 | 13,0 / 13,5 | 302,8 / 307,4 | 21,5 / 22,3 |
| Tät, rullning | 2112 | 44,7 / 47,2 | 20,3 / 22,8 | 48,3 / 49,5 | 21,7 / 22,5 | 350,5 / 356,5 | 32,1 / 32,7 |
| Tät, omritning | 2112 | 61,1 / 64,2 | 21,8 / 22,3 | 65,7 / 67,5 | 21,9 / 22,3 | 464,1 / 470,4 | 34,5 / 40,5 |

Dunder 0.12.1 och Rust 1.99.1 kördes på en Apple M5 med macOS 27.2.1 och en lokal Linux-VM. Rosetta körde Linux x64. Alla tre implementationer kördes i roterande ordning. Efter två uppvärmningsomgångar följde sju omgångar med tio uppvärmningar och 100 mätningar per process och arbetslast. Tabellen visar medianen av de sju processernas p50 respektive p95. Båda Dunder-versionerna använde samma mätprogram med `MonotonicClock.Raw`. För tät omritning på Macen låg processmedianerna på 59,4–62,5 µs för Dunder och 21,4–22,0 µs för Ratatui. Mätningar på nativ Linux x64 och Windows x64 återstår. [Samtliga 567 mätpar](results/raw-diff-pages-0.12.1.tsv)

Över 990 genomgångar sjönk antalet extra allokeringsanrop från 3 960 till 2 970. Mätningen drar bort en separat process som endast byggde buffertarna. Rust hade samma antal allokeringsanrop i båda körningarna. Medianen för högsta RSS över fem processer för Dunder och Ratatui var 2 512 och 2 160 KiB på macOS ARM64, 1 580 och 1 852 KiB på Linux ARM64 samt 5 224 och 5 264 KiB på översatt Linux x64. [Allokeringsräknare och minnesprov](results/raw-diff-pages-resources-0.12.1.tsv) · [Tidigare mätserie](results/raw-diff-scan-0.12.1.tsv) · [Tidigare minnesprov](results/raw-diff-scan-resources-0.12.1.tsv)

I macOS pseudoterminal skrev Dunder 2 064 494 byte för emojiloggen och 1 382 478 byte för ligaturloggen över 1 100 bilder vardera. Båda byteströmmarna var identiska före och efter ändringen. Ratatui skrev 2 280 403 respektive 1 624 225 byte och verifierade cellerna i samtliga 2 200 bilder. [Terminalprov](terminal.ddr) · [Referenskontroll](reference/examples/terminal_bench.rs) · [Körinstruktioner för pseudoterminalen](emoji.md)

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
git archive fa3e99a60c8d3a4aa611408d852b0a5b5e58b62a | tar -x -C "$baseline"
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
