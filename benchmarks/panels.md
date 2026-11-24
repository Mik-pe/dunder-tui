# En layout med 32 paneler

Fönstret har två kolumners marginal på varje sida. Panelerna kombinerar alla sex storleksregler, där två paneler behöver minst 19 respektive 12 celler. Dunder och Ratatui 0.30.2 gav exakt samma paneler och mellanrum för alla 15 uppdelningar i mätprogrammet. [Dunder](panels.ddr) · [Ratatui](reference/examples/panels.rs) · [Rektangelprovet](../tests/layout_reference.ddr)

Den viktade layouten tog 1,21 µs i median på macOS ARM64 med Dunder och 3,90 ms med Ratatui. I en mixer som faktiskt skrev bilden till en pseudoterminal tog storleksändring 30 / 35 µs i Dunder och 332 / 394 µs i Ratatui (median / p95). [Alla 273 processresultat](results/weighted-panels-0.12.1.tsv)

## Viktade paneler vid storleksändring

Det första provet har 32 paneler och en plats i layoutcachen. Det växlar mellan 160, 161 och 162 kolumner. Det andra mäter en uppdelning för varje bredd från 35 till 225 efter fem uppvärmningar. Vid 87–136 kolumner ryms minimikraven samtidigt som andra paneler måste krympa. Dunder fördelar först platsen efter regelprioritet. Bara panelerna som konkurrerar på samma prioritetsnivå behöver vägas mot sina grannar. När golven och övriga önskemål ryms delas utrymmet efter fyllnadsvikt. Stora vikter eller överlapp som skulle flytta en kant utanför fönstret kan kräva den allmänna lösaren.

Tabellen visar median / p95 i mikrosekunder per uppdelning inklusive läsning av paneler och mellanrum. För tre närliggande bredder är kontrollsumman 172 040 för alla tre programmen. Över hela intervallet kan likvärdiga indelningar ge olika summor. Båda axlarnas 382 uppdelningar kontrolleras därför rektangel för rektangel, med exakt kontroll av alternativa optima. [Dunderprov](../tests/layout_reference.ddr) · [Ratatui och oberoende kontroll](reference/examples/layout.rs)

| Plattform | Arbete | Föregående Dunder | Dunder | Ratatui |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | Tre närliggande bredder | 12 119 / 12 202 | 1,21 / 1,25 | 3 900 / 4 232 |
| macOS ARM64 | Hela breddintervallet | 10 639 / 28 805 | 1,17 / 18,96 | 3 632 / 5 298 |
| Linux ARM64 | Tre närliggande bredder | 12 782 / 12 989 | 1,25 / 1,38 | 4 546 / 5 013 |
| Linux ARM64 | Hela breddintervallet | 11 291 / 30 553 | 1,21 / 19,29 | 3 781 / 5 673 |
| Översatt Linux x86-64 | Tre närliggande bredder | 91 435 / 92 852 | 13,17 / 15,92 | 5 271 / 5 769 |
| Översatt Linux x86-64 | Hela breddintervallet | 79 603 / 214 864 | 12,79 / 173 | 4 446 / 6 687 |

## Den viktade mixern i terminalen

Tolv färgade band kombinerar de sex storleksreglerna i en vy på 96 × 24 celler. Vid storleksändring växlar den med 80 × 24 och en plats i layoutcachen. Alla 1 100 bilder matchade Ratatuis celler och stilar på samtliga tre system. Dunders terminalström var byteidentisk med föregående version och innehöll 2 331 370 byte. Ratatui skrev 3 330 672 byte. Ett separat interaktivt prov ändrar pseudoterminalens verkliga bredd sex gånger och läser hela panelraden vid 54, 80, 96 och 125 kolumner. [Mixer](bands.ddr) · [Interaktivt terminalprov](../tests/weighted_terminal.ddr)

Tiderna nedan inkluderar ritning, diff och skrivning till pseudoterminalen. Enheten är mikrosekunder per bild, median / p95.

