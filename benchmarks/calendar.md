# Sex månadskalendrar i 0.12.0

Båda programmen förbereder tolv månader med datumstilar och ritar sex åt gången i en terminal på 96 × 24 celler. Encellsprovet växlar ett tecken längst ner. Bläddring flyttar alla månader ett steg, storleksändring växlar mellan 80 och 96 kolumner och full omritning växlar bakgrund på hela bilden. En separat körning jämför alla 1 100 bilder cell för cell med Ratatui 0.30.2. [Dunder](calendar.ddr) · [Rust](reference/examples/calendar_bench.rs)

## Bildtid i pseudoterminal

Varje par i tabellen visar median · p95 i mikrosekunder, avrundat. D står för Dunder 0.11.2 och R för Ratatui 0.30.2, byggd med Rust 1.99.1 i releaseprofil med layoutcache. macOS-värden körde 27.2.1. Varje program kördes två gånger för uppvärmning och sedan sju gånger med växlande körordning. I varje körning ingick 20 uppvärmningsbilder och 200 mätbilder per arbetslast. Tabellen visar medianen av sju körningar. Minsta och största p50 och p95 finns per fall i rådata, både för fall med terminal och med utdata i minnet. [Alla 60 mätserier](results/calendar-0.12.0.tsv) · [Mätprogram](measure.ddr)

| Arbetslast | macOS ARM64 D / R | Linux ARM64 D / R | Linux x86-64 översatt D / R | Byte per bild D / R |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad | 27 · 30 / 41 · 44 | 27 · 29 / 41 · 45 | 265 · 282 / 68 · 75 | 0 / 19 |
| En cell | 32 · 35 / 41 · 47 | 33 · 36 / 41 · 46 | 273 · 285 / 68 · 78 | 20 / 43 |
| Bläddra | 63 · 67 / 72 · 78 | 51 · 55 / 56 · 60 | 407 · 432 / 91 · 98 | 2 659 / 3 334 |
| Ny bredd | 75 · 83 / 99 · 113 | 57 · 62 / 65 · 73 | 455 · 484 / 106 · 116 | 4 902 / 9 918 |
| Full omritning | 84 · 92 / 117 · 133 | 67 · 73 / 82 · 90 | 545 · 567 / 137 · 150 | 4 842 / 9 474 |

Dunder skickade sammanlagt 2 746 241 byte till pseudoterminalen, Ratatui 5 041 140 byte inklusive uppvärmningsbilderna. I just denna sexmånadersvy hade Dunder lägre median och p95 för samtliga fem fall på macOS och Linux ARM64. På Linux x86-64 kördes båda programmen översatta på samma Apple M5-värd. Där hade Ratatui lägre bildtid i samtliga fall. En jämförelse på fysisk x86-64 återstår.

## Allokeringar och minne

Allokeringsräknarna kördes separat från tidtagningen och omfattar uppstart och 1 100 bilder. Dunder räknar även poolåterbruk. 438 anrop gav färska allokeringar och 128 127 återanvände poolminne. Rust använde en räknande global allokerare. RSS är medianen av tre nya processers maximala residenta minne.

| Plattform | Allokeringsanrop D / R | Begärda byte D / R | Högsta RSS D / R, KiB |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 128 565 / 373 260 | 3 654 000 / 25 405 126 | 1 808 / 2 816 |
| Linux ARM64 | 128 565 / 373 243 | 3 654 000 / 25 400 023 | 400 / 2 080 |
| Linux x86-64, översatt | 128 565 / 373 240 | 3 654 000 / 25 407 586 | 5 572 / 5 644 |

## Upprepa

Kör från repots rot. Dunder bygger och kör offline. Rustprogrammet behöver de låsta referenspaketen i sin Cargo-cache för att också kunna byggas offline.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/calendar.ddr -o build/calendar
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features calendar-oracle,layout-cache --example calendar_bench
build/calendar --verify | build/rust/release/examples/calendar_bench --verify
build/measure build/calendar build/rust/release/examples/calendar_bench build/pty
dunder build --runtime-profile --locked --offline benchmarks/calendar.ddr -o build/calendar-profile
build/calendar-profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --features calendar-oracle,layout-cache,allocation-counts --example calendar_bench -- --allocations
```
