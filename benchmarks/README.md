# Rendering mot Ratatui 0.30.2

Båda programmen bygger om samma vy på 120 × 40 celler, jämför två återanvända bildbuffertar och kodar uppdateringen till en återanvänd bytebuffert. Vid storleksändring växlar bredden mellan 100 och 120 och hela ytan skickas. Rustprogrammet använder `Buffer::set_stringn`, `Buffer::diff_iter` och `CrosstermBackend::draw` med färgutdata påslagen. Rustberoendena hör till referensprogrammet. [Dunderprogram](render.ddr) · [Rustprogram](reference/src/main.rs)

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