| Plattform | Arbete | Föregående Dunder | Dunder | Ratatui |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | Oförändrad bild | 4,00 / 4,00 | 4,00 / 4,00 | 22,00 / 22,75 |
| macOS ARM64 | En cell | 6,00 / 7,00 | 6,00 / 7,00 | 21,83 / 22,29 |
| macOS ARM64 | Skrollning | 38,00 / 45,00 | 39,00 / 44,00 | 60,00 / 66,67 |
| macOS ARM64 | Storleksändring | 435 / 558 | 30,00 / 35,00 | 332 / 394 |
| macOS ARM64 | Full omritning | 38,00 / 43,00 | 38,00 / 44,00 | 74,38 / 82,00 |
| Linux ARM64 | Oförändrad bild | 3,83 / 3,96 | 3,83 / 4,79 | 25,17 / 26,79 |
| Linux ARM64 | En cell | 8,04 / 12,29 | 8,04 / 10,21 | 24,75 / 26,71 |
| Linux ARM64 | Skrollning | 26,58 / 27,62 | 26,46 / 27,54 | 45,17 / 47,33 |
| Linux ARM64 | Storleksändring | 459 / 569 | 18,17 / 20,83 | 348 / 410 |
| Linux ARM64 | Full omritning | 26,42 / 27,75 | 26,42 / 27,33 | 59,25 / 62,25 |
| Översatt Linux x86-64 | Oförändrad bild | 29,08 / 30,25 | 29,12 / 32,08 | 36,67 / 40,54 |
| Översatt Linux x86-64 | En cell | 36,00 / 40,12 | 35,96 / 40,62 | 36,62 / 40,42 |
| Översatt Linux x86-64 | Skrollning | 140 / 146 | 138 / 145 | 72,38 / 83,12 |
| Översatt Linux x86-64 | Storleksändring | 3 239 / 4 415 | 130 / 143 | 432 / 509 |
| Översatt Linux x86-64 | Full omritning | 153 / 160 | 153 / 168 | 97,92 / 119 |

