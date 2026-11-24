# Färglägg en råbuffert via rektangelns koordinater

Provet uppdaterar förgrundsfärgen i samtliga 96 × 24 celler vid origo (7, 11). Varje bild förskjuter ett diagonalt mönster med fyra förberedda färger. Tidtagningen omfattar traversering, koordinatuppslag och färgbyte i den återanvända bufferten. Fyra separata bildutskrifter kontrollerar alla 9 216 celler mot Ratatui 0.30.2. Tidsprovet skickar noll terminalbyte. [Dunder](rect_paint.ddr) · [Ratatui](reference/examples/rect_paint.rs)

## Tid per bild

Tabellen visar median / p95 i mikrosekunder. Varje rad bygger på sju processer per anropssätt efter två uppvärmningsomgångar. De fem programvarianterna roterade i körordning. Varje process ritade tio uppvärmningsbilder och 100 tidsmätta bilder. [Alla 105 processvärden](results/rect-paint-0.12.1.tsv)

| Plattform | Dunder `for` | Dunder `advance()` | Dunder två loopar | Ratatui `positions()` | Ratatui två loopar |
| --- | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 63 / 160 | 49 / 74 | 44 / 110 | 4 / 4 | 3,2 / 3,3 |
| Linux ARM64 | 72,6 / 164,5 | 56,3 / 122,6 | 50 / 97 | 4 / 4,4 | 3,4 / 3,4 |
| Översatt Linux x64 | 609,9 / 789,8 | 417,8 / 526,8 | 393,3 / 582,7 | 9,3 / 12,2 | 7,3 / 18,5 |

På Macen låg de sju processmedianerna inom 58–102 µs för Dunders `for`, 44–49 µs för `advance()` och 3,6–9,9 µs för Ratatuis `positions()`. På Linux ARM64 låg de inom 70,8–158,5, 55,4–58,6 respektive 4–10,1 µs. Macen hade Apple M5 och macOS 27.2.1. Linux ARM64 kördes i en lokal VM och x64 med Rosetta i samma VM. Nativa x64-tider återstår. Byggena använde Dunder 0.12.1 och Rust 1.99.1 i releaseprofil.

## Allokeringar och minne

Efter avdrag för uppstart, palett och buffert begärde Dunders `for` 506 880 allokeringar för 110 bilder. `advance()`, de nästlade Dunderlooparna och båda Rustvarianterna begärde noll ytterligare allokeringar. Dunders räknare omfattar också återanvända block. Alla varianter gav kontrollsumman 495.

Medianen av fem mätningar av högsta RSS var 1 840 / 2 000 KiB på macOS, 256 / 1 548 KiB på Linux ARM64 och 5 328 / 4 968 KiB på översatt Linux x64 för de två `for`-looparna, med Dunder först. [Profiler och enskilda RSS-värden](results/rect-paint-resources-0.12.1.tsv)

## Kör lokalt

```sh
mkdir -p build
dunder build --locked --offline benchmarks/rect_paint.ddr -o build/paint
dunder build --locked --offline --runtime-profile benchmarks/rect_paint.ddr -o build/profile
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak
cargo build --release --locked --offline --no-default-features --features theme-oracle \
    --manifest-path benchmarks/reference/Cargo.toml --example rect_paint --target-dir build/reference
ref="$PWD/build/reference/release/examples/rect_paint"
"$ref" for --cells >build/expected
for mode in for cursor nested; do
    build/paint "$mode" --cells >build/actual
    cmp build/expected build/actual || exit 1
done
for n in 1 2 3 4 5 6 7 8 9; do
    for k in 0 1 2 3 4; do
        case $(((n+k)%5)) in
            0) build/paint for ;;
            1) "$ref" nested ;;
            2) build/paint cursor ;;
            3) "$ref" for ;;
            4) build/paint nested ;;
        esac
    done
done
for mode in for cursor nested; do
    build/profile "$mode" --setup
    build/profile "$mode" --allocations
    for n in 1 2 3 4 5; do build/peak "$PWD/build/paint" "$mode" --allocations; done
done
for mode in for nested; do
    for n in 1 2 3 4 5; do build/peak "$ref" "$mode" --allocations; done
    for phase in --setup --allocations; do
        cargo run --release --locked --offline --no-default-features --features theme-oracle,allocation-counts \
            --manifest-path benchmarks/reference/Cargo.toml --example rect_paint -- "$mode" "$phase"
    done
done
```
