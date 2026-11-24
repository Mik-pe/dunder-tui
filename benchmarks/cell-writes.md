# Skriv direkt till en rå cellbuffert

Varje tidsprov uppdaterar 96 × 24 celler i minnet. Textproven skriver tolv kopior av `1234567.` eller `A茶é🦀..` på varje rad och växlar färg och attribut mellan bilderna. Spannprovet återanvänder ett stiliserat Unicode-spann för samtliga rader. Stilprovet ändrar bara cellernas stil. Alla fyra kontrollerar sista cellens symbol och stil. Terminalutdata är noll byte under ritningen. [Dunder](cell_write.ddr) · [Ratatui](reference/examples/cell_edits.rs)

Tabellen visar p50 / p95 i mikrosekunder per bild. För varje mål kördes två uppvärmningspar och sju mätpar med växlande ordning. Varje process ritade tio uppvärmningsbilder och mätte sedan 100 per arbetslast. Varje kolumn visar medianen av de sju processvärdena. Mätningarna använde Apple M5, macOS 27.2.1, Linux 7.0.14 i en lokal ARM64-VM, Dunder 0.12.1 och Rust 1.99.1 i releaseprofil. Linux x86-64 kördes med Rosetta i samma VM. [Alla 168 mätpar av p50 och p95](results/cell-writes-0.12.1.tsv)

| Arbetslast | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Översatt Linux x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| ASCII | 21 / 28 | 31,8 / 46,3 | 20,5 / 26,5 | 33,8 / 48,2 | 310 / 460 | 52,4 / 97,9 |
| Unicode | 213 / 368 | 36,5 / 39,0 | 215 / 328 | 37,0 / 58,7 | 2 079 / 2 661 | 60,6 / 86,8 |
| Förberett spann | 57 / 82 | 34,3 / 52,3 | 57,4 / 96,8 | 35,0 / 55,8 | 637 / 1 043 | 52,0 / 123 |
| Stil | 12 / 13 | 3,4 / 3,5 | 12,9 / 14,5 | 3,4 / 3,6 | 176 / 233 | 8,0 / 8,6 |

På Macen låg de sju processmedianerna för direkt Unicode-skrivning mellan 208 och 215 µs i Dunder och mellan 36,3 och 38,5 µs i Ratatui. Ett förberett spann sänkte Dunders intervall till 55–65 µs. ASCII gav 20–21 µs mot 30,7–32,4 µs. Nativ x64 och Windows återstår för denna arbetslast.

Dunders profilerade process begärde 317 292 allokeringar på totalt 7 387 268 byte för uppbyggnad och 440 uppdateringar. Rustreferensen begärde 13 allokeringar på Macen och 12 på Linux. I fem separata körningar av binärerna utan profilering var medianen för högsta RSS 2 048 / 2 112 KiB på Macen, 384 / 1 852 KiB på Linux ARM64 och 5 736 / 5 084 KiB på översatt Linux x64, med Dunder först. [Räknare och alla RSS-värden](results/cell-write-resources-0.12.1.tsv)

## Kör lokalt

```sh
mkdir -p build
dunder build --locked --offline benchmarks/cell_write.ddr -o build/write
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features theme-oracle --example cell_edits --target-dir build/reference
ref="$PWD/build/reference/release/examples/cell_edits"
for n in 1 2 3 4 5 6 7 8 9; do
    if test $((n % 2)) -eq 1; then build/write && "$ref" --bench; else "$ref" --bench && build/write; fi
done
build/peak "$PWD/build/write" --allocations
build/peak "$ref" --allocations
dunder build --locked --offline --runtime-profile benchmarks/cell_write.ddr -o build/write-profile
build/write-profile --allocations
cargo run --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features theme-oracle,allocation-counts --example cell_edits -- --allocations
```
