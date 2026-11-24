# Testterminalens skrollhistorik

Dunder 0.12.0 och Ratatui 0.30.2 körs med samma skärm på åtta kolumner och två rader. Båda bygger och läser skrollhistoriken i minnet. Arbetslasten skriver noll byte till en terminal. [Dunderprogram](scrollback.ddr) · [Rustprogram](reference/examples/scrollback.rs) · [Beteendejämförelse](../docs/parity.md#återställ-testterminalen)

| Prov | En tidsmätt operation |
| --- | --- |
| `blank-limit` | Lägg till 65 535 radbrytningar och sedan två till på en tom skärm. Kontrollera den slutliga historiklängden. |
| `mixed-limit` | Starta med två textrader, kör samma radbrytningar och läs tecknet i den äldsta kvarvarande raden. |
| `painted` | Rita en rad med ASCII, 茶 och 🌊 och skrolla upp en rad. Upprepa 256 gånger och läs den senaste historikraden. |
| `restore` | Läs en JSON-sträng på 120 795 byte med skärm, markör och 128 historikrader. Kontrollera två tecken i den första raden. |

Alla mätningar kördes lokalt på samma Apple M5 med macOS 27.2.1, en Linux ARM64-gäst och en Linux x86-64-gäst med processoröversättning. Rustreferensen byggdes med 1.99.1 i releaseprofil. Programmen växlade startordning i två uppvärmningspar och sju mätpar. Varje process körde tre uppvärmningsprov och 20 mätprov per arbetslast. De två gränsfallen tidsmättes fyra operationer i taget. Rådata visar medianen av processernas p50 och p95 samt minsta och högsta värde. [Operationstider och spridning](results/scrollback-dunder-0.12.0-local.tsv)

Minnesprovet kör varje arbetslast en gång i en egen process. Den högsta minnesanvändningen i KiB är medianen från fem körningar. Allokeringsanrop och begärda byte mäts separat med Dunders runtimeprofil och Rustprogrammets globala allokerare. Dunders räknare inkluderar återbruk ur minnespoolen. För `restore` ingår även skapandet av indatans JSON i resursmätningen. [Allokeringar och högsta RSS](results/scrollback-dunder-0.12.0-resources.tsv)

På macOS ARM64 mättes dessa tider per operation. Värdena är p50 / p95 i mikrosekunder. För `painted` omfattar en operation samtliga 256 rader.

| Prov | Dunder | Ratatui |
| --- | ---: | ---: |
| Tom historik till gränsen | 1,25 / 1,25 | 4 969 / 6 369 |
| Blandad historik till gränsen | 5 / 5,5 | 5 085 / 5 912 |
| 256 målade rader | 265 / 443 | 50 / 57 |
| Återställ från JSON | 3 803 / 6 230 | 1 618 / 2 450 |

Tomma rader delar lagring i Dunder. Vid gränsen på 65 535 tomma rader var högsta RSS 1 712 KiB för Dunder och 26 576 KiB för Ratatui på Macen. När testbackenden flyttade målade rader direkt i den befintliga bildbufferten gick Dunders allokeringsanrop i 256-radersprovet från 7 271 till 3 943. Ratatui använde 25 anrop för samma prov. [Versionen med mellanbuffert](https://github.com/Mik-pe/dunder-tui/tree/d2303bb3b38ec28cc1306b7e14c9c29b3286b2b3) · [Resursmätningar](results/scrollback-dunder-0.12.0-resources.tsv)

## Återställning efter ändringen i färgläsaren

Efter ändringen i färgläsaren gick resursprovet för `restore` från 136 924 till 118 204 allokeringsanrop och från 4 267 808 till 3 625 088 begärda byte på alla tre Unix-mål. I den nya parkörningen på macOS tog återställningen 1,774 ms i median och 1,999 ms vid p95 i Dunder. Ratatui tog 1,049 respektive 1,271 ms. Provet läste samma 120 795 byte och återställde 128 historikrader. [Tider, spridning, allokeringar och RSS per plattform](results/palette-restore-dunder-0.12.0-local.tsv)

## Kör lokalt

```sh
mkdir -p build
dunder build --locked --offline benchmarks/scrollback.ddr -o build/scrollback
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features backend-oracle --example scrollback --target-dir build/rust
build/measure "$PWD/build/scrollback" "$PWD/build/rust/release/examples/scrollback"

dunder build --locked --offline --runtime-profile benchmarks/scrollback.ddr -o build/scrollback-profile
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features backend-oracle,allocation-counts --example scrollback --target-dir build/rust-profile
build/scrollback-profile --resources painted
build/rust-profile/release/examples/scrollback --resources painted
build/scrollback-profile --resources restore
build/rust-profile/release/examples/scrollback --resources restore
```

För RSS, kör de två tidsmätta binärerna med `--resources` och provets namn under `/usr/bin/time -l` på macOS eller `/usr/bin/time -v` på Linux. Cargo behöver den fristående referensens låsta paket i sin lokala cache.
