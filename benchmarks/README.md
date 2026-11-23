# Rendering mot Ratatui 0.30.2

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
