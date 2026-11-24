# Tolv frekvensband i en terminal

Mixern delar upp terminalen i tolv färgade band med `Length(8)`, `SpaceAround` och två cellers avstånd. Vid 80 kolumner konkurrerar banden och mellanrummen om samma plats. Dunder och Ratatui 0.30.2 gav samma 1 100 bilder på 80 × 24 och 96 × 24 celler. [Dunderprogram](bands.ddr) · [Rustprogram](reference/examples/bands.rs)

## Dela upp en ny bredd

Bredden växlar från 80 till 111 kolumner med en enda plats i layoutcachen. Varje delning omfattar alla tolv panelerna och tretton mellanrummen. Kontrollsumman för de 120 delningarna är 170 656 för alla tre programmen. Tabellen visar median / p95 i **mikrosekunder per delning**.

| Plattform | Dunder före ändringen | Dunder med direkt beräkning | Ratatui |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 5 655 / 6 695 | 0,50 / 1,00 | 3 326 / 4 313 |
| Linux ARM64 | 7 200 / 9 188 | 0,54 / 1,01 | 4 139 / 4 999 |
| Översatt Linux x86-64 | 52 425 / 67 227 | 8,85 / 10,04 | 6 313 / 8 986 |

Varje process körde fem uppvärmningsgrupper och 25 uppmätta grupper om fyra delningar. Programmen växlade startordning över sju processer var. Tabellen använder medianen av processernas p50 och p95. Gruppstorleken ger steg om 0,25 µs på den provade Macens mikrosekundklocka. [De 63 processresultaten](results/bands-dunder-0.12.0-cold.tsv) · [Biblioteket före ändringen](https://github.com/Mik-pe/dunder-tui/tree/b9fc9f4eb69ad098c44ff922afdf9bf9fb0b638c)

## Hela bilder med terminalutdata

Mixern använder bibliotekens vanliga layoutcache under interaktiv ritning. Vid skrollning flyttas bandens nivåer ett steg, vid storleksändring växlar bildens bredd och vid full omritning byter bakgrunden färg. Tiderna omfattar rendering, celldiff och skrivning till en pseudoterminal. Varje cell visar **Dunder · Ratatui**, med median/p95 i µs för respektive program.

| Arbete | macOS ARM64 | Linux ARM64 | Översatt Linux x86-64 |
| --- | ---: | ---: | ---: |
| Oförändrad bild | 5/9 · 21/39 | 9/10 · 29/47 | 148/188 · 55/82 |
| En cell | 7/8 · 21/39 | 10/24 · 27/47 | 159/203 · 57/87 |
| Skrollning | 45/95 · 81/118 | 37/74 · 54/94 | 594/728 · 129/183 |
| Storleksändring | 49/76 · 72/103 | 26/58 · 44/76 | 472/646 · 102/151 |
| Full omritning | 69/110 · 118/159 | 42/84 · 73/121 | 744/875 · 199/266 |

Dunder 0.12.0 och Rust 1.99.1 kördes lokalt på en Apple M5 med macOS 27.2.1 och Linux 7.0.14-linuxkit i Docker Desktop. x86-64 kördes med processoröversättning. Två uppvärmningspar följdes av sju uppmätta processpar. Varje arbetslast hade 20 uppvärmningsbilder och 200 tidsmätta bilder. Rådata innehåller också spridningen mellan processerna, tiden i minnet och byte per bild. [Bildtider och spridning](results/bands-dunder-0.12.0-local.tsv)

Hela terminalströmmen var 2 328 959 byte med Dunder och 3 354 661 byte med Ratatui på alla tre plattformar. Minnestalen nedan är medianen av fem processers högsta RSS i KiB. Allokeringsräknarna kommer från separata körningar av hela arbetslasten.

| Plattform | Högsta RSS, Dunder / Ratatui | Allokeringsanrop, Dunder / Ratatui | Begärda byte, Dunder / Ratatui |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 1 792 / 3 344 | 452 / 5 855 | 353 378 / 3 065 335 |
| Linux ARM64 | 176 / 2 188 | 452 / 5 818 | 353 378 / 2 979 294 |
| Översatt Linux x86-64 | 5 712 / 5 616 | 452 / 5 713 | 353 378 / 2 714 710 |

Samtliga enskilda minnesmätningar och byteantal finns i [resursmätningarna](results/bands-dunder-0.12.0-resources.tsv).

## Kör lokalt

Cargo använder de låsta referenspaketen från sin lokala cache. Dunderprogrammet byggs offline från bibliotekskoden.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/bands.ddr -o build/bands
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml \
  --features layout-cache --example bands --target-dir build/reference
ref="$PWD/build/reference/release/examples/bands"
build/bands --check >build/band-cells
"$ref" --check >build/reference-cells
cmp build/band-cells build/reference-cells
build/bands --cold
"$ref" --cold
build/measure "$PWD/build/bands" "$ref" "$PWD/build/pty"
dunder build --locked --offline --runtime-profile benchmarks/bands.ddr -o build/bands-profile
build/bands-profile --allocations
cargo run --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml \
  --features layout-cache,allocation-counts --example bands -- --allocations
```

Bygg jämförelseversionen från `b9fc9f4` genom att lägga samma `benchmarks/bands.ddr` i den utcheckningen och köra det första byggkommandot där. För högsta RSS används `/usr/bin/time -l` på macOS och GNU `time -v` på Linux med argumentet `--allocations` till vardera ordinarie programmet.
