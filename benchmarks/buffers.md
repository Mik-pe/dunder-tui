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

## Ommätning i 0.16.0

Samma 1 100 bilder jämfördes efter införandet av direkta marköranrop. Tabellen visar median och p95 i mikrosekunder per bild i en pseudoterminal. I resultatfilen finns alla fem arbetslaster i minne och terminal, inklusive spridningen mellan körningarna. [Samtliga 60 mätserier](results/terminal-0.16.0.tsv)

| Plattform | Oförändrad, Dunder | Oförändrad, Ratatui | Rullning, Dunder | Rullning, Ratatui | Full omritning, Dunder | Full omritning, Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 102 / 106 | 96 / 99 | 161 / 172 | 160 / 171 | 167 / 176 | 178 / 197 |
| Linux ARM64 | 101 / 111 | 98 / 105 | 142 / 155 | 130 / 141 | 143 / 155 | 142 / 158 |
| Linux x86-64, översatt | 932 / 984 | 186 / 203 | 1245 / 1302 | 231 / 253 | 1312 / 1376 | 257 / 280 |

Dunder skrev 4 839 210 byte till pseudoterminalen och Ratatui skrev 8 096 420. I detta prov använder appen en muterbar backendreferens mellan bilder. Det tillkom 5 256 byte för att återställa markörens synlighet jämfört med 0.15.0. Allokeringsanropen inkluderar återanvändning av poolminne. RSS är medianen av tre körningars högsta residenta minne.

| Plattform | Anrop Dunder | Anrop Ratatui | Begärda byte Dunder | Begärda byte Ratatui | RSS Dunder, KiB | RSS Ratatui, KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 559 458 | 25 823 | 10 334 467 | 129 153 308 | 2 064 | 2 976 |
| Linux ARM64 | 559 458 | 25 819 | 10 334 467 | 129 153 109 | 428 | 2 024 |
| Linux x86-64, översatt | 559 458 | 25 817 | 10 334 467 | 129 152 800 | 6 128 | 5 728 |

## Ommätning i 0.17.0

Samma 1 100 bilder jämfördes med den nya arbetsbufferten och anropet som tömmer backendens utmatning. Tabellen visar median och p95 i mikrosekunder per bild i en pseudoterminal. I resultatfilen finns alla fem arbetslaster i minne och terminal, inklusive spridningen mellan körningarna. [Samtliga 60 mätserier](results/terminal-0.17.0.tsv)

| Plattform | Oförändrad, Dunder | Oförändrad, Ratatui | Rullning, Dunder | Rullning, Ratatui | Full omritning, Dunder | Full omritning, Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 102 / 112 | 93 / 104 | 171 / 190 | 169 / 201 | 176 / 197 | 197 / 222 |
| Linux ARM64 | 102 / 109 | 98 / 103 | 143 / 151 | 132 / 137 | 144 / 150 | 145 / 154 |
| Linux x86-64, översatt | 930 / 1014 | 182 / 196 | 1244 / 1351 | 234 / 250 | 1292 / 1363 | 259 / 274 |

Dunder skrev 4 838 820 byte till pseudoterminalen och Ratatui skrev 8 096 420. Det är 390 byte mindre för Dunder än i 0.16.0. Första diffen utgår nu från en tom bild med terminalens faktiska mått. Allokeringsanropen inkluderar återanvändning av poolminne. RSS är medianen av tre körningars högsta residenta minne.

| Plattform | Anrop Dunder | Anrop Ratatui | Begärda byte Dunder | Begärda byte Ratatui | RSS Dunder, KiB | RSS Ratatui, KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 559 462 | 25 822 | 10 334 579 | 129 152 500 | 2 064 | 2 960 |
| Linux ARM64 | 559 462 | 25 819 | 10 334 579 | 129 153 109 | 428 | 2 024 |
| Linux x86-64, översatt | 559 462 | 25 817 | 10 334 579 | 129 152 800 | 6 192 | 5 664 |

