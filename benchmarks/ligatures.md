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

## Nativ x86-64 i 0.18.3

Båda programmen kördes på samma värd per operativsystem. Linux använde en pseudoterminal på Ubuntu 24.04 under KVM på en Intel Xeon 6975P-C. På Windows Server 2025 skrev programmen till en Win32-konsolbuffert med VT och UTF-8. Drivrutinen läste tillbaka statuscellen på rad 24 efter varje process. Båda plattformarna använde Dunder 0.11.2 och sju uppmätta processpar efter två uppvärmningar. Tabellen visar median / p95 i mikrosekunder. [Linuxprovet](https://github.com/Mik-pe/dunder-tui/actions/runs/45303881483/job/138142828857) · [Windowsprovet](https://github.com/Mik-pe/dunder-tui/actions/runs/45303881483/job/138142828516) · [Råvärden och spridning](results/native-x64-0.18.3.tsv)

| Arbete | Linux Dunder | Linux Ratatui | Windows Dunder | Windows Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 371 / 380 | 59 / 70 | 1 247 / 1 427 | 213 / 289 |
| En cell | 369 / 380 | 61 / 70 | 1 259 / 1 320 | 214 / 248 |
| Skrollning | 387 / 397 | 63 / 71 | 1 580 / 1 665 | 244 / 293 |
| Storleksändring | 638 / 706 | 101 / 110 | 1 969 / 2 307 | 362 / 483 |
| Full omritning | 573 / 586 | 96 / 106 | 1 773 / 2 080 | 377 / 436 |

På båda systemen tog Dunder längre tid i alla fem fallen, även i minnet. Över 1 100 bilder tog terminalerna emot 1 382 478 byte från Dunder och 1 624 225 byte från Ratatui. Högsta RSS på Linux var i median 1 088 respektive 2 708 KiB över tre processer. På Windows avlästes högsta arbetsminne med `K32GetProcessMemoryInfo` under körningen och efter avslut. Medianen var 4 076 respektive 4 252 KiB. [Windowsdrivrutinen](console.ddr)

Linux räknade 82 996 allokeringsanrop i Dunder och 23 184 i Ratatui. På Windows registrerade Ratatui 23 227. Dunders profilerade program på Windows avslutades normalt utan att skriva sin räknarrapport. [Dunders Windowsuppföljning](https://github.com/Mik-pe/dunder/blob/main/docs/next-steps.md#verifiera-windows-som-kompilatorvärd)

Callgrind räknade 497 802 698 exekverade instruktioner i ett separat Dunderprogram på Linux x86-64 som ritade den oförändrade vyn 220 gånger. `Frame::fill_cells` stod för 25,74 % och `Frame::same_row` för 21,11 % i funktionernas egna instruktioner. Procenttalen gäller hela processen inklusive första bilden och uppstarten. [Profil med instruktionsadresser](https://github.com/Mik-pe/dunder-tui/actions/runs/45304057607/job/138143422641) · [Funktionerna i Frame](../src/tui.ddr)

## Allokering och minne

Dunders profil räknade 82 996 allokeringsanrop och 1 839 634 begärda byte. 265 anrop hämtade nytt minne och 82 731 återanvände poolen. Ratatui räknade 23 188 anrop och 1 265 238 begärda byte på macOS, samt 23 184 anrop på Linux. Medianen av tre processers högsta RSS var 2 032 / 2 480 KiB på macOS, 432 / 1 940 KiB på Linux ARM64 och 6 104 / 5 468 KiB på översatt Linux x86-64, med Dunder först i varje par.

## Upprepa mätningen

Följ [bygg- och mätkommandona för loggvyn](emoji.md#upprepa-mätningen) med `TUI_BENCH_SCENE=ligatures` före verifiering och tidtagning. Mätningen ovan använder Dunder 0.11.2 och Rust 1.99.1 i releaseprofil.

Kör `gh workflow run native.yml` i det publika repot för att upprepa x64-serierna med den publicerade kompilatorn. Arbetsflödet skriver verktygsversioner, bildtider, bytesummor och minnesvärden till körloggen. Med `-f profile=true` räknar Callgrind dessutom exekverade instruktioner på Linux för den oförändrade vyn. [Bygg- och mätkommandon](../.github/workflows/native.yml)

## Vanlig textritning i 0.23.1

I 0.23.0 delade en fristående textrad ritloop med stycken som tillåter ett brett tecken i sista kolumnen. På Linux x64 steg medianen för en oförändrad ligaturbild från 224 till 232 µs mot 0.22.0 i en parad mätning. I 0.23.1 har den vanliga raden en egen utskriftsloop och återanvänder spannets bredd. [Jämförelse med 0.22.0](https://github.com/Mik-pe/dunder-tui/actions/runs/45329696568/job/138227651497)

| Terminal | 0.23.0 median / p95 i µs | 0.23.1 median / p95 i µs |
| --- | ---: | ---: |
| macOS ARM64 | 17 / 18 | 16 / 18 |
| Linux ARM64 | 17,8 / 19,8 | 17,1 / 19,5 |
| Linux x64 | 232 / 245 | 217 / 230 |
| Windows x64 | 764 / 940 | 754 / 879 |

Tabellen visar den oförändrade bilden. Rådata omfattar även en celländring, skrollning, storleksändring och full omritning för båda versionerna i minnet och i terminalen. Varje mätpar kördes på samma värd. Båda Dunderprogrammen matchade referensens 1 100 bilder och skrev 1 382 478 byte till terminalen. [Spridning och samtliga bildtider](results/ligatures-before-after-0.23.1.tsv) · [Nativ Linux och Windows](https://github.com/Mik-pe/dunder-tui/actions/runs/45330521588)
