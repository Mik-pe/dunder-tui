# Layout i 0.7.0

Vyn använder en fast rubrik, fyra kolumner med olika storleksregler och fyra rader i varje kolumn. Referensen och Dunder ger identiska tecken och stilar i alla 1 100 bilder. Båda använder layoutcache. [Dundervy](layout_scene.ddr) · [Ratatui-vy](reference/src/layout_scene.rs)

## Bildtid

Varje bild innefattar layout, ritning, celldiff och ANSI-kodning. PTY-läget inkluderar skrivning till pseudoterminalen. Bildytan är 96 × 24 celler. Storleksprovet växlar mellan 80 och 96 kolumner, skrollprovet flyttar ett synligt mönster en rad och omritningsprovet ändrar bakgrunden i varje panel. Tabellen visar medianen av sju körningars p50 / p95 i µs. Varje körning hade 20 uppvärmningsbilder och 200 mätbilder per fall. [Min/max och full upplösning i ns](results/layout-0.7.0.tsv)

| Ändring | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui | Byte/bild Dunder / Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 10 / 10 | 24 / 27 | 11 / 11 | 31 / 35 | 139 / 155 | 42 / 46 | 0 / 19 |
| En cell | 13 / 15 | 25 / 27 | 15 / 18 | 31 / 35 | 145 / 157 | 42 / 46 | 7 / 26 |
| Skrollning | 54 / 58 | 68 / 74 | 37 / 41 | 56 / 62 | 338 / 367 | 82 / 88 | 3 143 / 4 123 |
| Ny bredd | 47 / 51 | 53 / 63 | 31 / 33 | 40 / 48 | 263 / 279 | 61 / 65 | 3 448 / 4 234 |
| Hela panelytan | 57 / 61 | 68 / 75 | 40 / 43 | 56 / 60 | 327 / 344 | 85 / 90 | 3 143 / 4 123 |

Inklusive uppvärmningen lästes 2 151 287 byte från Dunders pseudoterminal och 2 764 913 från Ratatuis på alla tre plattformarna. Båda ARM64-körningarna gjordes på en Apple M5. Linux x86-64 använde CPU-översättning. Där hade Dunder högre bildtid i alla fem fallen. macOS var 27.2.1, Dunder 0.11.0 och Rust 1.99.1.

## Kostnad för att dela en yta

I varje mätgrupp görs 64 delningar och alla paneler och mellanrum läses. Båda programmen kontrolleras mot samma positionssumma. Cachen rymmer en lösning. Bredden är oförändrad i första fallet och växlar mellan 40 och 63 i de andra. Grupperna upprepas 35 gånger, de fem första räknas bort. Tabellen visar medianen av fem processers p50 / p95 för grupptid per delning, i ns.

| Fall | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Samma yta | 15 / 31 | 46 / 54 | 39 / 39 | 82 / 96 | 103 / 104 | 55 / 58 |
| Ny bredd med fasta och flexibla mått | 203 / 218 | 41 007 / 58 465 | 219 / 505 | 38 637 / 51 561 | 2 460 / 4 896 | 49 811 / 77 456 |
| Ett bindande minimimått | 187 / 203 | 28 796 / 44 157 | 199 / 256 | 27 665 / 39 188 | 2 375 / 4 643 | 37 343 / 52 887 |
| Fyra paneler, överlappning och SpaceAround | 297 593 / 437 875 | 136 125 / 169 164 | 222 479 / 315 761 | 123 402 / 168 903 | 1 814 877 / 2 220 488 | 162 574 / 197 842 |

Överlappningsfallet aktiverar den generella lösaren. Det kostar mer än referensen på macOS. Övriga fall kan ofta lösas direkt i Dunder.

## Minne

Mätningen omfattar start, alla fem arbetslaster och uppvärmning. Dunders räknare inkluderar återanvända poolblock. Rust räknar allokerings- och omallokeringsanrop. Högsta RSS är medianen av tre separata körningar.

| Plattform | Anrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS Dunder / Ratatui, KiB |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 1 420 / 11 164 | 359 364 / 2 183 197 | 1 728 / 2 720 |
| Linux ARM64 | 1 420 / 11 164 | 359 364 / 2 189 990 | 336 / 2 028 |
| Linux x86-64, översatt | 1 420 / 11 197 | 359 364 / 2 233 137 | 5 372 / 5 492 |

## Kör själv

```sh
mkdir -p build
dunder build --locked --offline benchmarks/layouts.ddr -o build/layouts
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --target-dir build/rust --example layout_bench
build/measure build/layouts build/rust/release/examples/layout_bench build/pty
build/layouts --splits
build/rust/release/examples/layout_bench --splits

dunder build --locked --offline --runtime-profile benchmarks/layouts.ddr -o build/profile
build/profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache,allocation-counts --example layout_bench -- --allocations
```

För RSS, kör varje tidsmätt binär med `/usr/bin/time -l` på macOS eller `/usr/bin/time -v` på Linux. Den låsta Rustreferensen kan byggas med `--offline` efter den första hämtningen.
