# Läs ändrade celler ur två råa buffertar

Båda programmen jämför färdigritade buffertar med 96 × 24 celler. Varje rad innehåller CJK, ett kombinerande tecken och en statusetikett. Dunder använder `BufferPair.diff_iter()` med `advance()`. Ratatui 0.30.2 använder `Buffer::diff_iter` med `underline-color`. För varje lämnad cell läser båda koordinater och attribut till samma kontrollsumma. Mätningen börjar efter att buffertarna byggts och slutar när iteratorn är uttömd. Terminalutdata är noll byte. [Dunder](raw_diff.ddr) · [Ratatui](reference/examples/raw_diff.rs)

Tabellen visar median / p95 i mikrosekunder per genomgång. Rullning flyttar de fyra radtexterna ett steg. Vid storleksändring jämförs de första 20 raderna med en bild på 24 rader och en ändring på sista gemensamma raden. Vid omritning växlar hela bufferten färg. De breda tecknen upptar två kolumner, så iteratorerna lämnar 2 280 av de 2 304 fysiska cellerna.

| Arbetslast | Ändrade celler | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Översatt Linux x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 0 | 39 / 79 | 12,1 / 16,7 | 42,5 / 75,7 | 12,2 / 13,3 | 349 / 505 | 19,4 / 33,5 |
| En cell | 1 | 39 / 58 | 12,0 / 12,7 | 43,2 / 52,3 | 12,0 / 12,8 | 349 / 483 | 19,5 / 31,8 |
| Rullning | 24 | 40 / 76 | 11,7 / 12,5 | 43,8 / 65,0 | 11,7 / 18,5 | 353 / 531 | 20,2 / 38,5 |
| Storleksändring | 1 | 33 / 52 | 9,6 / 16,0 | 36,3 / 50,6 | 10,0 / 10,1 | 294 / 540 | 16,5 / 26,2 |
| Omritning | 2 280 | 55 / 82 | 17,8 / 19,2 | 65,5 / 102 | 18,1 / 25,6 | 507 / 695 | 38,3 / 58,5 |

Varje mål fick två uppvärmningspar och sju mätpar med växlande körordning. Varje process gjorde tio uppvärmningar och 100 mätningar per arbetslast. Tabellen visar medianen av de sju processernas p50 respektive p95. Vid omritning på Macen låg processmedianerna mellan 54 och 57 µs för Dunder och mellan 17,2 och 18,5 µs för Ratatui. Maskinen var en Apple M5 med macOS 27.2.1 och en lokal Linux 7.0.14-VM. Linux x64 kördes med Rosetta. Nativa tider på Windows x64 och Linux x64 återstår för denna jämförelse. [Alla 210 mätpar av p50 och p95](results/raw-diff-dunder-0.12.1.tsv)

Över fem arbetslaster och 550 genomgångar begärde Dunder 92 344 allokeringar inklusive uppbyggnad. En separat process som bara byggde buffertarna begärde 74 744. Dunders räknare omfattar också återanvända objekt i minnespoolen. Rust begärde 18 allokeringar i båda körningarna på Macen och 16 på Linux. Medianen för högsta RSS över fem oprofilerade körningar var 2 336 / 2 160 KiB på macOS, 640 / 1 716 KiB på Linux ARM64 och 5 928 / 5 280 KiB på översatt Linux x64, med Dunder först. [Räknare och minnesprov](results/raw-diff-resources-0.12.1.tsv)

## Kör lokalt

Mätningarna använder Dunder 0.12.1 och Rust 1.99.1. Lägg den låsta referensens Rust-paket i Cargo-cachen före offlinekörningen.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/raw_diff.ddr -o build/raw
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak
cargo build --release --locked --offline --no-default-features --features theme-oracle \
  --manifest-path benchmarks/reference/Cargo.toml --example raw_diff --target-dir build/reference
ref="$PWD/build/reference/release/examples/raw_diff"
build/raw --allocations >build/dunder-check
"$ref" --allocations >build/rust-check
cmp build/dunder-check build/rust-check
for run in 1 2 3 4 5 6 7 8 9; do
    if test $((run % 2)) -eq 1; then build/raw && "$ref"; else "$ref" && build/raw; fi
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
