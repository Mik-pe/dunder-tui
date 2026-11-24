# Rendering mot Ratatui 0.30.2

## Lånad grafemläsning i 0.21.0

En rad innehåller 32 spann och 248 synliga grafem. Båda programmen summerar teckenföljdens första byte, dess längd och den ärvda kursivstilen. Tiderna nedan är median / p95 i µs för att läsa hela raden. Varje värd körde tre serier. Varje serie omfattar 20 uppvärmningar och 200 mätningar om 16 hela genomläsningar. Tabellen visar medianen av seriernas p50 respektive p95. [Alla serier och kontrollsummor](results/graphemes-0.21.0.tsv) · [Dunder](graphemes.ddr) · [Ratatui](reference/examples/grapheme_bench.rs)

| Värd | Dunders iterator | Dunders `visit_graphemes` | Ratatuis iterator |
| --- | ---: | ---: | ---: |
| macOS ARM64, Apple M5 | 17,75 / 21,38 | 2,75 / 3,13 | 3,15 / 3,77 |
| Linux ARM64, Apple M5 | 19,72 / 20,93 | 2,97 / 3,14 | 3,35 / 3,47 |
| Linux x64, Xeon Platinum 8573C | 261,61 / 268,76 | 20,96 / 22,79 | 8,18 / 9,11 |
| Windows Server 2025 x64 | 425,87 / 440,91 | 81,14 / 86,26 | 7,84 / 9,01 |

