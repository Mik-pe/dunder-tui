# Lista med 16 384 poster i 0.6.0

Varje post har en titel. Var tredje post har också en färgad rad med ett brett och ett sammansatt tecken. Båda programmen bygger listans text och stilar före tidtagningen och ritar den från samma valda post, med två posters skrollmarginal. Alla 1 320 bilder och alla tillhörande markeringar och skrollpositioner stämmer mot Ratatui 0.30.2. [Dundervy](list_scene.ddr) · [Rustvy](reference/src/list_scene.rs) · [Jämförelse](../docs/parity.md#listor-i-060)

## Bildtid och terminalutdata

Bilden är 96 × 24 celler. I breddprovet växlar ritytan mellan 80 och 96 kolumner. Vid skrollning flyttas markeringen nedåt en post per bild. Det långa hoppet växlar mellan index 40 och 16 378. Omritningsprovet byter bakgrund på hela listytan. En separat statusrad används för encellsändringen. Provet mäter tömning, val av synliga poster, ritning, celldiff, kodning samt i PTY-läget skrivning till pseudoterminal. Den interaktiva storleksändringen i bokhylleappen provas separat.

Efter två uppvärmningskörningar per program följer sju mätkörningar med växlande startordning. Varje fall har 20 uppvärmningsbilder och 200 mätbilder. Tabellen visar medianen av körningarnas p50 / p95 i µs i PTY-läget. Linuxgästerna kördes på samma Apple M5 som macOS 27.2.1. ARM64 kördes med inbyggd kod och x86-64 med CPU-översättning. Verktygen var Dunder 0.11.0 och Rust 1.99.1 i optimerade byggen. [Mätvärden i ns, inklusive minnesläget och körningarnas min/max](results/lists-0.6.0.tsv)

| Ändring | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui | Byte/bild Dunder / Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 32 / 32 | 65 / 67 | 17 / 22 | 33 / 34 | 148 / 154 | 64 / 70 | 0 / 19 |
| En cell | 35 / 36 | 66 / 67 | 23 / 68 | 33 / 36 | 154 / 162 | 65 / 73 | 8 / 27 |
| Skrollning | 68 / 73 | 93 / 104 | 35 / 47 | 44 / 48 | 213 / 221 | 93 / 101 | 1 379 / 1 549 |
| Ny bredd | 65 / 75 | 91 / 100 | 31 / 44 | 39 / 44 | 190 / 200 | 75 / 89 | 2 372 / 2 657 |
| Hela listytan | 88 / 93 | 117 / 125 | 40 / 71 | 53 / 58 | 274 / 284 | 115 / 120 | 2 339 / 2 502 |
| Långt hopp | 24 / 49 | 59 / 130 | 28 / 32 | 52 / 56 | 169 / 177 | 89 / 95 | 412 / 452 |

Den fysiska terminalströmmen var 1 440 729 byte för Dunder och 1 595 009 byte för Ratatui på alla tre plattformar, inklusive uppvärmningsbilderna. På Mac jämfördes också varje byte i PTY-strömmen med samma programs omdirigerade utdata. Dunder hade lägre mediantid i alla sex PTY-fallen på båda ARM64-plattformarna. På Linux ARM64 var Dunders p95 högre vid encellsändring och omritning av hela listytan. Ratatui var snabbare på den översatta x86-64-processorn. Spridningen på Mac var stor. I breddprovet varierade Dunders p95 från 70 till 2 941 µs och Ratatuis från 95 till 252 µs över sju körningar. Resultatfilen innehåller också en andra serie på Mac.

## Allokeringar och minne

Allokeringarna räknas över programstart, konstruktion av hela listan och alla bilder. Bildklockan var frånkopplad i den körningen. Dunders runtime räknar även anrop som återanvänder poolminne. Rust räknar via en global allokerare. Högsta RSS kommer från tre separata tidsmätta körningar per program och plattform. Tabellen visar median-RSS i KiB.

| Plattform | Allokeringsanrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS Dunder / Ratatui, KiB |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 379 830 / 135 881 | 14 365 828 / 8 693 333 | 21 600 / 8 720 |
| Linux ARM64 | 379 830 / 135 877 | 14 365 828 / 8 693 114 | 21 000 / 8 976 |
| Linux x86-64, översatt | 379 830 / 135 877 | 14 365 828 / 8 693 116 | 27 196 / 12 412 |

Av Dunders anrop återanvände 57 521 redan allokerat minne. Före ändringarna i textlagringen tog samma arbetslast 34,3 MB i högsta RSS på Mac. Fristående textrader lagrar nu sin bredd direkt. Endast stycken bygger index för radbrytning och vanlig ASCII-text behöver inga grafemtabeller. Minnet och antalet allokeringar för den stora listan är fortfarande högre än i Ratatui.

## Kör själv

Kör från repots rot. Båda benchmarkprogrammen använder samma 1 320 bilder i alla lägen. Cargo kan bygga med `--offline` efter första hämtningen av den fristående referensens låsta paket.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/lists.ddr -o build/list-bench
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --example list_bench
build/measure build/list-bench build/rust/release/examples/list_bench build/pty

dunder build --locked --offline --runtime-profile benchmarks/lists.ddr -o build/list-profile
build/list-profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust-profile --features allocation-counts --example list_bench -- --allocations
```

För RSS, kör de två tidsmätta binärerna var för sig med `/usr/bin/time -l` på macOS eller `/usr/bin/time -v` på Linux. För att spara hela strömmen från pseudoterminalen, kör `build/pty build/list-bench build/list.ansi`.

Efter stödet för sparade tillstånd kördes listan med 16 384 poster och tabellen med 4 096 rader om på tre lokala mål. [Bildtider, allokeringar, minne och terminalbyte](tables.md#sparade-markeringar-i-listor-och-tabeller)

Listans aktuella mätning använder full markeringsbredd och kontrollerar även celler som överlappar ett brett tecken. [Sex arbetslaster med Dunder 0.12.1 och Ratatui 0.30.2](tables.md#hela-markörer-och-kolumnspann-med-dunder-0121)
