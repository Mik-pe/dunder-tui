# Rendering mot Ratatui 0.30.2

Båda programmen ritar samma 120 × 40 ASCII-celler, jämför två återanvända bildbuffertar och kodar ändringen till en återanvänd bytebuffert. Vid storleksändring växlar bredden mellan 100 och 120 och hela ytan skickas. Rustprogrammet använder `Buffer::set_stringn`, `Buffer::diff_iter` och `CrosstermBackend::draw`. Rustberoendena hör till det fristående referensprogrammet. [Dunderprogram](render.ddr) · [Rustprogram](reference/src/main.rs)

## Resultat

Apple M5, macOS 27.2.1, Dunder 0.11.0 med standardoptimeringar och Rust 1.99.1 i releaseprofil. Varje fall kör 20 uppvärmningsbilder och 200 mätbilder. Två hela körningar per program värmde upp maskinen. Tabellen visar medianen av sju körningars p50 och p95, med växlande körordning. Klockans upplösning i Dunder var cirka 1 µs.

| Ändring | Dunder p50 / p95, µs | Ratatui p50 / p95, µs | Dunder byte/bild | Ratatui byte/bild |
| --- | ---: | ---: | ---: | ---: |
| Oförändrad bild | 18 / 18 | 79 / 83 | 0 | 13 |
| Ett tecken | 18 / 20 | 78 / 83 | 9 | 22 |
| Rullande textmönster | 30 / 32 | 95 / 99 | 2 028 | 2 041 |
| Ny bredd | 44 / 50 | 86 / 93 | 4 671 | 4 684 |
| Alla tecken ändras | 51 / 51 | 118 / 127 | 5 071 | 5 084 |

För encellsfallet låg de sju körningarnas medianer inom 18–21 µs för Dunder och 76–85 µs för Ratatui. Mätningen slutar när ANSI-strömmen finns i minnet. Teckenritning i terminalemulatorn, färgrik grafik och tangentbordslatens behöver egna prov.

Över alla fem fall med avstängd mätklocka rapporterade Dunders runtime 143 allokeringsanrop och 708 670 begärda byte. 111 anrop återanvände block från poolen. Rustprogrammets globala allokerare registrerade 1 175 anrop och 2 585 196 begärda byte inklusive omallokering. I en separat körning med `/usr/bin/time -l` var processens högsta RSS 1 818 624 respektive 2 834 432 byte.

## Kör själv

Kör i repots rot. Varje rad i tidsmätningen ger arbetslast, p50 och p95 i nanosekunder och genomsnittligt antal kodade byte per bild.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/render.ddr -o build/render
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust
build/render
build/rust/release/ratatui-render-reference
```

Allokeringsprovet kör samma bilder. Det kan köras separat från tidsmätningen.

```sh
dunder build --runtime-profile benchmarks/render.ddr -o build/render-profile
build/render-profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features allocation-counts -- --allocations
```