På alla tre Unixmål gav 100 genomläsningar med Dunders iterator 174 600 extra allokeringsanrop, varav 12 hämtade nytt minne. Besöksfunktionen gav noll extra anrop. Jämförelsen använder samma rad och drar bort kostnaden för att bygga den. Rusts räknare gav noll extra anrop på alla fyra värdar. Dunder 0.11.2 lämnar ingen allokeringsrapport på Windows. [Nativa x64-körningar](https://github.com/Mik-pe/dunder-tui/actions/runs/45323890237)

```sh
mkdir -p build
dunder build --locked --offline benchmarks/graphemes.ddr -o build/graphemes
build/graphemes iterator timing
build/graphemes visitor timing
cargo run --quiet --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --example grapheme_bench -- timing
dunder build --runtime-profile --locked --offline benchmarks/graphemes.ddr -o build/profile-graphemes
build/profile-graphemes visitor setup
build/profile-graphemes visitor count
```

Samma version kördes dessutom genom riktextvyns fem arbetslaster. Alla 1 100 terminalbilder matchade referensen. De verkliga terminalutskrifterna summerade till 1 382 478 byte för Dunder och 1 624 225 för Ratatui på varje plattform. Den oförändrade bilden tog i median 14,0 / 32,6 µs på macOS ARM64, 15,3 / 39,3 på Linux ARM64, 293,4 / 80,4 på Linux x64 och 757,2 / 211,9 på Windows x64 (Dunder / Ratatui). Högsta RSS eller working set i samma ordning var 2 048 / 2 464, 444 / 2 016, 1 132 / 2 688 och 4 116 / 4 272 KiB. Dunder gjorde 7 125 allokeringsanrop med 272 färska på Unix. Ratatui gjorde 23 184–23 227 anrop över de fyra målen. [Samtliga bildtider, spridning och byte per bild](results/ligatures-0.21.0.tsv) · [Provets körkommandon](ligatures.md#upprepa-mätningen)

## Radfyllning och textspann i 0.20.1

Riktextvyn ger samma 1 100 bilder i Dunder och Ratatui. Tabellen visar median / p95 i µs för en oförändrad bild i terminalen. De två Dunder-versionerna kördes på samma värd med Dunder 0.11.2. Linux körde biblioteksparet och versionsparet i två separata serier. På Windows turades alla tre program om inom samma serie. [Alla arbetslaster mot Ratatui](results/ligatures-0.20.1.tsv) · [Versionsjämförelse med spridning](results/ligatures-before-after-0.20.1.tsv) · [x64-körning, råvärden och instruktionsprofil](https://github.com/Mik-pe/dunder-tui/actions/runs/45315649743)

| Värd | Dunder 0.20.0 | Dunder 0.20.1 | Ratatui 0.30.2 |
| --- | ---: | ---: | ---: |
| macOS ARM64, Apple M5 | 16 / 18 | 14 / 17 | 32 / 36 |
| Linux ARM64, Apple M5 | 18 / 22 | 17 / 19 | 39 / 45 |
| Linux x64, Xeon Platinum 8573C | 362 / 385 | 293 / 314 | 84 / 98 |
| Windows Server 2025 x64 | 909 / 1 060 | 762 / 874 | 217 / 303 |

När ett spann ryms på raden behöver ritningen två nollor för startpositionen. Tidigare skapade den en tupel varje gång. Med två lokala heltal sjönk antalet allokeringsanrop för hela Unix-provet från 83 025 till 7 125, varav 272 färska i båda versionerna. Ratatui gjorde 23 184–23 188 anrop på Unix. Dunder begärde 628 142 byte i 0.20.1. Högsta RSS från tre körningar var 2 048 KiB på macOS, 444 KiB på Linux ARM64 och 1 132 KiB på Linux x64. På Windows var högsta working set 4 108 KiB. Dunders allokeringsräknare på Windows saknar fortfarande slutrapport. Terminalutmatningen var 1 382 478 byte för båda Dunder-versionerna och 1 624 225 byte för Ratatui. [Arbetslast och lokala körkommandon](ligatures.md#upprepa-mätningen) · [Isolerat prov för kompilatorn](language/conditional_pair.ddr)

## Riktext i 0.20.0

Samma vy med 23 textrader gav 1 100 identiska bilder på varje plattform. Tabellen visar median / p95 i µs för en oförändrad bild i pseudoterminal eller Win32-konsol. Minneskolumnerna visar högsta RSS eller Windows working set i KiB från tre separata körningar. [Alla fem arbetslaster, utdata och spridning](results/ligatures-0.20.0.tsv) · [Arbetslast och körkommandon](ligatures.md#upprepa-mätningen) · [Nativa x64-körningar](https://github.com/Mik-pe/dunder-tui/actions/runs/45312061033)

| Värd | Dunder µs | Ratatui µs | Dunder KiB | Ratatui KiB |
| --- | ---: | ---: | ---: | ---: |
| macOS ARM64, Apple M5 | 16 / 20 | 34 / 40 | 2 048 | 2 464 |
| Linux ARM64, Apple M5 | 17 / 22 | 40 / 49 | 444 | 2 016 |
| Linux x64, Xeon 6975P-C | 279 / 295 | 60 / 70 | 1 128 | 2 632 |
| Windows Server 2025 x64 | 1 164 / 1 341 | 223 / 278 | 4 064 | 4 256 |

Terminalerna tog emot 1 382 478 byte från Dunder och 1 624 225 från Ratatui för hela arbetslasten. På Unix rapporterade Dunder 83 025 allokeringsanrop med 272 färska allokeringar per körning. Ratatui rapporterade 23 184–23 188 anrop på Unix och 23 227 på Windows. Dunder 0.11.2 gav ingen allokeringsrapport på Windows.

## Radfyllning i 0.19.0

På samma Linux x64-maskin gick en oförändrad riktextbild från 231 till 183 µs i median när biblioteket begränsade arbetet per rad. I den separata jämförelsen mellan biblioteken tog bilden 187 µs i Dunder och 73 µs i Ratatui. [Fyra plattformar, fem arbetslaster och instruktionsprofil](cell-widgets.md)

## Nativ x86-64 i 0.18.3

På Linux x86-64 tog en oförändrad riktextbild 371 µs i median med Dunder och 59 µs med Ratatui i en pseudoterminal. I en Win32-konsol tog samma bild 1 247 respektive 213 µs. [Fem arbetslaster med bildtid, utmatning och minne](ligatures.md#nativ-x86-64-i-0183)

## Ligaturer och textjustering i 0.18.2

I en vy med centrerade, färgsatta textrader tog skrollning i en pseudoterminal på macOS ARM64 29 µs i median med Dunder och 38 µs med Ratatui. På översatt Linux x86-64 tog samma arbete 234 respektive 63 µs. [Mätvärden för fem arbetslaster](ligatures.md)

## Loggvy i 0.18.1

Loggvyn mäter ritning och omritning av blandad text med sammansatta emoji. På macOS ARM64 tog skrollning 85 µs i median med Dunder och 92 µs med Ratatui. På översatt Linux x86-64 var motsvarande tider 772 och 139 µs. [Alla fem arbetslaster med bildtid, terminalbyte, minne och allokeringar](emoji.md)

## Resultat för 0.11.0

Efter införandet av vyportar jämfördes samma fem terminalarbetslaster och 1 100 bilder igen. På macOS var medianen för hel omritning 163 µs med Dunder och 188 µs med Ratatui. [Tider, terminalbyte, minne och allokeringar](terminal.md#version-0110)

## Resultat för 0.10.0

Terminalprovet kör de publika ritanropen i Dunder och Ratatui med samma diagram och synlig markör. Det kontrollerar de 1 100 fullständiga bilderna och mäter även skrivning till en pseudoterminal. [Bildtid, minne och terminalbyte](terminal.md)

## Resultat för 0.9.0

Ett diagram med tre dataserier mäts i fem arbetslaster med samma färdiga celler för båda implementationerna. [Bildtider, terminalutdata och minne](charts.md)

## Resultat för 0.8.0

En tabell med 4 096 avgångar mäts i sex arbetslaster, med skrollning och hopp mellan ändarna. [Bildtid, minne och terminalutdata](tables.md)

## Resultat för 0.7.0

En vy med kapslade layouter mäts med fem sorters bildändringar. Ett separat prov mäter en varm layout och tre fall där varje ändring av bredden kräver en ny lösning. [Bildtid, layouttid, minne och terminalutdata](layout.md)

Båda programmen bygger om samma vy på 120 × 40 celler, jämför två återanvända bildbuffertar och kodar uppdateringen till en återanvänd bytebuffert. Vid storleksändring växlar bredden mellan 100 och 120 och hela ytan skickas. Rustprogrammet använder `Buffer::set_stringn`, `Buffer::diff_iter` och `CrosstermBackend::draw` med färgutdata påslagen. Rustberoendena hör till referensprogrammet. [Dunderprogram](render.ddr) · [Rustprogram](reference/src/main.rs)

I version 0.5.0 mäts även en komplett diagramvy på macOS och Linux, både i minnet och med skrivning till pseudoterminal. [Diagramvyns mätningar och körinstruktioner](widgets.md)

Version 0.6.0 lägger till en lista med blandade radhöjder och hopp mellan ändarna av 16 384 poster. Tid, allokeringar, minne och utdata redovisas för samma sex arbetslaster. [Listmätningar och körinstruktioner](lists.md)

## Resultat för 0.3.0

Apple M5, macOS 27.2.1, Dunder 0.11.0 med standardoptimeringar och Rust 1.99.1 i releaseprofil. Varje fall kör 20 uppvärmningsbilder och 200 mätbilder. Efter två uppvärmningskörningar växlade programmen körordning sju gånger. Tabellen visar medianen för körningarnas p50 och p95 avrundade till hela mikrosekunder. Dunders klocka hade cirka 1 µs upplösning.

| Ändring | Dunder p50 / p95, µs | Ratatui p50 / p95, µs | Dunder byte/bild | Ratatui byte/bild |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad text | 17 / 21 | 93 / 101 | 0 | 19 |
| Ett tecken | 19 / 22 | 92 / 100 | 9 | 28 |
| Rullande textmönster | 31 / 38 | 111 / 121 | 2 028 | 2 047 |
| Ny bredd | 48 / 58 | 93 / 112 | 4 671 | 4 690 |
| Alla tecken ändras | 55 / 60 | 142 / 154 | 5 071 | 5 090 |
| Oförändrad stiliserad text | 25 / 29 | 99 / 105 | 0 | 19 |
| Ny understrykningsfärg på en cell | 31 / 35 | 100 / 108 | 58 | 79 |
| Ny understrykningsfärg på alla rader | 101 / 107 | 146 / 155 | 5 509 | 5 530 |

De sista tre fallen ritar kursiv, understruken text med RGB-förgrund och RGB-bakgrund. Varje rad väljer en egen understrykningsfärg. För en ändrad sådan cell varierade körningarnas median mellan 29 och 34 µs i Dunder och mellan 90 och 104 µs i Ratatui. Mätningen slutar när ANSI-strömmen finns i minnet.

| Arbetslaster | Dunder allokeringsanrop / begärda byte | Rust allokeringsanrop / begärda byte | Högsta RSS Dunder / Rust, byte |
| --- | ---: | ---: | ---: |
| Fem textfall | 156 / 709 330 | 1 178 / 2 585 257 | 1 835 008 / 2 818 048 |
| Tre stilfall | 123 / 408 621 | 1 595 / 1 441 167 | 1 802 240 / 2 572 288 |

Allokeringsproven kördes utan bildklocka. Dunders runtime räknar även återanvändning från poolen, som stod för 121 respektive 93 anrop. Rust räknar anrop via den globala allokeraren, inklusive omallokering. RSS kommer från en separat körning med `/usr/bin/time -l`.

Stilfallen använder `foreground_rgb`, `background_rgb` och `underline_index` för att bygga stilar direkt från numeriska färgdata. Flaggan `--color-values` skapar i stället Dunders färgvärden `.Rgb(...)` och `.Indexed(...)` på varje ritad rad. Utdata var identiska. Den varianten gjorde 79 546 allokeringsanrop för 2 524 210 byte, varav 79 515 anrop återanvände poolminne. Allokeringskostnaden för dynamiskt skapade färgvärden är fortfarande högre än i Ratatui.

## Kör själv

Kör i repots rot. Varje resultatrad visar arbetslast, p50 och p95 i nanosekunder och genomsnittligt antal kodade byte per bild. Efter första hämtningen av Rustreferensen kan även Cargo köras med `--offline`.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/render.ddr -o build/render
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust
build/render
build/rust/release/ratatui-render-reference

cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features style-oracle
build/render --styles
build/rust/release/ratatui-render-reference --styles
```

Allokeringsprovet kör samma bilder.

```sh
dunder build --locked --offline --runtime-profile benchmarks/render.ddr -o build/render-profile
build/render-profile --allocations
build/render-profile --styles --allocations
build/render-profile --styles --color-values --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features allocation-counts -- --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features style-oracle,allocation-counts -- --styles --allocations
```

## Radbruten, färgsatt Unicode-text i 0.4.0

Båda programmen visar 18 rader ur samma dokument med 48 styckerader, tre textspann per rad, breda tecken, flaggor och sammansatta emoji. Bredden är 60 kolumner och växlar till 44 vid storleksändring. Skrollprovet flyttar en rad per bild genom 48 lägen. Alla 96 kombinationer av bredd och läsposition har jämförts cell för cell. Tidtagningen omfattar nollställning, layout, ritning, celldiff och kodning till en bytebuffert. [Dunder](paragraph.ddr) · [Ratatui](reference/examples/paragraph.rs) · [Bildjämförelsen](../tests/text_reference.ddr)

Apple M5, macOS 27.2.1, Dunder 0.11 och Rust 1.99.1. Två uppvärmningskörningar följdes av sju mätkörningar med roterande startordning. Varje rad nedan visar medianen för körningarnas p50 och p95 i mikrosekunder från 200 bilder efter 20 uppvärmningsbilder.

| Ändring | Paragraph p50 / p95 | Förberedd p50 / p95 | Ratatui p50 / p95 | Byte per bild Dunder / Ratatui |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad | 15 / 16 | 9 / 10 | 27 / 29 | 0 / 19 |
| Skrollning | 31 / 37 | 19 / 20 | 60 / 77 | 2 053 / 2 376 |
| Breddändring | 19 / 24 | 48 / 51 | 27 / 36 | 1 574 / 2 076 |

Vanliga stycken räknar fram raderna fram till vyns nederkant. Förberedd layout räknar vid `resize` om hela dokumentet så att `height()` och hopp till valfri rad har ett färdigt resultat. Under breddändring tog detta längre tid än i Ratatui. Körningarnas p50 för breddändring låg mellan 18 och 23 µs för vanlig layout, mellan 47 och 48 µs för förberedd och mellan 26 och 32 µs för Ratatui.

| Alla tre arbetslaster tillsammans | Allokeringsanrop | Begärda byte | Högsta RSS, byte |
| --- | ---: | ---: | ---: |
| Paragraph | 27 797 | 2 960 252 | 1 916 928 |
| Förberedd | 16 868 | 3 873 698 | 1 933 312 |
| Ratatui | 61 187 | 21 472 547 | 2 801 664 |

Allokerings- och minnesproven inkluderar uppbyggnaden av dokumenten. De kördes separat med samma räknare som ovan. 26 192 respektive 15 254 av Dunders anrop återanvände poolminne.

```sh
dunder build --locked --offline benchmarks/paragraph.ddr -o build/paragraph
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --example paragraph
build/paragraph
build/paragraph --prepared
build/rust/release/examples/paragraph
dunder build --locked --offline --runtime-profile benchmarks/paragraph.ddr -o build/paragraph-profile
build/paragraph-profile --allocations
build/paragraph-profile --prepared --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --features allocation-counts --example paragraph -- --allocations
```

## Sex månadskalendrar

För oförändrad bild, en celländring, sex rullande månader, ny bredd och full omritning finns median, p95, spridning, allokeringar, RSS och faktisk terminalutdata. [Kalendermätningen i 0.12.0](calendar.md)

## Diagram med delad terminalyta

I 0.13.0 mäts även diagrammet med överlåtna celler, en cell som alltid skrivs och en fast utskriftsbredd. Jämförelsen kontrollerar skärmen efter varje storleksändring och mäter utdata i en pseudoterminal. [Resultat och körkommandon](buffers.md)
