# En hel terminalritning med synlig markör

Båda programmen ritar samma diagram med tre dataserier genom sitt publika `Terminal.draw` och begär en synlig markör på statusraden. Testbackendarna jämför samtliga 1 100 bilder, inklusive cellstilar, markörposition och bildnummer. För tidsmätningen skriver Dunder sin ANSI-diff och Ratatui använder Crossterm. Båda samlar utdata i en återanvänd bytebuffert och kan skriva den till en pseudoterminal. Storleken kommer från samma förutbestämda serie i båda programmen. [Dunder](terminal.ddr) · [Rust](reference/examples/terminal_bench.rs)

## Version 0.11.0

Samma diagram och 1 100 färdiga bilder jämfördes igen efter införandet av absoluta buffertkoordinater och vyportar. Tabellen visar bildtid med PTY i mikrosekunder som median / p95. Varje värde är medianen av sju mätserier. Minsta och största resultat för varje serie finns i råtabellen. [Alla tider och byte per bild](results/terminal-0.11.0.tsv)

| Ändring | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 92 / 99 | 93 / 101 | 92 / 98 | 98 / 105 | 906 / 945 | 184 / 190 |
| En cell | 97 / 103 | 93 / 101 | 98 / 106 | 98 / 104 | 915 / 967 | 183 / 188 |
| Skrollning | 156 / 166 | 167 / 180 | 135 / 144 | 130 / 137 | 1 204 / 1 253 | 232 / 257 |
| Ny bredd | 152 / 163 | 174 / 193 | 124 / 133 | 136 / 146 | 1 151 / 1 218 | 247 / 266 |
| Hela diagrammet | 163 / 177 | 188 / 200 | 138 / 147 | 142 / 157 | 1 255 / 1 338 | 254 / 271 |

På macOS tog en hel omritning 163 µs med Dunder och 188 µs med Ratatui. En ensam celländring tog 97 respektive 93 µs. Programmen skrev sammanlagt 4 835 689 respektive 8 081 000 byte till terminalen. Linux x86-64 använde CPU-översättning. En mätserie på Linux ARM64 gav tydliga tidsavvikelser vid ändrad bredd och full omritning.

| Plattform | Allokeringsanrop Dunder / Rust | Färska Dunderallokeringar | Begärda byte Dunder / Rust | Högsta RSS i KiB Dunder / Rust |
| --- | ---: | ---: | ---: | ---: |
| macOS ARM64 | 544 284 / 25 822 | 562 | 10 042 016 / 129 153 325 | 2 016 / 2 960 |
| Linux ARM64 | 544 284 / 25 818 | 562 | 10 042 016 / 129 153 126 | 400 / 2 020 |
| Linux x86-64, översatt | 544 284 / 25 816 | 562 | 10 042 016 / 129 152 817 | 6 064 / 5 668 |

## Bildtid i 0.10.0

Varje arbetslast körs i 96 × 24 celler med 20 uppvärmningsbilder och 200 mätbilder. Storleksprovet växlar mellan 80 och 96 kolumner. Tiden omfattar förberedelse av bildbufferten, ritning, diff, markörstyrning och i PTY-läget skrivningen. Tabellen visar medianen av sju körningars p50 / p95 i mikrosekunder med PTY. Programmen växlade startordning. Råtabellen visar även variationen mellan körningar och motsvarande värden utan terminalskrivning. [Mätserier i nanosekunder](results/terminal-0.10.0.tsv)

| Ändring | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui | Byte/bild Dunder / Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 92 / 98 | 93 / 100 | 92 / 99 | 98 / 105 | 900 / 974 | 188 / 213 | 0 / 32 |
| En cell | 97 / 102 | 93 / 99 | 99 / 105 | 98 / 104 | 911 / 937 | 187 / 209 | 15 / 40 |
| Skrollning | 158 / 170 | 169 / 182 | 135 / 143 | 129 / 140 | 1198 / 1230 | 246 / 271 | 6 246 / 10 776 |
| Ny bredd | 153 / 166 | 176 / 193 | 129 / 134 | 137 / 143 | 1091 / 1180 | 262 / 296 | 7 647 / 12 634 |
| Hela diagrammet | 167 / 178 | 189 / 201 | 138 / 145 | 141 / 153 | 1243 / 1419 | 282 / 307 | 7 988 / 13 115 |

Pseudoterminalen tog emot 4 835 689 byte från Dunder och 8 081 000 från Ratatui inklusive uppvärmning. Det motsvarar 40,2 procent färre byte. På macOS var Dunders median för skrollning, ny bredd och full omritning lägre än Ratatuis lägsta median över de sju körningarna. För en ensam ändrad cell hade Ratatui lägre median. Linux x86-64 kördes med CPU-översättning och hade högre bildtider för Dunder i samtliga fall.

## Allokeringar och minne i 0.10.0

Profilerna avser uppstart och samma 1 100 bilder. Dunders räknare inkluderar poolträffar. Av 544 269 anrop var 559 färska allokeringar. Rust använder en räknande global allokerare. RSS är medianen av toppvärdet i tre separata processer utan profilräknare.

| Plattform | Allokeringsanrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS i KiB Dunder / Ratatui |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 544 269 / 25 822 | 10 041 496 / 129 153 565 | 1 984 / 2 960 |
| Linux ARM64 | 544 269 / 25 816 | 10 041 496 / 129 151 510 | 332 / 2 136 |
| Linux x86-64, översatt | 544 269 / 25 816 | 10 041 496 / 129 152 817 | 5 936 / 5 652 |

## Återskapa

Mätvärdena togs på Apple M5 med macOS 27.2.1. Linux ARM64 använde nativ kod i en gäst. Verktygen var Dunder 0.11.2, Rust 1.99.1 och Ratatui 0.30.2 med layoutcache. Kör från repots rot. Lägg till `--offline` på Cargo-anropen när referenspaketen har hämtats.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/terminal.ddr -o build/terminal-bench
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example terminal_bench --target-dir build/rust
build/terminal-bench --verify | build/rust/release/examples/terminal_bench --verify
build/measure build/terminal-bench build/rust/release/examples/terminal_bench build/pty

dunder build --locked --offline --runtime-profile benchmarks/terminal.ddr -o build/profile
build/profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache,allocation-counts --example terminal_bench -- --allocations
```

För RSS, kör de två tidsmätta binärerna var för sig med `/usr/bin/time -l` på macOS eller `/usr/bin/time -v` på Linux. `build/pty build/terminal-bench build/output.ansi` sparar terminalströmmen.
