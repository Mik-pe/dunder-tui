# Affisch med logotyp, maskot och programlista

Båda programmen ritar affischen genom terminalens publika ritfunktion. Vänsterspalten skrollar genom fyra programpunkter. Statusraden växlar ett tecken vid en celländring. Vid full omritning växlar hela bakgrunden och maskotens ögonfärg. Samma 1 100 färdiga bilder motsvarar varandra cell för cell, inklusive färger, markörposition och bildnummer. [Dundervyn](artwork.ddr) · [Ratatuis vy](reference/examples/terminal_bench.rs)

## Tid per bild

Tabellerna visar median / p95 i mikrosekunder för en optimerad körbar fil som skriver till en pseudoterminal eller Win32-konsol. Varje rad bygger på sju uppmätta processpar efter två uppvärmningspar. Programmen turas om att starta först. Varje arbetslast ritar 220 bilder, varav 200 ingår i tidtagningen. Bredden är 96 kolumner och växlar mellan 80 och 96 under storleksprovet. Höjden är 24 rader. Råtabellen innehåller också minsta och största p50 och p95 från de sju serierna, antal byte per bild och motsvarande mätningar med utdata i minnet. [Rådata i nanosekunder](results/artwork-0.22.0.tsv)

| Arbete | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 16 / 21 | 32 / 40 | 20 / 23 | 42 / 48 |
| En cell | 17 / 21 | 32 / 40 | 23 / 24 | 42 / 48 |
| Skrollning | 30 / 33 | 37 / 45 | 32 / 37 | 48 / 55 |
| Storleksändring | 62 / 67 | 79 / 87 | 43 / 52 | 69 / 81 |
| Full omritning | 55 / 58 | 75 / 81 | 41 / 47 | 64 / 70 |

| Arbete | Linux x64 Dunder | Ratatui | Windows x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 169 / 185 | 70 / 83 | 865 / 971 | 211 / 293 |
| En cell | 173 / 187 | 70 / 78 | 891 / 945 | 213 / 249 |
| Skrollning | 229 / 243 | 79 / 91 | 1448 / 1604 | 265 / 334 |
| Storleksändring | 450 / 469 | 125 / 141 | 2057 / 2361 | 404 / 496 |
| Full omritning | 365 / 381 | 112 / 120 | 1775 / 2030 | 374 / 425 |

De båda ARM64-proven kördes på en Apple M5 med Linux i en ARM64-gäst. Linux x64 kördes på en AMD EPYC 9V74 80-Core Processor. Windows x64 använde Windows Server 2025 med en separat konsolbuffert. Alla fyra värdar tog emot 1 714 438 byte från Dunder och 1 944 438 byte från Ratatui över de fem arbetslasterna. [Linux x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45327111173/job/138219251357) · [Windows x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45327111173/job/138219251629)

## Minne och allokeringar

För varje par står Dunder först. Högsta minnesanvändning är medianen av tre processer, med RSS på Unix och högsta arbetsminne på Windows. Allokeringsanrop räknas från start till avslut för alla 1 100 bilder.

| Värd | Högsta minne i KiB | Allokeringsanrop | Begärda byte |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 1 808 / 2 576 | 6 803 / 4 482 | 430 497 / 1 758 500 |
| Linux ARM64 | 164 / 2 020 | 6 803 / 4 478 | 430 497 / 1 758 301 |
| Linux x64 | 640 / 2 632 | 6 803 / 4 478 | 430 497 / 1 758 388 |
| Windows x64 | 3 892 / 4 284 | saknas / 4 521 | saknas / 1 760 759 |

Dunder 0.11.2 på Windows avslutade mätprogrammet utan att skriva allokeringsrapporten. På macOS och Linux ARM64 använde Dunder 62 nya allokeringar och återanvände poolminne 6 741 gånger. [Dunders uppföljning av Windowsrapporten](https://github.com/Mik-pe/dunder/blob/main/docs/next-steps.md#verifiera-windows-som-kompilatorvärd)

## Kör provet

Installera Dunder 0.11.2. Hämta referensens låsta Rustpaket en gång med `cargo fetch --locked --manifest-path benchmarks/reference/Cargo.toml`. Följande kommandon bygger båda programmen offline på macOS eller Linux. För Windows finns samma arbetslast i arbetsflödet `native.yml` med valet `scene=artwork`. [Nativ körning av taggen](https://github.com/Mik-pe/dunder-tui/actions/runs/45327111173)

```sh
mkdir -p build
export TUI_BENCH_SCENE=artwork
dunder build --locked --offline benchmarks/artwork.ddr -o build/poster
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example terminal_bench --target-dir build/rust
ref=build/rust/release/examples/terminal_bench
build/poster --verify | "$ref" --verify
build/measure build/poster "$ref" build/pty
```
