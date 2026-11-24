# Skicka färdiga bildbuffertar till terminalen

Provet växlar mellan två färglagda ASCII-bilder med 96 × 24 celler i en pseudoterminal. Det mäter från första jämförelsen till slutförd skrivning. Dunder använder `Frame.ansi_cells` och Ratatui använder `Buffer::diff_iter` och Crossterm. Bilderna byggs före mätningen. Vid breddbyte växlar bildytan mellan 96 och 80 kolumner och båda ritar om hela den nya ytan. [Dunder](frame_output.ddr) · [Ratatui](reference/examples/frame_output.rs)

## Bildtid

Median / p95 i mikrosekunder per bild. Varje process gör 20 uppvärmningar och 200 tidsmätta bildväxlingar. Tabellen visar medianen av sju processers respektive percentil på ARM64 och fjorton på översatt Linux x64. Inom varje serie roterar körordningen mellan programmen och mellan arbetslasterna. Två uppvärmningsprocesser per kombination ingår inte i tabellen. Dunder 0.12.1 och Rust 1.99.1 kördes på en Apple M5 med macOS 27.2.1. Linux kördes i en lokal VM, med Rosetta för x64. [Samtliga 672 körningar](results/frame-output-0.12.1.tsv) · [Föregående Dunder](https://github.com/Mik-pe/dunder-tui/tree/f12c0e50108c26dd99bbf099e049c95d91288b50)

| Plattform | Bildändring | Föregående Dunder | Dunder | Ratatui 0.30.2 |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | Oförändrad | 3,96 / 4,04 | 0,92 / 0,96 | 12,46 / 14,83 |
| macOS ARM64 | En cell | 5,29 / 5,67 | 3,50 / 3,71 | 13,12 / 15,29 |
| macOS ARM64 | Enbart färg | 5,38 / 6,17 | 4,04 / 5,08 | 13,17 / 16,21 |
| macOS ARM64 | Sista cellen | 5,42 / 5,58 | 3,50 / 3,71 | 14,17 / 15,42 |
| macOS ARM64 | En cell per rad | 6,42 / 6,67 | 6,92 / 7,21 | 14,92 / 16,75 |
| macOS ARM64 | Rullning | 20,50 / 24,38 | 20,25 / 30,71 | 43,67 / 57,21 |
| macOS ARM64 | Breddbyte | 13,25 / 24,29 | 13,12 / 27,29 | 29,79 / 41,92 |
| macOS ARM64 | Alla celler | 19,21 / 30,12 | 19,17 / 29,79 | 42,21 / 52,00 |
| Linux ARM64 | Oförändrad | 3,96 / 4,00 | 0,92 / 0,92 | 13,79 / 14,96 |
| Linux ARM64 | En cell | 7,04 / 8,38 | 4,71 / 7,21 | 13,92 / 15,50 |
| Linux ARM64 | Enbart färg | 7,08 / 9,58 | 5,42 / 8,46 | 13,79 / 15,42 |
| Linux ARM64 | Sista cellen | 7,00 / 14,92 | 4,88 / 7,00 | 13,50 / 15,33 |
| Linux ARM64 | En cell per rad | 8,17 / 10,12 | 8,71 / 12,67 | 14,67 / 16,54 |
| Linux ARM64 | Rullning | 12,96 / 20,50 | 13,08 / 21,00 | 35,96 / 39,33 |
| Linux ARM64 | Breddbyte | 7,75 / 29,46 | 7,83 / 33,83 | 24,46 / 27,00 |
| Linux ARM64 | Alla celler | 12,29 / 34,58 | 12,17 / 15,12 | 37,54 / 40,25 |
| Linux x64, Rosetta | Oförändrad | 32,06 / 37,92 | 17,13 / 18,29 | 24,44 / 43,08 |
| Linux x64, Rosetta | En cell | 38,75 / 43,42 | 24,31 / 26,58 | 23,69 / 25,73 |
| Linux x64, Rosetta | Enbart färg | 37,92 / 43,00 | 24,10 / 26,17 | 23,75 / 25,46 |
| Linux x64, Rosetta | Sista cellen | 37,77 / 42,94 | 24,35 / 26,85 | 24,50 / 26,10 |
| Linux x64, Rosetta | En cell per rad | 50,94 / 58,27 | 37,77 / 43,56 | 25,75 / 42,25 |
| Linux x64, Rosetta | Rullning | 87,54 / 143,73 | 106,83 / 164,35 | 68,19 / 118,77 |
| Linux x64, Rosetta | Breddbyte | 73,40 / 110,88 | 73,08 / 79,38 | 47,23 / 59,46 |
| Linux x64, Rosetta | Alla celler | 82,17 / 127,19 | 80,63 / 126,60 | 63,65 / 76,31 |

På båda ARM64-målen har Dunder lägre median än Ratatui i alla åtta fallen. En ändrad cell per rad blev något långsammare än med föregående Dunder. På Linux ARM64 har breddbyte även högre p95 än Ratatui. På översatt Linux x64 tar rullning, breddbyte och hel omritning längre tid än i Ratatui. Nativ Linux x64 återstår för detta prov.

Processmedianerna för oförändrad bild sträckte sig över följande intervall i µs.

| Plattform | Föregående Dunder | Dunder | Ratatui |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 3,88–4,00 | 0,92–0,92 | 11,04–14,25 |
| Linux ARM64 | 3,96–4,00 | 0,92–0,92 | 12,79–14,42 |
| Linux x64, Rosetta | 30,12–81,25 | 16,42–46,04 | 17,92–48,58 |

## Terminalutdata och minne

Byteantalet omfattar alla 220 bildväxlingar efter den första bilden. För varje process stämdes räknaren av mot antalet byte som lästes från pseudoterminalen. På de tre målen gav de två Dunder-versionerna byteidentiska inspelningar för alla åtta arbetslaster.

| Bildändring | Dunder | Ratatui macOS | Ratatui Linux |
| --- | ---: | ---: | ---: |
| Oförändrad | 0 | 4 180 | 4 180 |
| En cell | 3 960 | 8 800 | 8 800 |
| Enbart färg | 3 960 | 8 800 | 8 800 |
| Sista cellen | 3 960 | 8 800 | 8 800 |
| En cell per rad | 63 360 | 91 300 | 91 300 |
| Rullning | 560 340 | 588 280 | 588 280 |
| Breddbyte | 518 100 | 546 040 | 546 040 |
| Alla celler | 552 090 | 568 480 | 568 480 |

För oförändrad bild visar tabellen medianen för processens högsta RSS i KiB. Allokeringsanrop och begärda byte kommer från separata profilerade processer och avser skillnaden mellan en hel körning och en som stannar direkt efter första bilden. Rådata redovisar även de andra sju arbetslasterna.

| Plattform | Program | Högsta RSS (KiB) | Extra allokeringsanrop | Extra begärda byte |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | Föregående Dunder | 1 744 | 894 | 51 228 |
| macOS ARM64 | Dunder | 1 728 | 894 | 51 216 |
| macOS ARM64 | Ratatui | 2 256 | 221 | 4 335 |
| Linux ARM64 | Föregående Dunder | 1 004 | 894 | 51 228 |
| Linux ARM64 | Dunder | 940 | 894 | 51 228 |
| Linux ARM64 | Ratatui | 1 864 | 220 | 4 271 |
| Linux x64, Rosetta | Föregående Dunder | 4 408 | 894 | 51 240 |
| Linux x64, Rosetta | Dunder | 4 408 | 894 | 51 240 |
| Linux x64, Rosetta | Ratatui | 5 264 | 220 | 4 271 |

## Upprepa på macOS eller Linux

Bygg referensen med de låsta Rust-paketen i lokal cache. Kör båda programmen för varje arbetslast och ersätt `idle` i kommandona med `cell`, `style`, `last`, `scattered`, `scroll`, `resize` eller `full`. Argumentet `setup` efter namnet avslutar direkt efter utskriften av den första bilden. Med `once` visas också första ändringen.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/frame_output.ddr -o build/frame
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --features theme-oracle --example frame_output --target-dir build/reference
ref="$PWD/build/reference/release/examples/frame_output"
build/pty --peak-rss "$PWD/build/frame" -- idle
build/pty --peak-rss "$ref" -- idle
build/frame --cells >build/dunder-cells
"$ref" --cells >build/ratatui-cells
cmp build/dunder-cells build/ratatui-cells
```

Använd `dunder build --runtime-profile` och Rust-funktionen `allocation-counts` för allokeringsräkning. Celljämförelsen omfattar 36 480 råa celler och 13 494 ändringskoordinater i ordning. I en separat kontroll lästes 36 864 synliga celler med förgrundsfärg tillbaka från 16 tmux-panelbilder på macOS. [Beteendeprov](../docs/parity.md#terminalutdata-från-färdiga-bilder)
