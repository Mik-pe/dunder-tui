# Redigering av en körlista i 0.23.0

Körlistan har 23 centrerade rader med tre textspann per rad. Skrollning flyttar första raden till slutet av samma dokument. Vid full omritning växlar två ägda program och bakgrundsfärgen byts. Vid en celländring växlar ett tecken på sista raden. Båda programmen gav samma 1 100 färdiga bilder, färger och markörpositioner. [Dunder](collections.ddr) · [Ratatui 0.30.2](reference/src/collection_scene.rs)

## Tid per bild

Siffrorna är median / p95 i mikrosekunder från sju uppmätta processpar efter två uppvärmningspar. Programmen turades om att starta först. Varje arbetslast ritade 220 bilder och tidtagningen omfattade de sista 200. Ytan var 96 × 24 celler och växlade mellan 80 och 96 kolumner i storleksprovet. På Unix ingår skrivning till en pseudoterminal. På Windows ingår skrivning till en separat Win32-konsol. Rådatan innehåller även minsta och största p50 och p95 mellan processparen, mätningar med utdata i minnet och genomsnittligt antal byte per bild. [Rådata i nanosekunder](results/collections-0.23.0.tsv)

| Arbete | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 10 / 10 | 30 / 36 | 11 / 15 | 47 / 57 |
| En cell | 10 / 11 | 31 / 37 | 12 / 13 | 47 / 59 |
| Skrollning | 19 / 24 | 33 / 37 | 24 / 26 | 48 / 55 |
| Storleksändring | 50 / 56 | 73 / 85 | 33 / 41 | 84 / 95 |
| Full omritning | 52 / 59 | 78 / 86 | 34 / 40 | 72 / 85 |

| Arbete | Linux x64 Dunder | Ratatui | Windows x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 154 / 167 | 56 / 66 | 577 / 648 | 203 / 277 |
| En cell | 157 / 170 | 55 / 65 | 603 / 670 | 203 / 238 |
| Skrollning | 206 / 220 | 57 / 66 | 1135 / 1208 | 220 / 265 |
| Storleksändring | 466 / 518 | 98 / 110 | 1665 / 1965 | 354 / 469 |
| Full omritning | 437 / 454 | 97 / 108 | 1465 / 1753 | 374 / 445 |

Dunder hade lägre median och p95 i alla fem arbetslaster på Apple M5, med Linux ARM64 i en gäst på samma maskin. På Linux x64 och Windows x64 var Ratatui snabbare i alla fem. Linux x64 kördes på en Intel Xeon 6975P-C. Alla fyra terminalerna tog emot 1 349 574 byte från Dunder och 1 602 753 från Ratatui över de 1 100 bilderna. [Linux x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45330011065/job/138228704710) · [Windows Server 2025 x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45330011065/job/138228704923)

## Minne och allokeringar

Högsta minne är medianen från tre separata processer, med RSS på Unix och högsta arbetsminne på Windows. Varje par visar Dunder / Ratatui. Allokeringsproven kör alla 1 100 bilder med tidsmätningen avstängd.

| Värd | Högsta minne i KiB | Allokeringsanrop | Begärda byte |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 1 888 / 2 544 | 8 319 / 47 589 | 480 874 / 1 316 509 |
| Linux ARM64 | 316 / 2 028 | 8 319 / 47 585 | 480 874 / 1 316 310 |
| Linux x64 | 640 / 2 660 | 8 319 / 47 585 | 480 874 / 1 316 397 |
| Windows x64 | 4 004 / 4 264 | saknas / 47 628 | saknas / 1 318 768 |

