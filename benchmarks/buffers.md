# Diagram med delad terminalyta i 0.13.0

På diagrammets statusrad delar två ritare samma terminalyta. Åtta celler överlåts till den andra ritaren, ett plustecken skrivs på varje bild och en pil reserverar fyra kolumner. Diagrammet använder tre dataserier med 96 sampel vardera. Båda programmen använder det publika terminal-API:t, ritar samma innehåll och placerar markören på rad 24. De avslutade bilderna i båda testbackendarna jämförs för varje cell, dess stil, terminalmarkör och bildräknare. [Dunder](terminal.ddr) · [Ratatui](reference/examples/terminal_bench.rs) · [Cellregler i en pseudoterminal](../tests/buffer_terminal.ddr)

## Bildtid och terminalutdata

Apple M5 med macOS 27.2.1, Linux ARM64 i en gäst på samma värd och Linux x86-64 under översättning. Dunder 0.11.2 med standardoptimering och Ratatui 0.30.2 byggd med Rust 1.99.1 i releaseprofil med layoutcache. Alla 1 100 bilder matchade på varje plattform. Den fasta storleken är 96 × 24 celler. Storleksändringen växlar mellan 80 och 96 kolumner.

Varje arbetslast hade 20 uppvärmningsbilder följda av 200 mätbilder. Efter två uppvärmningskörningar kördes sju växelvis ordnade mätningar per program. Tabellen visar medianen av körningarnas p50 · p95 i mikrosekunder vid skrivning till en pseudoterminal. D är Dunder och R är Ratatui. Rådata anger även minsta och största p50 och p95 samt motsvarande tider för utmatning i minnet. [Alla 60 mätserier](results/buffers-0.13.0.tsv)

| Arbetslast | macOS ARM64 D / R | Linux ARM64 D / R | Linux x86-64 översatt D / R | Byte per bild D / R |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad | 99 · 109 / 94 · 101 | 100 · 110 / 98 · 108 | 919 · 976 / 181 · 194 | 16 / 41 |
| En cell | 98 · 108 / 93 · 100 | 100 · 109 / 98 · 106 | 918 · 962 / 178 · 189 | 24 / 49 |
| Skrollning | 164 · 177 / 168 · 183 | 141 · 154 / 130 · 142 | 1231 · 1306 / 231 · 250 | 6 259 / 10 793 |
| Ny bredd | 168 · 182 / 176 · 192 | 145 · 155 / 140 · 150 | 1355 · 1436 / 244 · 258 | 7 588 / 12 652 |
| Full omritning | 170 · 185 / 189 · 205 | 142 · 155 / 144 · 158 | 1292 · 1354 / 260 · 283 | 8 001 / 13 132 |

Pseudoterminalen tog emot 4 833 954 byte från Dunder och 8 096 420 från Ratatui per körning inklusive uppvärmningsbilderna. Skillnaden är 40,3 procent. På macOS hade Dunder lägre median vid skrollning, storleksändring och hel omritning. Ratatui hade lägre median för oförändrad vy och encellsändring. På Linux ARM64 hade Ratatui lägre median vid skrollning och på översatt x86-64 i samtliga fall. Fysisk x86-64 och nativ Windows återstår för den här arbetslasten.

## Allokeringar och minne

Profilerna omfattar uppstart och 1 100 bilder utan tidtagning. RSS är medianen av tre nya processers maximala residenta minne utan profilerare. Dunders allokeringsräknare inkluderar poolåterbruk. På macOS gav 573 anrop färskt minne och 558 880 anrop återanvände poolminne. Rust använde en räknande global allokerare.

| Plattform | Allokeringsanrop D / R | Begärda byte D / R | Högsta RSS D / R, KiB |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 559 453 / 25 823 | 10 334 307 / 129 153 308 | 2 064 / 2 960 |
| Linux ARM64 | 559 453 / 25 818 | 10 334 307 / 129 152 301 | 428 / 2 088 |
| Linux x86-64, översatt | 559 453 / 25 817 | 10 334 307 / 129 152 800 | 6 192 / 5 660 |

## Upprepa

Kör från repots rot. Miljövariabeln väljer vyn med delade celler även i underprocesserna. Rustreferensen kan byggas med Cargo-flaggan `--offline` efter att de låsta referenspaketen hämtats.

```sh
mkdir -p build
export TUI_BENCH_DIFF_OPTIONS=1
dunder build --locked --offline benchmarks/terminal.ddr -o build/terminal
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features layout-cache --example terminal_bench
build/terminal --verify | build/rust/release/examples/terminal_bench --verify
build/measure build/terminal build/rust/release/examples/terminal_bench build/pty
dunder build --locked --offline --runtime-profile benchmarks/terminal.ddr -o build/terminal-profile
build/terminal-profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache,allocation-counts --example terminal_bench -- --allocations
```

## Ommätning i 0.15.0

Samma 1 100 bilder jämfördes efter ändringen av terminalens storleksanpassning. Tabellen visar median och p95 i mikrosekunder per bild i en pseudoterminal. I resultatfilen finns alla fem arbetslaster i minne och terminal, inklusive spridningen mellan körningarna. [Samtliga 60 mätserier](results/terminal-0.15.0.tsv)

| Plattform | Oförändrad, Dunder | Oförändrad, Ratatui | Rullning, Dunder | Rullning, Ratatui | Full omritning, Dunder | Full omritning, Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 99 / 107 | 94 / 101 | 164 / 178 | 169 / 180 | 172 / 182 | 188 / 202 |
| Linux ARM64 | 100 / 108 | 98 / 105 | 140 / 152 | 131 / 141 | 141 / 151 | 142 / 154 |
| Linux x86-64, översatt | 931 / 970 | 178 / 192 | 1243 / 1296 | 231 / 253 | 1307 / 1366 | 258 / 277 |

Dunder skrev 4 833 954 byte till pseudoterminalen och Ratatui skrev 8 096 420. Allokeringsanropen inkluderar återanvändning av poolminne. RSS är medianen av tre körningars högsta residenta minne.

| Plattform | Anrop Dunder | Anrop Ratatui | Begärda byte Dunder | Begärda byte Ratatui | RSS Dunder, KiB | RSS Ratatui, KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 559 458 | 25 822 | 10 334 347 | 129 152 500 | 2 064 | 2 960 |
| Linux ARM64 | 559 458 | 25 818 | 10 334 347 | 129 152 301 | 428 | 2 084 |
| Linux x86-64, översatt | 559 458 | 25 817 | 10 334 347 | 129 152 800 | 6 184 | 5 664 |
