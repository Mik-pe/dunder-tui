# Loggvy med emoji i 0.18.1

På varje bild skriver appen 23 loggrader med svensk text, färg, flaggor och sammansatta emoji till en terminal på 96 × 24 celler. På sista raden visas en statusmarkering. Båda programmen läser oformaterade strängar vid ritningen. Skrollning flyttar loggen en rad, storleksändring växlar mellan 80 och 96 kolumner och full omritning växlar hela loggens färger. I den oberoende jämförelsen matchade celler, stilar, markör och bildnummer i alla 1 100 bilder på varje uppmätt plattform. [Dunder](terminal.ddr) · [Ratatui](reference/examples/terminal_bench.rs)

## Bildtid och terminalutdata

Tabellen anger median · p95 i mikrosekunder per bild vid utmatning till en pseudoterminal. Dunder står först i varje par. Varje arbetslast har 20 uppvärmningsbilder och 200 mätbilder. Efter två uppvärmningskörningar växlar programmen körordning i sju mätomgångar. Resultaten är medianen av respektive omgångs p50 och p95. Rådata innehåller även minsta och största uppmätta p50 och p95, antal byte per bild och tider för utmatning i minnet. [Alla 60 mätserier](results/emoji-0.18.1.tsv)

| Ändring | macOS ARM64 | Linux ARM64 | Linux x86-64 under översättning |
| --- | ---: | ---: | ---: |
| Oförändrad | 45 · 50 / 54 · 60 | 47 · 52 / 63 · 69 | 462 · 489 / 95 · 106 |
| En cell | 45 · 49 / 54 · 59 | 47 · 52 / 63 · 69 | 460 · 487 / 95 · 103 |
| Skrollning | 85 · 90 / 92 · 99 | 70 · 76 / 85 · 93 | 772 · 800 / 139 · 151 |
| Ny bredd | 79 · 85 / 95 · 103 | 68 · 74 / 86 · 100 | 762 · 823 / 145 · 165 |
| Full omritning | 78 · 83 / 94 · 102 | 65 · 71 / 87 · 94 | 688 · 723 / 142 · 154 |

Totalt läste pseudoterminalen 2 064 494 byte från Dunder och 2 280 403 från Ratatui för samma 1 100 bilder. Mätvärdena kommer från en Apple M5 med macOS 27.2.1 och en Linuxgäst på samma maskin. Linux x86-64 kördes under översättning. Programmen byggdes med Dunder 0.11.2 och Rust 1.99.1 i releaseprofil med Ratatui 0.30.2.

## Allokering och minne

Profilerna räknar allokeringsanrop och begärda byte över alla fem arbetslaster med tidtagningen avstängd. Av Dunders 121 708 anrop hämtade 148 nytt minne och 121 560 återanvände minnespoolen. RSS är medianen av tre separata oprofilerade processers högsta residenta minne.

| Plattform | Anrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS i KiB, Dunder / Ratatui |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 121 708 / 1 177 | 13 381 255 / 1 154 374 | 2 016 / 2 496 |
| Linux ARM64 | 121 708 / 1 173 | 13 381 255 / 1 154 181 | 412 / 1 948 |
| Linux x86-64, översatt | 121 708 / 1 173 | 13 381 255 / 1 154 184 | 6 076 / 5 476 |

## Upprepa mätningen

Bygg först mätprogrammen. Rustreferensen har en egen låsfil. När dess källpaket finns i den lokala Cargo-cachen går även referensbygget att köra med `--offline`.

```sh
mkdir -p build
export TUI_BENCH_SCENE=emoji
unset TUI_BENCH_DIFF_OPTIONS
dunder build --locked --offline benchmarks/terminal.ddr -o build/logg
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example terminal_bench --target-dir build/rust
cp build/rust/release/examples/terminal_bench build/ratatui
build/logg --verify >build/loggbilder
build/ratatui --verify <build/loggbilder
build/measure build/logg build/ratatui build/pty
```

För allokeringsräknarna, bygg Dunderprogrammet med `--runtime-profile` och Rustprogrammet med `--features layout-cache,allocation-counts`. Kör vardera med `--allocations`. Kör de oprofilerade programmen med `/usr/bin/time -l` på macOS och `/usr/bin/time -v` på Linux för att läsa högsta RSS.
