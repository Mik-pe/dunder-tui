# Diagram med tre dataserier

Tre serier med 96 punkter ritas som fylld yta med sextanter, linje med braille och punktdiagram. Även y-axel, etiketter, teckenförklaring och statusrad ingår. Båda programmen förbereder data vid start, återanvänder diagrammet och ger samma celler och stilar i samtliga 1 100 bilder. [Dundervy](chart_scene.ddr) · [Rustvy](reference/src/chart_scene.rs) · [Jämförelseprov](../tests/chart_reference.ddr)

## Dunder 0.12.1 med cellvis ritning

Den 24 november 2026 kördes samma 1 100 bilder mot Ratatui 0.30.2 på Apple M5, macOS 27.2.1, och i lokala Linuxgäster. Tabellen visar medianen av sju körningars p50 / p95 i µs vid skrivning till en pseudoterminal. Två omgångar värmde upp systemen och programmen växlade startordning. I TSV-filen finns även tider utan terminal och min–max mellan körningarna. [Samtliga mätvärden](results/chart-shapes-0.12.1.tsv)

| Ändring | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Linux x64 översatt Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 90 / 190 | 105 / 204 | 89 / 150 | 109 / 223 | 703 / 1015 | 197 / 426 |
| En cell | 92 / 190 | 102 / 159 | 92 / 150 | 108 / 175 | 707 / 1249 | 198 / 396 |
| Skrollning | 143 / 291 | 181 / 322 | 120 / 184 | 171 / 256 | 973 / 1408 | 284 / 501 |
| Ny bredd | 140 / 280 | 165 / 293 | 114 / 222 | 163 / 294 | 972 / 1497 | 265 / 531 |
| Hel omritning | 151 / 287 | 203 / 370 | 121 / 202 | 193 / 365 | 1020 / 1509 | 320 / 684 |

Medianbildtiden var lägre för Dunder på båda ARM64-systemen i alla fem fall. Vid ändring av en cell på macOS var p95 högre för Dunder, och intervallen för p95 överlappade. På Linux x64 med CPU-översättning var Ratatui snabbare i alla fem fall. För hela sekvensen lästes 4 829 492 byte från Dunders terminal och 8 084 900 från Ratatuis på alla tre plattformar.

| Plattform | Allokeringsanrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS Dunder / Ratatui, KiB |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 27 444 / 25 820 | 1 425 346 / 129 245 345 | 1 936 / 2 944 |
| Linux ARM64 | 27 444 / 25 816 | 1 425 346 / 129 245 152 | 384 / 2 060 |
| Linux x64, översatt | 27 444 / 25 814 | 1 425 346 / 129 244 840 | 5 944 / 5 652 |

Dunders räknare omfattar 516 nya allokeringar och 26 928 anrop som återanvände poolminne. RSS är medianen av tre separata processkörningar. Kommandona längre ned bygger och kör samma mätprogram.

## Bildtid och terminalutdata i dunder-tui 0.9.0

Bilden är 96 × 24 celler. Encellsprovet växlar ett tecken på statusraden. Skrollning flyttar x-axelns intervall. Storleksändringen växlar bredden mellan 80 och 96 kolumner. Vid full omritning ändras diagrammets bakgrund. Tidtagningen börjar före tömning av bildbufferten och inkluderar ritning, celldiff, ANSI-kodning och i PTY-läget skrivning till pseudoterminal.

Tabellen visar medianen av sju körningars p50 / p95 i µs med skrivning till pseudoterminal. Programmen växlade startordning efter två uppvärmningskörningar. Varje fall omfattade 20 uppvärmningsbilder och 200 mätbilder. Maskinen var en Apple M5 med macOS 27.2.1. Linux ARM64 körde nativ maskinkod i en gäst och Linux x86-64 använde CPU-översättning. Versionerna var Dunder 0.11.2, Rust 1.99.1 och Ratatui 0.30.2 med layoutcache. [Alla mätvärden i ns och variation mellan körningar](results/charts-0.9.0.tsv)

| Ändring | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui | Byte/bild Dunder / Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 95 / 99 | 96 / 103 | 92 / 101 | 98 / 105 | 903 / 933 | 171 / 183 | 0 / 19 |
| En cell | 99 / 104 | 96 / 101 | 99 / 108 | 98 / 103 | 917 / 966 | 171 / 180 | 8 / 27 |
| Skrollning | 156 / 166 | 160 / 172 | 135 / 149 | 130 / 142 | 1199 / 1234 | 220 / 234 | 6 239 / 10 763 |
| Ny bredd | 152 / 161 | 149 / 164 | 129 / 136 | 117 / 127 | 1100 / 1187 | 200 / 208 | 7 640 / 12 702 |
| Hela diagrammet | 164 / 173 | 179 / 193 | 138 / 144 | 143 / 155 | 1245 / 1329 | 243 / 258 | 7 981 / 13 102 |

Från pseudoterminalen lästes 4 829 492 byte för Dunder och 8 084 900 för Ratatui på varje plattform inklusive uppvärmningen. Det motsvarar 40,3 procent färre byte. På båda ARM64-systemen hade Dunder lägre median för full omritning. Ratatui hade lägre median i alla fem fall på översatt x86-64.

## Allokeringar och minne i dunder-tui 0.9.0

Profilerna omfattar uppstart och alla 1 100 bilder med bildklockan frånkopplad. Dunder räknar också anrop som tjänas av runtimens pool. Av 539 786 anrop återanvände 539 232 poolminne och 554 var färska allokeringar. Rust använde en räknande global allokerare. Högsta RSS i KiB är medianen av tre separata körningar per program.

| Plattform | Allokeringsanrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS Dunder / Ratatui, KiB |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 539 786 / 25 820 | 10 020 778 / 129 245 549 | 1 968 / 2 960 |
| Linux ARM64 | 539 786 / 25 816 | 10 020 778 / 129 245 192 | 336 / 2 052 |
| Linux x86-64, översatt | 539 786 / 25 814 | 10 020 778 / 129 244 882 | 5 920 / 5 680 |

## Kör själv

Kör från repots rot. Båda benchmarkprogrammen använder samma 1 100 bilder i alla lägen. Cargo kan bygga med `--offline` efter första hämtningen av den fristående referensens låsta paket.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/charts.ddr -o build/chart-bench
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features layout-cache --example chart_bench
build/measure build/chart-bench build/rust/release/examples/chart_bench build/pty

dunder build --locked --offline --runtime-profile benchmarks/charts.ddr -o build/chart-profile
build/chart-profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust-profile --features layout-cache,allocation-counts --example chart_bench -- --allocations
```

För RSS, kör de två tidsmätta binärerna var för sig med `/usr/bin/time -l` på macOS eller `/usr/bin/time -v` på Linux. För att spara hela strömmen från pseudoterminalen, kör `build/pty build/chart-bench build/chart.ansi`.
