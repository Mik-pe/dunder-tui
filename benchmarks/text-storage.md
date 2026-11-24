# Minnet för text i listor och tabeller

På macOS ARM64 tog det 2,60 ms i median att bygga en tabell med 4 096 rader med Dunder 0.12.1. Före ändringen tog det 3,38 ms och Ratatui 0.30.2 tog 1,32 ms. Varje textspann, rad och dokument lagrar nu sina färger och attribut i två heltal. En sträng utan radmatning flyttas direkt till sitt spann. [Dunderprovet](text_storage.ddr) · [Ratatui-provet](reference/examples/text_storage.rs)

| Uppbyggnad på macOS ARM64 | Dunder före, ms | Dunder nu, ms | Ratatui, ms | Allokeringsanrop i Dunder, före → nu |
| --- | ---: | ---: | ---: | ---: |
| Tabell med 4 096 rader | 3,38 | 2,60 | 1,32 | 329 255 → 224 079 |
| Lista med 4 096 poster | 1,38 | 1,17 | 0,37 | 107 932 → 76 517 |
| 16 384 dokument med enkel text | 2,11 | 1,37 | 0,66 | 229 391 → 131 087 |
| 16 384 dokument med stil på spann, rad och dokument | 2,34 | 2,07 | 0,35 | 213 007 → 163 855 |

Tidtagningen omfattar konstruktionen och slutar före kontroll och frigöring. Varje fall startades i 21 nya processer på samma värd, med roterande körordning. Tabellen visar medianen. Rådata innehåller även p95, lägsta och högsta tid, processens totala allokeringsanrop och medianen av tre separata RSS-prov. [Uppbyggnad på macOS och Linux](results/text-storage-0.12.1.tsv)

## Ritning efter uppbyggnad

Vid skrollning av tabellen i en pseudoterminal var median/p95 på macOS ARM64 36/48 µs för Dunder och 53/62 µs för Ratatui. I den separata jämförelsen mellan Dunderrevisionerna tog samma prov 35/47 µs före och 36/49 µs efter. På Linux ARM64 var tiderna 47/54 µs för Dunder och 55/59 µs för Ratatui. På översatt Linux x86-64 var de 260/319 respektive 87/98 µs. [Alla sex arbetslaster med mätintervall](results/text-frames-0.12.1.tsv)

På Mac sjönk högsta RSS för hela tabellprovet från 19 232 till 16 528 KiB. Ratatui använde 10 112 KiB. För listan med 16 384 poster gick Dunder från 21 648 till 18 512 KiB, mot 8 672 KiB för Ratatui. De båda Dunderrevisionerna skrev identiska terminalströmmar på alla tre Unixmål, 1 499 730 byte för tabellen och 1 441 240 byte för listan. [Resurser före](results/frame-diff-resources-0.12.1.tsv) · [Resurser efter](results/text-frame-resources-0.12.1.tsv) · [Metod och körinstruktioner för bildmätningen](tables.md#kör-själv)

## Kör uppbyggnadsprovet lokalt

Kör från repots rot med Dunder 0.12.1. Kommandot för den fristående Rustreferensen använder Cargos lokala paketcache. Byt `table` mot `list`, `plain` eller `styled` för de andra fallen. Sista värdet på varje utdatarad är byggtiden i nanosekunder.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/text_storage.ddr -o build/storage
dunder build --locked --offline --runtime-profile benchmarks/text_storage.ddr -o build/storage-profile
build/storage table
build/storage-profile table
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --target-dir build/reference --features layout-cache --example text_storage
build/reference/release/examples/text_storage table
```

För att upprepa föreprovet, kopiera samma mätprogram till revisionen som föregick ändringen och bygg det med samma kompilator.

```sh
baseline=$(mktemp -d)
git archive 017c103c37eb7b3d4623261fb9dab89e5078394a | tar -xf - -C "$baseline"
cp benchmarks/text_storage.ddr "$baseline/benchmarks/"
dunder build --locked --offline "$baseline/benchmarks/text_storage.ddr" -o build/storage-before
build/storage-before table
```
