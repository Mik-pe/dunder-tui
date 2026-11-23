# Tabell med 4 096 avgångar i 0.8.0

Båda programmen ritar fyra kolumner, sidhuvud, sidfot och en markerad cell. Var tredje avgång har en andra färgad rad med ett brett tecken och ett kombinerande tecken. Alla 1 320 bilder, markeringar och skrollpositioner matchar Ratatui 0.30.2. [Dundervy](table_scene.ddr) · [Rustvy](reference/src/table_scene.rs) · [Celljämförelse](../docs/parity.md#tabeller-i-080)

## Bildtid och terminalutdata

Ritytan har 96 × 24 celler. I breddprovet växlar bredden mellan 80 och 96 kolumner. Skrollprovet går ned en rad per bild och långhoppet växlar mellan radindex 40 och 4 090. En separat statusrad används för encellsändringen. Omritningsprovet ändrar bakgrunden i hela tabellytan.

Tiden mäts från tömningen av bildbufferten genom tabellritning, celldiff, kodning och skrivning till pseudoterminal. Efter två uppvärmningskörningar per program följer sju mätkörningar med växlande startordning. Varje körning innehåller 20 uppvärmningsbilder och 200 mätbilder per fall. Tabellen visar medianen av körningarnas p50 / p95 i µs. Linuxgästerna kördes på samma Apple M5 som macOS 27.2.1. Linux x86-64 använde CPU-översättning. Båda programmen var optimerade byggen med Dunder 0.11.1 och Rust 1.99.1. [Alla prov i ns, inklusive minnesläget och spridningen mellan körningar](results/tables-0.8.0.tsv)

| Ändring | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui | Byte/bild Dunder / Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 19 / 20 | 42 / 47 | 25 / 25 | 42 / 46 | 202 / 211 | 75 / 81 | 0 / 19 |
| En cell | 24 / 27 | 42 / 46 | 31 / 34 | 42 / 45 | 208 / 216 | 73 / 78 | 8 / 27 |
| Skrollning | 33 / 40 | 53 / 63 | 39 / 43 | 49 / 54 | 244 / 263 | 83 / 91 | 839 / 1 171 |
| Ny bredd | 47 / 51 | 66 / 75 | 36 / 43 | 49 / 54 | 254 / 269 | 89 / 95 | 2 523 / 3 093 |
| Hela tabellytan | 57 / 61 | 80 / 86 | 49 / 53 | 65 / 70 | 340 / 355 | 111 / 118 | 2 481 / 2 901 |
| Långt hopp | 31 / 35 | 60 / 66 | 38 / 43 | 51 / 56 | 242 / 255 | 93 / 99 | 818 / 1 138 |

Från pseudoterminalerna lästes sammanlagt 1 476 726 byte för Dunder och 1 848 065 för Ratatui per plattform, inklusive uppvärmningsbilderna. På båda ARM64-systemen hade Dunder lägre mediantid i alla sex fall. På den översatta Linux x86-64-gästen var Ratatui snabbare i alla sex.

## Allokeringar och minne

Allokeringsprovet omfattar programstart, uppbyggnad av alla rader och samtliga bilder. Dunders räknare omfattar även anrop som återanvänder poolminne. Rust mäter med en global allokerare. Bildklockan är frånkopplad under räkningen. Högsta RSS är medianen av tre separata körningar per program och plattform.

| Plattform | Allokeringsanrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS Dunder / Ratatui, KiB |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 336 260 / 170 523 | 12 519 319 / 10 401 726 | 19 568 / 10 160 |
| Linux ARM64 | 336 260 / 170 514 | 12 519 319 / 10 400 531 | 18 556 / 9 872 |
| Linux x86-64, översatt | 336 260 / 170 486 | 12 519 319 / 10 389 437 | 23 444 / 13 324 |

## Kör själv

Kör från repots rot. Båda benchmarkprogrammen använder samma 1 320 bilder i alla lägen. Cargo kan bygga med `--offline` efter första hämtningen av den fristående referensens låsta paket.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/tables.ddr -o build/table-bench
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features layout-cache --example table_bench
build/measure build/table-bench build/rust/release/examples/table_bench build/pty

dunder build --locked --offline --runtime-profile benchmarks/tables.ddr -o build/table-profile
build/table-profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust-profile --features layout-cache,allocation-counts --example table_bench -- --allocations
```

För RSS, kör de två tidsmätta binärerna var för sig med `/usr/bin/time -l` på macOS eller `/usr/bin/time -v` på Linux. För att spara hela strömmen från pseudoterminalen, kör `build/pty build/table-bench build/table.ansi`.