På de tre Unixvärdarna gjorde Dunder 1 079 färska allokeringar och återanvände poolminne 7 240 gånger. Dunder 0.11.2 på Windows avslutade med kod 0 och utelämnade allokeringsrapporten. [Dunders uppföljning](https://github.com/Mik-pe/dunder/blob/c340de1/docs/next-steps.md#verifiera-windows-som-kompilatorvärd)

## Kör provet

Använd Dunder 0.11.2 och checka ut `v0.23.0`. Referensprogrammets låsta Rustpaket kan hämtas en gång med `cargo fetch --locked --manifest-path benchmarks/reference/Cargo.toml`. På Windows kan samma program mätas lokalt med `TUI_BENCH_SCENE=collections` och `benchmarks/console.ddr` som konsoldrivrutin. [Tidigare mätning](https://github.com/Mik-pe/dunder-tui/actions/runs/45330011065)

```sh
mkdir -p build
export TUI_BENCH_SCENE=collections
dunder build --locked --offline benchmarks/collections.ddr -o build/collections
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example terminal_bench --target-dir build/rust
ref=build/rust/release/examples/terminal_bench
build/collections --verify | "$ref" --verify
build/measure build/collections "$ref" build/pty
```

## Körlistan i 0.23.1

Med den publicerade korrigeringen hade Dunder lägre median och p95 än Ratatui i alla fem arbetslaster på båda ARM64-systemen. Ratatui hade lägre tider i alla fem på båda x64-systemen. Tabellen visar en oförändrad bild i terminalen, i µs. För alla fem arbetslaster finns även motsvarande mätningar i minnet och minsta och största p50 och p95 över sju processpar. [Samtliga mätvärden för 0.23.1](results/collections-0.23.1.tsv)

| Värd | Dunder median / p95 | Ratatui median / p95 |
| --- | ---: | ---: |
| macOS ARM64 | 10 / 11 | 33 / 36 |
| Linux ARM64 | 12 / 15 | 52 / 57 |
| Linux x64 | 151 / 164 | 56 / 66 |
| Windows x64 | 657 / 810 | 160 / 193 |

| Värd | Högsta minne i KiB | Allokeringsanrop | Begärda byte |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 1 888 / 2 544 | 8 319 / 47 589 | 480 874 / 1 316 509 |
| Linux ARM64 | 316 / 1 952 | 8 319 / 47 585 | 480 874 / 1 316 310 |
| Linux x64 | 640 / 2 688 | 8 319 / 47 585 | 480 874 / 1 316 397 |
| Windows x64 | 3 980 / 4 244 | saknas / 47 628 | saknas / 1 318 768 |

Samtliga fyra terminaler fick 1 349 574 byte från Dunder och 1 602 753 byte från Ratatui. Minnesparen anges som Dunder / Ratatui och mättes på samma sätt som i 0.23.0. Kör kommandona ovan från taggen `v0.23.1`. De publicerade körlistorna, maskotarna och animationsgallerierna drevs även med riktiga tangenthändelser i terminalproven. [Linux x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45331074419/job/138232169189) · [Windows x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45331074419/job/138232168889) · [Bildtider före och efter korrigeringen för ligaturer](ligatures.md#vanlig-textritning-i-0231)

## Lokalt med Dunder 0.11.3

Standardbibliotekets nya arrayoperationer används för att ändra buffertstorlek och flytta textrader. Kör samma kommandon som ovan från den här revisionen. Sju processpar per läge gav följande tider för en oförändrad bild i pseudoterminalen. I mätfilen finns alla fem arbetslaster i minne och terminal, inklusive spannet mellan lägsta och högsta p50 och p95. [Alla 60 mätvärden](results/collections-dunder-0.11.3-local.tsv)

| Värd på Apple M5 | Dunder median / p95, µs | Ratatui median / p95, µs |
| --- | ---: | ---: |
| macOS ARM64 | 11,0 / 13,0 | 32,4 / 72,3 |
| Linux ARM64 i lokal VM | 14,8 / 32,8 | 53,8 / 82,2 |
| Linux x86-64 med översättning i samma VM | 111,0 / 132,9 | 58,0 / 74,8 |

På varje system matchade alla 1 100 bilder. Terminalerna fick 1 349 574 byte från Dunder och 1 602 753 från Ratatui. På macOS gav separata profiler 8 962 respektive 47 589 allokeringsanrop och 442 362 respektive 1 316 587 begärda byte. Medianen av tre mätningar med `/usr/bin/time -l` var 1 872 respektive 2 768 KiB i högsta residenta minne. Kör Dunderbygget med `--runtime-profile` och bygg Rustreferensen med `layout-cache,allocation-counts` för att jämföra allokeringar med `--allocations`. Mät minnet på de vanliga byggena. För den här revisionen återstår tidsmätningar på nativa x64-system.