## Ommätning i 0.18.0

Testet ritar samma 1 100 bilder efter ändringarna i linje- och ytdiagrammen. Tabellen visar median och p95 i mikrosekunder per bild i en pseudoterminal. I resultatfilen finns alla fem arbetslaster i minne och terminal, inklusive spridningen mellan körningarna. [Samtliga 60 mätserier](results/terminal-0.18.0.tsv)

| Plattform | Oförändrad, Dunder | Oförändrad, Ratatui | Rullning, Dunder | Rullning, Ratatui | Full omritning, Dunder | Full omritning, Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 72 / 76 | 93 / 100 | 134 / 144 | 169 / 185 | 139 / 149 | 189 / 206 |
| Linux ARM64 | 71 / 77 | 98 / 104 | 110 / 122 | 132 / 143 | 109 / 117 | 142 / 152 |
| Linux x86-64, översatt | 628 / 660 | 178 / 191 | 961 / 1005 | 230 / 245 | 1040 / 1080 | 255 / 270 |

Dunder skrev 4 838 820 byte till pseudoterminalen och Ratatui skrev 8 096 420. Den tidigare Dunder-versionen skrev exakt samma antal byte i alla fem arbetslaster. Allokeringsanropen inkluderar återanvändning av poolminne. RSS är medianen av tre körningars högsta residenta minne.

| Plattform | Anrop Dunder | Anrop Ratatui | Begärda byte Dunder | Begärda byte Ratatui | RSS Dunder, KiB | RSS Ratatui, KiB |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 141 426 | 25 823 | 3 581 171 | 129 153 308 | 2 016 | 2 976 |
| Linux ARM64 | 141 426 | 25 818 | 3 581 171 | 129 152 301 | 336 | 2 024 |
| Linux x86-64, översatt | 141 426 | 25 817 | 3 581 171 | 129 152 800 | 6 064 | 5 660 |

### Före och efter på samma värd

Dunder 0.17.0 och 0.18.0 byggdes med samma Dunder 0.11.2. Programmen kördes växelvis i en egen serie på varje värd, med två uppvärmningskörningar följda av sju uppmätta körningar per program. Tabellen visar median / p95 i µs per bild i en pseudoterminal. Resultatfilen innehåller också minsta och största uppmätta median och p95. [Alla 60 mätserier för versionsjämförelsen](results/terminal-before-after-0.18.0.tsv)

| Plattform | Oförändrad 0.17 | Oförändrad 0.18 | Rullning 0.17 | Rullning 0.18 | Full omritning 0.17 | Full omritning 0.18 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 99 / 108 | 70 / 77 | 164 / 175 | 133 / 147 | 171 / 184 | 137 / 149 |
| Linux ARM64 | 101 / 108 | 71 / 77 | 142 / 150 | 110 / 118 | 143 / 152 | 107 / 113 |
| Linux x86-64, översatt | 924 / 982 | 621 / 651 | 1235 / 1342 | 947 / 1001 | 1287 / 1342 | 1022 / 1088 |

För att upprepa versionsjämförelsen, bygg `benchmarks/terminal.ddr` från vardera taggen med samma kompilator. Ge båda binärerna och PTY-drivrutinen från körkommandona ovan till mätprogrammet.

```sh
TUI_BENCH_DIFF_OPTIONS=1 build/measure build/terminal-0.17.0 build/terminal-0.18.0 build/pty --labels v0.17.0 v0.18.0
```

På Apple M5 med macOS 27.2.1 och i Linux ARM64 på samma värd hade Dunder lägre median och p95 än Ratatui för samtliga fem arbetslaster i pseudoterminalen. Även den högsta uppmätta medianen respektive p95 för Dunder låg under motsvarande lägsta värde för Ratatui i dessa körningar. På översatt Linux x86-64 var Ratatui snabbare i samtliga fall. Fysisk Linux x86-64 och nativ Windows x64 återstår.