Varje process mätte 200 bilder efter 20 uppvärmningsbilder per arbetslast. En uppvärmningsprocess följdes av fem mätprocesser per implementation på ARM64 och tre på översatt x86-64. De tre implementationerna växlade startordning och plattformarna mättes i tur och ordning på en Apple M5 med macOS 27.2.1 och lokal Linux-VM. Dunder 0.12.1 och Rust 1.99.1 byggde respektive program. Högsta enskilda p95 över alla fem terminalarbetslasterna och alla mätserier för Dunder var 45 µs på macOS ARM64 och 27,96 µs på Linux ARM64. På översatt x86-64 var Ratatui fortfarande snabbare vid skrollning och full omritning. [Rådata med spridning mellan processerna](results/weighted-panels-0.12.1.tsv) · [Föregående implementation](https://github.com/Mik-pe/dunder-tui/tree/7b0a4a5e1cf1f640ba6785645eeec73e272cbe3b)

## Resurser för viktade paneler

Varje cell visar antal allokeringsanrop / begärda byte / högsta RSS i KiB. För uppdelningarna subtraheras uppstart och uppbyggnad av reglerna från allokeringsräknarna. För mixern omfattar de samtliga 1 100 bilder inklusive uppstart. RSS kommer från en separat full körning. Begärda byte summeras även när lagringen kan återanvändas.

| Plattform | Arbete | Föregående Dunder | Dunder | Ratatui |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | Tre närliggande bredder | 144 768 / 32 544 845 / 2 384 | 348 / 28 885 / 1 584 | 109 342 / 103 029 343 / 5 360 |
| macOS ARM64 | Hela breddintervallet | 954 346 / 216 746 173 / 2 368 | 11 700 / 749 113 / 1 600 | 706 464 / 605 604 343 / 5 904 |
| macOS ARM64 | 1 100 terminalbilder | 318 275 / 42 357 002 / 1 968 | 1 791 / 434 738 / 1 808 | 252 123 / 81 259 454 / 3 152 |
| Linux ARM64 | Tre närliggande bredder | 144 768 / 32 544 845 / 1 452 | 348 / 28 885 / 812 | 109 336 / 102 986 831 / 3 476 |
| Linux ARM64 | Hela breddintervallet | 954 346 / 216 746 173 / 1 452 | 11 700 / 749 113 / 748 | 705 988 / 604 059 439 / 3 520 |
| Linux ARM64 | 1 100 terminalbilder | 318 275 / 42 357 002 / 444 | 1 791 / 434 738 / 128 | 252 085 / 81 216 879 / 2 096 |
| Översatt Linux x86-64 | Tre närliggande bredder | 144 768 / 32 544 845 / 5 040 | 348 / 28 895 / 4 136 | 109 350 / 103 616 943 / 6 936 |
| Översatt Linux x86-64 | Hela breddintervallet | 954 346 / 216 746 177 / 5 036 | 11 700 / 749 123 / 4 284 | 700 610 / 574 123 919 / 7 064 |
| Översatt Linux x86-64 | 1 100 terminalbilder | 318 275 / 42 357 002 / 5 848 | 1 791 / 434 738 / 5 736 | 251 780 / 81 795 919 / 5 544 |

```sh
mkdir -p build
dunder build --locked --offline tests/layout_reference.ddr -o build/layout
dunder build --locked --offline benchmarks/panels.ddr -o build/panels
dunder build --locked --offline benchmarks/bands.ddr -o build/bands
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features layout-cache --example panels --example bands --example layout --target-dir build/reference
ref=build/reference/release/examples
for mode in weighted-limits weighted-resize; do
  build/layout "--$mode" | "$ref/layout" "--$mode"
done
for mode in weighted sweep; do build/panels "$mode"; "$ref/panels" "$mode"; done
build/bands --check --weighted >build/bands-cells
"$ref/bands" --check --weighted >build/ref-cells
cmp build/bands-cells build/ref-cells
build/pty --peak-rss "$PWD/build/bands" build/weighted.ansi -- --weighted
build/pty --peak-rss "$PWD/$ref/bands" build/ref-weighted.ansi -- --weighted
dunder run --locked --offline tests/weighted_terminal.ddr
```

Upprepa de två terminalproven med Dunder byggd med `--runtime-profile`, Rustfeatures `layout-cache,allocation-counts` och argumentet `--allocations` efter `--weighted` för allokeringsräknarna.

## Mätning före den viktade fördelningen

På macOS ARM64 tog en ny bredd 1,50 µs i median med lika fyllnadsvikter. När en av panelerna fick tre gånger så hög vikt behövdes den allmänna lösaren. Då sjönk Dunders median från 27,81 till 16,36 ms. Ratatui tog 5,75 ms.

### Tid per uppdelning

Tabellen visar median / p95 i mikrosekunder per anrop, inklusive läsning av alla 32 paneler och 33 mellanrum. Normal bredd växlar mellan 160, 161 och 162 kolumner. Det trånga fönstret växlar mellan 79, 80 och 81. Överlapp använder ett avstånd på −1, övriga ett avstånd på 1. Båda programmen har en plats i layoutcachen.

| Plattform | Arbete | Dunder före | Dunder nu | Ratatui |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | Normal bredd | 87 486 / 135 385 | 1,50 / 1,58 | 4 467 / 6 228 |
| macOS ARM64 | Trångt | 12 534 / 21 015 | 1,54 / 1,62 | 4 926 / 6 812 |
| macOS ARM64 | Överlapp | 24 830 / 33 244 | 1,46 / 1,50 | 4 334 / 6 555 |
| macOS ARM64 | Olika vikter | 27 807 / 36 139 | 16 360 / 22 208 | 5 748 / 8 225 |
| macOS ARM64 | Samma bredd | 0,083 / 0,084 | 0,083 / 0,125 | 0,125 / 0,209 |
| Linux ARM64 | Normal bredd | 79 602 / 95 349 | 1,50 / 1,54 | 4 861 / 7 460 |
| Linux ARM64 | Trångt | 11 687 / 24 950 | 2,92 / 3,04 | 4 812 / 6 908 |
| Linux ARM64 | Överlapp | 27 240 / 37 684 | 1,50 / 1,58 | 4 829 / 6 350 |
| Linux ARM64 | Olika vikter | 29 147 / 38 484 | 15 902 / 25 027 | 5 178 / 8 512 |
| Linux ARM64 | Samma bredd | 0,084 / 0,125 | 0,167 / 0,209 | 0,167 / 0,167 |
| Översatt Linux x86-64 | Normal bredd | 458 179 / 476 957 | 14,6 / 14,8 | 5 443 / 6 576 |
| Översatt Linux x86-64 | Trångt | 70 251 / 80 730 | 14,8 / 15,3 | 4 984 / 5 725 |
| Översatt Linux x86-64 | Överlapp | 146 440 / 153 734 | 13,9 / 14,0 | 5 533 / 6 084 |
| Översatt Linux x86-64 | Olika vikter | 150 604 / 161 482 | 97 790 / 103 558 | 5 964 / 6 535 |
| Översatt Linux x86-64 | Samma bredd | 0,875 / 0,917 | 0,917 / 0,917 | 0,250 / 0,292 |

Mätningen kördes på en Apple M5 med macOS 27.2.1 och lokal Linux-VM. Linux x86-64 använde processoröversättning. Båda Dunderbyggena använde 0.12.1 och referensen Rust 1.99.1 i releaseprofil. En uppvärmningsprocess följdes av fem mätta processer per program och arbetslast på ARM64 och tre på översatt x86-64. Startordningen roterade. Varje process värmde upp med fem uppdelningar och mätte sedan 25. Tabellen tar medianen av processernas p50 respektive p95. Mac- och Linux-serierna delade värddator med pågående verifieringar. För den viktade layouten på macOS låg Dunders processmedianer mellan 14,88 och 18,29 ms och Ratatuis mellan 4,50 och 6,45 ms. Cacheträffar på ARM64 tog bara ett fåtal klocksteg. [Samtliga 195 mätserier med kontrollsummor](results/mixed-panels-0.12.1.tsv) · [Föregående bibliotek](https://github.com/Mik-pe/dunder-tui/tree/81d9c4460bafe84ae1d3eff38b76ed4360070dcf)

### Allokeringar och minne

Över 30 viktade uppdelningar på macOS sjönk antalet allokeringsanrop från 49 452 408 till 144 768 när lösaren återanvände termlagringen. Ratatui gjorde 109 523 anrop. Tabellen visar **anrop / begärda byte / högsta RSS i KiB**. Räknarna kommer från separata profiler där programstart och uppbyggnad av layoutreglerna har dragits av. Högsta RSS gäller en hel separat process per program och arbetslast.

| macOS ARM64 | Dunder före | Dunder nu | Ratatui |
| --- | ---: | ---: | ---: |
| Normal bredd | 137 941 188 / 2 278 587 009 / 2 944 | 348 / 28 885 / 1 584 | 110 347 / 107 247 063 / 5 488 |
| Trångt | 20 211 628 / 359 521 795 / 2 576 | 348 / 28 875 / 1 584 | 102 289 / 59 121 599 / 4 000 |
| Överlapp | 48 057 758 / 809 872 517 / 2 960 | 348 / 28 877 / 1 584 | 110 422 / 107 887 879 / 5 552 |
| Olika vikter | 49 452 408 / 835 883 245 / 2 928 | 144 768 / 32 544 845 / 2 352 | 109 523 / 103 909 071 / 5 360 |
| Samma bredd | 4 607 471 / 76 115 065 / 2 944 | 203 / 10 299 / 1 584 | 3 706 / 3 567 355 / 5 024 |

För den viktade layouten på Linux ARM64 var högsta RSS 2 156 / 1 452 / 3 400 KiB i samma ordning. På översatt Linux x86-64 var den 5 476 / 5 040 / 6 772 KiB. Den trånga layouten gav 348 allokeringsanrop över 30 uppdelningar i Dunder på alla tre plattformar. [Allokeringar, högsta RSS och bildtider](results/mixed-panels-0.12.1.tsv) · [Fristående prov för arrayer av små poster](language/sparse_terms.ddr)

## Rektanglar och terminalutdata

Med lika fyllnadsvikter och fast avstånd kan Dunder fördela utrymmet jämnt mellan de flexibla panelerna. När alla minimibredder ryms får paneler som behöver mer än sin andel först sin minimibredd. När enbart minimibredderna fyller fönstret begränsas i stället varje andel uppåt av panelens minimibredd. Den direkta beräkningen kräver att de sammanlagda vikterna för tillväxt, likhet och grannpaneler ligger under den lägsta vikten för en fast önskad storlek. Vid konkurrerande krav används den allmänna lösaren. [Layoutberäkning](../src/layout.ddr) · [Lösare](../src/layout_solver.ddr)

Mätprogrammet delar rektanglar i minnet. För terminalkontrollen jämfördes också två vyer om vardera 1 100 bilder med Ratatui. Mixern skrev samma 2 328 959 byte i pseudoterminalen före och efter ändringen på macOS ARM64, Linux ARM64 och översatt Linux x86-64. [Mixerprovet](bands.md) · [Större panelkombinationer](../docs/parity.md#blandade-regler-för-32-och-48-paneler)

## Kör lokalt

Dunder bygger bibliotek och exempel offline. Cargo använder den separata referensens låsta paket i sin lokala cache.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/panels.ddr -o build/panels
dunder build --locked --offline tests/layout_reference.ddr -o build/layout
cargo build --locked --release --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features layout-cache --example panels --example layout --target-dir build/reference
ref=build/reference/release/examples
for mode in minimums crowded overlap weighted cached; do
  build/panels "$mode"
  "$ref/panels" "$mode"
done
build/layout --benchmark-panels | "$ref/layout" --benchmark-panels
build/layout --minimum-levels | "$ref/layout" --minimum-levels
build/layout --mixed-panels >build/mixed-panels
"$ref/layout" --recorded-panels <build/mixed-panels
"$ref/layout" --mixed-panels <build/mixed-panels
```

För en levande jämförelse anger `--mixed-panels 4 15` högst fyra referensförsök på 15 sekunder per indelning. Om en indelning fortfarande saknar svar prövas Dunders rektanglar med exakt aritmetik och programmet avslutar med status 2. Ett aktuellt referenssvar för fall 73 kan skrivas ut med `"$ref/layout" --reference-regions "$(sed -n '73p' build/mixed-panels)"`. Varje sparad rad börjar med indatans riktning, flexläge, avstånd, marginaler, fönster och storleksregler. Efter tabulatorn följer alla paneler och sedan alla mellanrum som `x y bredd höjd`. [Sparade svar från Ratatui 0.30.2](reference/fixtures/mixed-panels-0.30.2.tsv)

```sh
dunder build --locked --offline --runtime-profile benchmarks/panels.ddr -o build/panels-profile
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak-rss
cargo build --locked --release --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features layout-cache,allocation-counts --example panels --target-dir build/reference-profile
for program in build/panels-profile build/reference-profile/release/examples/panels; do
  "$program" weighted setup
  "$program" weighted
done
build/peak-rss "$PWD/build/panels" weighted
build/peak-rss "$PWD/$ref/panels" weighted
```
