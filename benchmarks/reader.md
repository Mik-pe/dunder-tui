# Läsare med ram i Dunder 0.12.1

Vyn visar 48 rader färgsatt text med svenska ord, CJK-tecken och sammansatta emoji. Stycket ligger i en rundad ram med titel och utfyllnad. De fem arbetslasterna låter bilden stå stilla, ändrar ett tecken i ramen, skrollar en textrad i taget, växlar mellan 80 och 96 kolumner eller byter hela bakgrundsfärgen. Höjden är 24 rader. Alla 104 kombinationer av bredd, synlig text, färg och markör har jämförts som råa celler med båda Dunder-vägarna och Ratatui 0.30.2. [Dunder](reader.ddr) · [Ratatui](reference/examples/reader.rs)

Direktläget bryter om de rader som behövs för den aktuella bilden. Med `--prepared` återanvänds radbrytningen och skrollning ändrar bara första synliga rad. När bredden ändras bryts dokumentet om i de befintliga arrayerna. Ratatui ritar från samma förbyggda stycken i båda körningarna. Två dokument med olika bakgrundsfärg skapas före tidtagningen. Processernas allokeringar och högsta RSS inkluderar uppstart.

Mätningen omfattar ritning, celldiff och ANSI-kodning samt skrivning till pseudoterminalen i PTY-läget. Efter två uppvärmningskörningar följer sju mätkörningar med växlande startordning. Varje arbetslast har 20 uppvärmningsbilder och 200 mätbilder per körning. TSV-filen innehåller medianen av de sju p50- och p95-värdena, deras min- och maxvärden och antal byte per bild. [Fullständiga mätningar](results/reader-0.12.1.tsv)

## Bildtid och minne

Tabellen visar mikrosekunder vid skrivning till pseudoterminalen för den förberedda läsaren och Ratatui. macOS och Linuxgästerna kördes på samma Apple M5.

| Plattform | Dunder skrollning p50 / p95 | Ratatui skrollning p50 / p95 | Dunder storleksändring p50 / p95 | Ratatui storleksändring p50 / p95 |
| --- | ---: | ---: | ---: | ---: |
| macOS ARM64 | 89 / 188 | 118 / 260 | 131 / 275 | 111 / 172 |
| Linux ARM64 | 95 / 142 | 132 / 237 | 117 / 174 | 100 / 203 |
| Linux x86-64, översatt | 749 / 1 235 | 195 / 438 | 976 / 1 496 | 164 / 274 |

Den förberedda läsaren skrollade snabbare än Ratatui på båda ARM64-värdarna. När bredden växlade för varje bild hade Ratatui lägre medianbildtid på alla tre värdar. Översatt Linux x86-64 visar fortfarande en stor skillnad och en nativ x64-mätning av den här läsaren återstår.

Över samtliga 1 100 bilder skrev Dunder 2 145 281 byte och Ratatui 2 815 132 byte till terminalen på varje plattform. På macOS gav återanvändning av radtabellerna vid storleksändring 57 799 allokeringsanrop och 2 096 222 begärda byte för den förberedda läsaren. Före ändringen krävdes 63 626 anrop och 4 424 926 byte för samma scen. Tabellen visar processvärden inklusive uppstart. RSS är medianen av tre processers högsta värde. D står för Dunder och R för Ratatui.

| Plattform och Dunder-läge | Allokeringsanrop D / R | Begärda byte D / R | Högsta RSS i KiB D / R |
| --- | ---: | ---: | ---: |
| macOS ARM64, direkt | 89 065 / 66 703 | 7 342 082 / 22 397 509 | 2 320 / 2 544 |
| macOS ARM64, förberedd | 57 799 / 66 705 | 2 096 222 / 22 397 577 | 2 320 / 2 672 |
| Linux ARM64, direkt | 89 065 / 66 699 | 7 342 082 / 22 397 316 | 676 / 1 952 |
| Linux ARM64, förberedd | 57 799 / 66 701 | 2 096 222 / 22 397 384 | 676 / 1 948 |
| Linux x86-64, översatt, direkt | 89 065 / 66 699 | 7 342 082 / 22 397 316 | 6 340 / 5 484 |
| Linux x86-64, översatt, förberedd | 57 799 / 66 701 | 2 096 222 / 22 397 384 | 6 340 / 5 484 |

## Kör lokalt

```sh
mkdir -p build
dunder build --locked --offline benchmarks/reader.ddr -o build/reader-bench
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --example reader --features layout-cache
build/measure build/reader-bench build/rust/release/examples/reader build/pty
build/measure build/reader-bench build/rust/release/examples/reader build/pty -- --prepared

dunder build --locked --offline --runtime-profile benchmarks/reader.ddr -o build/reader-profile
build/reader-profile --prepared --allocations
cargo run --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust-profile --example reader --features layout-cache,allocation-counts -- --allocations
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak
build/peak build/reader-bench --prepared --allocations
build/peak build/rust/release/examples/reader --allocations

cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust-cells --features theme-oracle --example reader
build/rust-cells/release/examples/reader --snapshots >build/reader.expected
for mode in '' --prepared; do
    build/reader-bench $mode --snapshots >build/reader.actual
    cmp build/reader.expected build/reader.actual || exit 1
done
```
