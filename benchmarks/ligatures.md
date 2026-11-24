# Textspann med ligaturer

Vyn visar 23 centrerade rader med skriftspecifika ligaturer, stilgränser och en statuscell. Båda programmen bygger sina sju textrader före tidtagningen och återanvänder dem. Alla 1 100 färdigritade bilder, stilar, markörpositioner och bildnummer stämmer med Ratatui 0.30.2. Storleken är 96 × 24 celler och växlar mellan 80 och 96 kolumner vid storleksändring. [Dunder](terminal.ddr) · [Ratatui](reference/examples/terminal_bench.rs)

## Tid per bild i en pseudoterminal

Varje par visar median / p95 i mikrosekunder, avrundat till närmaste heltal. Resultatet är medianen av sju körningar efter två uppvärmningar. Programmen turas om att köras först. Linux kördes i en gäst på samma Apple M5 som macOS. x86-64 kördes under översättning. Rådata innehåller även minsta och största uppmätta värde i varje serie. [Rådata i nanosekunder](results/ligatures-0.18.2.tsv)

| Arbete | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Linux x86-64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad bild | 23 / 25 | 36 / 40 | 25 / 27 | 44 / 49 | 221 / 235 | 59 / 67 |
| En cell | 23 / 25 | 35 / 39 | 25 / 29 | 43 / 50 | 221 / 236 | 59 / 66 |
| Skrollning | 29 / 33 | 38 / 43 | 31 / 36 | 45 / 51 | 234 / 256 | 63 / 71 |
| Storleksändring | 54 / 59 | 74 / 86 | 43 / 49 | 69 / 79 | 483 / 534 | 106 / 118 |
| Full omritning | 53 / 57 | 77 / 83 | 40 / 48 | 70 / 78 | 400 / 429 | 106 / 116 |

I den första serien på Linux ARM64 nådde enskilda p95-värden 1,53 ms. Vid en omkörning av hela serien låg Dunders sju p95-värden för skrollning mellan 34 och 36 µs. Sammanlagt läste pseudoterminalen 1 382 478 byte från Dunder och 1 624 225 från Ratatui i alla tre miljöerna.

## Allokering och minne

Dunders profil räknade 82 996 allokeringsanrop och 1 839 634 begärda byte. 265 anrop hämtade nytt minne och 82 731 återanvände poolen. Ratatui räknade 23 188 anrop och 1 265 238 begärda byte på macOS, samt 23 184 anrop på Linux. Medianen av tre processers högsta RSS var 2 032 / 2 480 KiB på macOS, 432 / 1 940 KiB på Linux ARM64 och 6 104 / 5 468 KiB på översatt Linux x86-64, med Dunder först i varje par.

## Upprepa mätningen

Följ [bygg- och mätkommandona för loggvyn](emoji.md#upprepa-mätningen) med `TUI_BENCH_SCENE=ligatures` före verifiering och tidtagning. Mätningen ovan använder Dunder 0.11.2 och Rust 1.99.1 i releaseprofil.
