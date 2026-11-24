# Skriv direkt till en rå cellbuffert

Båda biblioteken uppdaterar en buffert med 96 × 24 celler. ASCII- och Unicode-proven behåller texten och växlar stil på varje bild. Spannprovet återanvänder ett stiliserat Unicode-spann. Stilprovet ändrar enbart cellernas stil. Det rörliga provet växlar mellan fyra rader med CJK, kombinerande tecken och emoji. Varje rad får ny text och stil för varje bild. Ett separat prov jämför alla 9 216 celler i rörelsens fyra bildrutor. De fem tidsproven skriver noll byte till terminalen. [Dunder](cell_write.ddr) · [Ratatui](reference/examples/cell_edits.rs)

## Bildtid

Varje värde anger p50 / p95 i mikrosekunder per bild. Tre program kördes i roterande ordning på varje mål. Dunder före och efter ändringen använde samma mätprogram. Efter två uppvärmningsomgångar kördes sju mätomgångar. Varje process ritade tio uppvärmningsbilder och mätte sedan 100 bilder per arbetslast. Tabellen visar medianen av de sju processernas p50 respektive p95. [Alla mätvärden](results/unicode-writes-0.12.1.tsv)

| Arbetslast | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Översatt Linux x64 Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| ASCII | 20 / 36 | 31,5 / 50,5 | 21,8 / 28,8 | 33,8 / 68,5 | 313 / 488 | 53,6 / 123 |
| Unicode | 91 / 125 | 36,5 / 45,7 | 94,2 / 181 | 38,3 / 87,8 | 929 / 1 225 | 64,3 / 111 |
| Förberett spann | 45 / 102 | 35,0 / 78,3 | 48,0 / 67,1 | 34,9 / 54,7 | 504 / 802 | 60,7 / 105 |
| Stil | 12 / 13 | 3,4 / 3,6 | 13,5 / 17,2 | 3,6 / 3,6 | 175 / 226 | 8,7 / 9,0 |
| Rörlig Unicode | 107 / 207 | 36,5 / 53,6 | 111 / 230 | 37,6 / 59,7 | 1 114 / 1 477 | 63,8 / 99,3 |

För fast Unicode-text på Macen gick Dunders processmedianer från 206–216 till 89–95 µs. För rörlig Unicode gick de från 227–237 till 106–110 µs. Ratatuis intervall för rörlig Unicode var 34,4–37,6 µs. En skrivning läser nu varje skalär en gång och sparar nästa grafems första skalär. Emoji och skriftspecifika kombinationer som behöver en kontextuell breddberäkning använder fortfarande standardbibliotekets terminalbredd.

Mätningarna använde Apple M5, macOS 27.2.1, Linux 7.0.14 i en lokal ARM64-VM, Dunder 0.12.1 och Rust 1.99.1 i releaseprofil. Linux x86-64 kördes med Rosetta i samma VM. Nativa bildtider på Linux x64 och Windows x64 återstår för denna arbetslast.

## Allokeringar och minne

För uppbyggnad och 550 bilduppdateringar sjönk Dunders allokeringsanrop från 903 113 till 396 233 och begärda byte från 17 806 682 till 6 606 074. Räknaren omfattar även återanvändning i Dunders minnespool. Rusts globala allokerare fick 20 anrop på macOS och 18 på Linux. Medianen för högsta RSS över fem oprofilerade körningar var 2 064 / 2 160 KiB på Macen, 384 / 1 920 KiB på Linux ARM64 och 5 640 / 5 452 KiB på översatt Linux x64, med Dunder först. [Alla räknare och RSS-prov](results/unicode-write-resources-0.12.1.tsv)

## Kör lokalt

```sh
mkdir -p build
dunder build --locked --offline benchmarks/cell_write.ddr -o build/write
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features theme-oracle --example cell_edits --target-dir build/reference
ref="$PWD/build/reference/release/examples/cell_edits"
build/write --check-moving | "$ref" --check-moving

old=$(mktemp -d)
git archive aa3c7ec | tar -xf - -C "$old"
cp benchmarks/cell_write.ddr "$old/benchmarks/"
dunder build --locked --offline "$old/benchmarks/cell_write.ddr" -o build/before
for n in 1 2 3 4 5 6 7 8 9; do
    case $((n % 3)) in
        0) build/before && build/write && "$ref" --bench ;;
        1) build/write && "$ref" --bench && build/before ;;
        2) "$ref" --bench && build/before && build/write ;;
    esac
done
build/peak "$PWD/build/write" --allocations
build/peak "$ref" --allocations
dunder build --locked --offline --runtime-profile benchmarks/cell_write.ddr -o build/write-profile
build/write-profile --allocations
cargo run --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features theme-oracle,allocation-counts --example cell_edits -- --allocations
```
