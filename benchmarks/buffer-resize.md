# Buffertstorlek och bildtid i 0.24.0

## Ändra storlek på en buffert

Båda programmen växlar mellan två rektanglar och bevarar cellernas linjära ordning. Tabellen visar medianen av fem processers p50 i nanosekunder per anrop. Rådatan innehåller även p95 för varje process. Ett prov består av 512 storleksändringar. Varje process värmer upp med 20 prov och mäter sedan 200. Startordningen växlar mellan Dunder 0.11.2 och Ratatui 0.30.2. I miljoncellsprovet skapas alla celler före tidtagningen. [Dunderprov](reshape.ddr) · [Ratatui-prov](reference/examples/reshape.rs) · [Samtliga 160 processresultat och kontrollsummor](results/buffer-resize-0.24.0.tsv)

| Storlek och innehåll | macOS ARM64 Dunder / Ratatui, p50 | Linux ARM64 | Linux x64 | Windows x64 |
| --- | ---: | ---: | ---: | ---: |
| 80 × 24 ↔ 96 × 20, enkel text | 46 / 4 | 46 / 9 | 211 / 8 | 265 / 9 |
| Samma ytor med breda grafem och en diffregel | 587 / 2 | 619 / 10 | 3 419 / 8 | 5 458 / 9 |
| 80 × 24 ↔ 96 × 24, växande cellantal | 316 / 398 | 785 / 1 034 | 8 191 / 1 270 | 5 753 / 1 425 |
| 1 024 × 1 024 ↔ 2 048 × 512, enkel text | 19 / 2 | 46 / 9 | 211 / 8 | 265 / 9 |

När cellantalet är konstant ändrar Ratatui rektangeln och behåller sin cellvektor. Dunder kontrollerar dessutom de nya radgränserna för breda tecken och räknar om radinformationen som används vid nästa celldiff. Vid växande cellantal på macOS var Dunders median lägre, men dess p95 högre, 1 025 mot 798 ns. Resultaten för en enskild buffert och för en hel bild mäter olika arbete. [Buffertarnas implementation](../src/tui.ddr) · [Jämförelse av varje rå cell efter storleksändring](../tests/large_buffers.ddr)

## En körlista i en verklig terminal

Båda programmen gav samma 1 100 bilder, stilar och markörpositioner. Tabellen visar median / p95 i mikrosekunder per bild från sju uppmätta processpar efter två uppvärmningspar. Macen var en Apple M5. På Linux användes en Neoverse-N2 för ARM64 och en Intel Xeon Platinum 8573C för x64. Windows Server 2025 använde en separat Win32-konsol. Tider för utmatning i minnet, lägsta och högsta resultat mellan processparen samt byte per bild finns i rådata. [Arbetslast och körinstruktioner](collections.md#kör-provet) · [80 mätserier](results/collections-0.24.0.tsv) · [Linux och Windows](https://github.com/Mik-pe/dunder-tui/actions/runs/45333902464)

| Arbete | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 11 / 13 | 33 / 70 | 27 / 27 | 68 / 72 |
| En cell | 11 / 20 | 33 / 67 | 28 / 28 | 69 / 72 |
| Skrollning | 20 / 22 | 33 / 65 | 50 / 51 | 71 / 75 |
| Storleksändring | 41 / 75 | 80 / 147 | 75 / 81 | 119 / 136 |
| Full omritning | 43 / 89 | 80 / 156 | 77 / 79 | 131 / 135 |

| Arbete | Linux x64 Dunder | Ratatui | Windows x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 164 / 176 | 64 / 74 | 431 / 476 | 143 / 171 |
| En cell | 169 / 189 | 64 / 70 | 410 / 469 | 145 / 163 |
| Skrollning | 223 / 248 | 66 / 73 | 525 / 829 | 153 / 171 |
| Storleksändring | 535 / 573 | 116 / 131 | 855 / 1359 | 243 / 298 |
| Full omritning | 476 / 503 | 113 / 123 | 1019 / 1219 | 200 / 295 |

Körlistan gick snabbare i Dunder på båda ARM64-maskinerna och i Ratatui på båda x64-maskinerna i samtliga fem medianer. Dunder skrev 1 349 574 byte och Ratatui 1 602 753 byte på varje plattform. Minnet nedan är medianen av tre processers högsta RSS på Unix eller högsta arbetsminne på Windows. Räknarna omfattar alla 1 100 bilder i en separat körning. Dunder 0.11.2 skrev ingen allokeringsprofil på Windows. [Linux ARM64](https://github.com/Mik-pe/dunder-tui/actions/runs/45333902464/job/138241364659) · [Linux x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45333902464/job/138241364256) · [Windows x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45333902464/job/138241364583)

| Plattform | Högsta minne i KiB, Dunder / Ratatui | Allokeringsanrop | Begärda byte |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 1 872 / 2 544 | 8 962 / 47 589 | 442 362 / 1 316 587 |
| Linux ARM64 | 272 / 2 276 | 8 962 / 47 585 | 442 362 / 1 316 397 |
| Linux x64 | 640 / 2 640 | 8 962 / 47 585 | 442 362 / 1 316 397 |
| Windows x64 | 3 976 / 4 224 | saknas / 47 628 | saknas / 1 318 768 |

## Kör buffertprovet

Bygg Dunderprogrammet från taggen `v0.24.0`. Cargo används för den fristående referensen. Hämta dess låsta paket en gång före offlinebygget. Kör de två programmen i växlande ordning. Argumenten `plain`, `styled`, `grow` och `large` väljer arbetslast.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/reshape.ddr -o build/reshape
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --example reshape --target-dir build/rust
build/reshape styled
build/rust/release/examples/reshape styled
```
