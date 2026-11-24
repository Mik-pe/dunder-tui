# Avgångstavla med 128 redigerbara rader

Avgångstavlan byggs från en rad med fyra spann. Varje kopia får sitt eget avgångsnummer. En separat tabell visar fyra gröna kajstatusar. Dunder och Ratatui 0.30.2 ger samma 1 100 bilder när rubriken ändras, tavlan skrollar, bredden växlar mellan 80 och 96 kolumner och bakgrunden ritas om. Höjden är 24 rader. [Dunders scen](manifest_scene.ddr) · [Ratatuis scen](reference/examples/manifest.rs)

## Bildtid i terminalen

Tabellen visar median / p95 i mikrosekunder per bild. Varje värd körde båda programmen i växlande ordning. Två uppvärmningspar följdes av sju uppmätta par. För varje arbetslast i en process följdes 20 uppvärmningsbilder av 200 mätta bilder med rendering, celldiff och skrivning till en pseudoterminal. De redovisade tiderna är medianen av de sju processernas p50 respektive p95. Rådata innehåller också deras lägsta och högsta värden, byte per bild och motsvarande tider med utmatning i minnet. [Alla mätvärden](results/manifest-dunder-0.12.0-local.tsv) · [Mätprogram](measure.ddr)

| Arbete | macOS ARM64, Dunder · Ratatui | Linux ARM64, Dunder · Ratatui | Översatt Linux x86-64, Dunder · Ratatui |
| --- | ---: | ---: | ---: |
| Oförändrad bild | 13/15 · 35/55 | 14/14 · 35/44 | 163/194 · 64/100 |
| En cell | 16/25 · 34/40 | 18/21 · 35/45 | 170/216 · 63/82 |
| Skrollning | 24/35 · 35/46 | 25/28 · 36/44 | 197/227 · 65/81 |
| Storleksändring | 38/48 · 55/72 | 26/30 · 47/54 | 309/362 · 84/109 |
| Full omritning | 54/71 · 86/120 | 40/47 · 69/78 | 428/491 · 124/175 |

Dunder 0.12.0 och Rust 1.99.1 kördes lokalt på samma Apple M5. macOS var 27.2.1. Linux kördes i Docker Desktop med kärnan 7.0.14-linuxkit. Kolumnen för x86-64 använder processoröversättning. Nativa x86-64-tider för denna tavla återstår på både Linux och Windows.

När en klippt rad ryms i fönstret kan Dunder använda redan beräknade bredder för hela spann. I en separat parvis mätning sjönk skrollningens p50/p95 från 31/35 till 23/26 µs på Macen, från 34/41 till 26/32 µs på Linux ARM64 och från 251/291 till 197/234 µs på översatt Linux x86-64. Båda versionerna skrev exakt 1 399 408 byte per hel körning. [Föregående version](https://github.com/Mik-pe/dunder-tui/tree/6044f41affd9516162c4a318db08b21993509902) · [Parvisa resultat](results/manifest-dunder-0.12.0-local.tsv)

## Minne och terminalutdata

Allokeringarna omfattar uppstart, inläsning av tavlan och alla 1 100 bilder i en separat körning. Minnet är medianen av fem processers högsta RSS i KiB. Terminalutdata räknas vid läsning från pseudoterminalen. [Samtliga minnesprov och uppmätta byte](results/manifest-dunder-0.12.0-resources.tsv)

| Värd | Högsta RSS i KiB, Dunder / Ratatui | Allokeringsanrop, Dunder / Ratatui | Begärda byte, Dunder / Ratatui |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 2 192 / 3 008 | 50 386 / 39 974 | 6 758 009 / 4 563 116 |
| Linux ARM64 | 640 / 2 224 | 50 386 / 40 007 | 6 758 009 / 4 584 619 |
| Översatt Linux x86-64 | 6 368 / 5 756 | 50 386 / 39 926 | 6 758 009 / 4 548 251 |

Dunder skrev 1 399 408 byte och Ratatui 1 761 490 byte på alla tre värdar. Win64-programmet under Wine gav samma 1 100 bilder. Ett separat Win32-prov läste tillbaka avgångsnummer och kajstatus ur konsolbufferten efter skrollning. [POSIX-prov](../tests/constructor_terminal.ddr) · [Win32-prov](../tests/windows_console.ddr)

## Upprepa mätningen lokalt

Dunder använder standardbiblioteket och operativsystemets API. Den fristående Rust-referensen behöver sina låsta paket i Cargos lokala cache för offlinebygget.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/manifest.ddr -o build/manifest
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features macro-oracle,layout-cache --example manifest --target-dir build/reference
ref="$PWD/build/reference/release/examples/manifest"
build/manifest --check >build/manifest-dunder
"$ref" --check >build/manifest-rust
cmp build/manifest-dunder build/manifest-rust
build/measure "$PWD/build/manifest" "$ref" "$PWD/build/pty"
dunder build --locked --offline --runtime-profile benchmarks/manifest.ddr -o build/manifest-profile
build/manifest-profile --allocations
cargo run --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features macro-oracle,layout-cache,allocation-counts --example manifest -- --allocations
```

För högsta RSS, kör båda ordinarie programmen fem gånger var med argumentet `--allocations` och läs processens högsta minne med `/usr/bin/time -l` på macOS eller GNU `time -v` på Linux.
