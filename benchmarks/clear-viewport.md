# Rensa en fast vyport

En terminal på 96 × 24 celler växlar mellan två fasta vyer med `Terminal.resize`. Fyra fall mäter en fullbred vy till nederkanten, åtta fullbreda rader mitt på skärmen, 91 kolumner av åtta rader och en vy med höjd noll. Varje process gör tio uppvärmningar och 100 tidsmätta anrop mot en pseudoterminal. Terminalöppning och stängning ligger utanför bildtiden. [Dunder](clear_viewport.ddr) · [Ratatui 0.30.2](reference/examples/clear_viewport.rs)

## Tid per storleksändring

Tabellen visar median / p95 i mikrosekunder. Varje värde är medianen av sju processers respektive percentil. Körordningen roterade mellan fallen och de fyra programmen. Föregående Dunder är `991198e`, byggd med samma mätprogram. Linux ARM64 kördes i en lokal VM på samma Apple M5 som macOS 27.2.1. Linux x64 kördes med Rosetta i VM:n. Dunder 0.12.1 och Rust 1.99.1 byggde de optimerade programmen. [Samtliga 336 körningar](results/clear-viewport-0.12.1.tsv)

| Plattform | Vy | Föregående Dunder | Dunder | Crossterm | Termion |
| --- | --- | ---: | ---: | ---: | ---: |
| macOS ARM64 | Till nederkant | 11,75 / 29,62 | 2,12 / 2,92 | 9,79 / 10,92 | 2,92 / 3,71 |
| macOS ARM64 | Hela rader | 5,42 / 14,67 | 1,67 / 2,75 | 22 / 27 | 15,38 / 17,04 |
| macOS ARM64 | Del av rader | 5,54 / 18,12 | 4,88 / 14,38 | 19 / 28,88 | 5,79 / 10,25 |
| macOS ARM64 | Tom | 0,29 / 0,38 | 0,08 / 0,12 | 0 / 0,04 | 0 / 0,04 |
| Linux ARM64 | Till nederkant | 9,42 / 28,88 | 1,33 / 5,92 | 5,38 / 18,12 | 2,58 / 17,21 |
| Linux ARM64 | Hela rader | 5,83 / 18,67 | 1,12 / 14,5 | 9,12 / 35,33 | 9,62 / 33,5 |
| Linux ARM64 | Del av rader | 5 / 13,12 | 1,33 / 16,29 | 14,38 / 20,25 | 5,96 / 12,58 |
| Linux ARM64 | Tom | 0,25 / 0,29 | 0,08 / 0,12 | 0 / 0,04 | 0 / 0,04 |
| Översatt Linux x64 | Till nederkant | 89,17 / 96,92 | 8,25 / 11,83 | 6,08 / 8,54 | 4,08 / 6,71 |
| Översatt Linux x64 | Hela rader | 46,83 / 53,88 | 9,75 / 10,88 | 16,62 / 27,71 | 13,29 / 26,46 |
| Översatt Linux x64 | Del av rader | 44,88 / 51,92 | 11,04 / 14,12 | 21,17 / 23,62 | 9,17 / 13,12 |
| Översatt Linux x64 | Tom | 3,08 / 3,21 | 1,12 / 1,17 | 0,04 / 0,04 | 0,04 / 0,04 |

För fullbredd till nederkanten på macOS låg de sju processmedianerna inom 2,04–3,75 µs för Dunder, 8,62–11,79 µs för Crossterm och 2,58–3,5 µs för Termion. På översatt Linux x64 var motsvarande intervall 6,79–9,83, 4,79–8 och 2,62–7,67 µs. Båda Rust-backendarna behandlar den tomma vyn snabbare. Värden nära noll ligger vid klockans upplösning.

## Terminalbyte och minne

Byteantalet avser alla 110 anrop. En separat körning som endast öppnar och stänger samma vy dras av. Båda biblioteken använder ett rensningskommando för fullbredd till nederkanten och ett per rad för hela rader mitt på skärmen. I en smal vy skriver de blanksteg inom rektangeln.

| Vy | Föregående Dunder | Dunder | Crossterm macOS / Linux | Termion |
| --- | ---: | ---: | ---: | ---: |
| Till nederkant | 175 560 | 1 045 | 1 045 / 1 045 | 1 045 |
| Hela rader | 90 035 | 9 075 | 9 075 / 9 075 | 9 075 |
| Del av rader | 85 580 | 85 580 | 87 010 / 87 670 | 87 010 |
| Tom | 0 | 0 | 0 / 0 | 0 |

För vyn till nederkanten blev högsta RSS i KiB 1 680 / 2 160 / 2 176 på macOS och 832 / 1 948 / 1 948 på Linux ARM64, i ordningen Dunder / Crossterm / Termion. Det är medianen för den uppmätta barnprocessen via `getrusage`. I en separat profilerad macOS-process för samma 110 anrop registrerades 1 171 / 14 / 15 allokeringsanrop och 82 727 / 151 107 / 151 108 begärda byte. Dunders räknare inkluderar poolåterbruk. Rust räknar begäran till den globala allokeraren. Profilerna omfattar även start, mätvärden och sortering.

## Kör lokalt

```sh
mkdir -p build
dunder build --locked --offline benchmarks/clear_viewport.ddr -o build/clear
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml \
  --no-default-features --features erase-oracle --example clear_viewport --target-dir build/rust
rust="$PWD/build/rust/release/examples/clear_viewport"
for mode in bottom rows partial empty; do
  for app in "$PWD/build/clear" "$rust"; do
    build/pty --peak-rss "$app" -- "$mode"
    build/pty "$app" -- "$mode" setup
  done
  TUI_TERMION=1 build/pty --peak-rss "$rust" -- "$mode"
  TUI_TERMION=1 build/pty "$rust" -- "$mode" setup
done
```

För allokeringar, bygg mätprogrammen ytterligare en gång med Dunders `--runtime-profile` respektive Cargo-funktionerna `erase-oracle,allocation-counts`. För den tidigare Dunder-versionen, lägg samma [mätprogram](clear_viewport.ddr) i katalogen `benchmarks` i en lokal arbetskopia av [991198e](https://github.com/Mik-pe/dunder-tui/tree/991198e1c1808c70c3f8145ce71ded91d47514b2) och bygg på samma sätt.
