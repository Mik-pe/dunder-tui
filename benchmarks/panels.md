# En layout med 32 paneler

Fönstret har två kolumners marginal på varje sida. Panelerna kombinerar alla sex storleksregler, där två paneler behöver minst 19 respektive 12 celler. Dunder och Ratatui 0.30.2 gav exakt samma paneler och mellanrum för alla 15 uppdelningar i mätprogrammet. [Dunder](panels.ddr) · [Ratatui](reference/examples/panels.rs) · [Rektangelprovet](../tests/layout_reference.ddr)

På macOS ARM64 tog en ny bredd 1,50 µs i median med lika fyllnadsvikter. När en av panelerna fick tre gånger så hög vikt behövdes den allmänna lösaren. Då sjönk Dunders median från 27,81 till 16,36 ms. Ratatui tog 5,75 ms.

## Tid per uppdelning

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

## Allokeringar och minne

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
