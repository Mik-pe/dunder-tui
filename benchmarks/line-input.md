# Bygg en buffert från textrader

`Buffer::with_lines` får 24 redan skapade strängar. Varje rad består av åtta omväxlande textstycken. ASCII- och Unicode-bilderna är 96 × 24 celler. Den tredje bilden är 64 × 24 och innehåller arabiska ligaturer, japanska ljudmärken, emoji, tabb, LF och CRLF. Tiden omfattar breddmätning, allokering, fyllning av hela bufferten och läsning av måtten och en cell. Tidtagningen slutar före frigörandet. Konstruktorn skriver noll terminalbyte. [Dunder](line_input.ddr) · [Ratatui 0.30.2](reference/examples/line_input.rs)

## Mätningar

Tabellen visar median / p95 i mikrosekunder per färdig buffert. För varje värde tas medianen av sju processers p50 respektive p95. [Alla 189 mätningar](results/line-input-0.12.1.tsv)

| Plattform | Innehåll | Dunder före | Dunder nu | Ratatui |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | ASCII | 62 / 133 | 57 / 126 | 35,8 / 61 |
| macOS ARM64 | CJK och diakritiska tecken | 201 / 407 | 182 / 308 | 39,3 / 59,1 |
| macOS ARM64 | Blandat med radslut | 234 / 332 | 196 / 324 | 35,2 / 49,1 |
| Linux ARM64 | ASCII | 65,7 / 122,7 | 61,1 / 140,5 | 45,3 / 110,4 |
| Linux ARM64 | CJK och diakritiska tecken | 207,5 / 446,8 | 190,3 / 415 | 50,6 / 75,5 |
| Linux ARM64 | Blandat med radslut | 241,9 / 490,6 | 200,8 / 376,2 | 40 / 74,3 |
| Översatt Linux x64 | ASCII | 943,4 / 1525,1 | 857 / 1329,5 | 65,6 / 124,1 |
| Översatt Linux x64 | CJK och diakritiska tecken | 2261,8 / 3362,6 | 1964,8 / 3042,4 | 82,4 / 115 |
| Översatt Linux x64 | Blandat med radslut | 2456,8 / 3574 | 1896,4 / 2674,2 | 64,8 / 104,4 |

För texten med radslut låg de sju processmedianerna på Macen inom 233–241 µs före ändringen, 196–202 µs efter ändringen och 34,8–36,5 µs för Ratatui. Nativa tider för Windows x64 och Linux x64 återstår.

Över 330 byggen begärde Dunder 3 363 975 allokeringar jämfört med 3 676 815 tidigare, inklusive 615 för uppstart och indata. Dunders räknare inkluderar återanvändning ur minnespoolen. När uppstarten drogs av begärde Rust 13 860 allokeringar. Medianen för högsta RSS över fem oprofilerade processer var 2 080 / 2 352 KiB på macOS, 384 / 1 724 KiB på Linux ARM64 och 5 700 / 4 948 KiB på översatt Linux x64, med Dunder först. [Alla resursvärden](results/line-input-resources-0.12.1.tsv)

## Återskapa körningen lokalt

Dunder 0.12.1 och Rust 1.99.1 användes på en Apple M5 med macOS 27.2.1. Linux ARM64 kördes i en lokal VM och Linux x64 med Rosetta. Varje plattform fick två uppvärmningsomgångar och sju mätomgångar. Körordningen mellan de tre programmen roterade. Varje process byggde tio uppvärmningsbilder och 100 uppmätta bilder per arbetslast. Cargo behöver den separat låsta Rustreferensen i sin lokala cache.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/line_input.ddr -o build/lines
cargo build --release --locked --offline --no-default-features --features theme-oracle \
    --manifest-path benchmarks/reference/Cargo.toml --example line_input --example buffer_lines --target-dir build/reference
ref="$PWD/build/reference/release/examples/line_input"
build/lines --cells > build/dunder-cells
"$ref" --cells > build/rust-cells
cmp build/dunder-cells build/rust-cells

baseline=$(mktemp -d)
git archive 42c65d8fcaa4edb6cdc1ba86fbe08da81182e185 | tar -x -C "$baseline"
cp benchmarks/line_input.ddr "$baseline/benchmarks/line_input.ddr"
before="$PWD/build/lines-before"
(cd "$baseline" && dunder build --locked --offline benchmarks/line_input.ddr -o "$before")
for run in 1 2 3 4 5 6 7 8 9; do
    case $((run % 3)) in
        0) set -- "$before" build/lines "$ref" ;;
        1) set -- build/lines "$ref" "$before" ;;
        2) set -- "$ref" "$before" build/lines ;;
    esac
    for exe do "$exe"; done
done

dunder build --locked --offline --runtime-profile benchmarks/line_input.ddr -o build/lines-profile
for arg in --setup --allocations; do
    build/lines-profile "$arg"
    cargo run --release --locked --offline --no-default-features --features theme-oracle,allocation-counts \
        --manifest-path benchmarks/reference/Cargo.toml --example line_input -- "$arg"
done
dunder build --locked --offline benchmarks/peak_rss.ddr -o build/peak
build/peak "$PWD/build/lines" --allocations
build/peak "$ref" --allocations
```

## Jämför celler och koordinater

Referensen använder både `Buffer::with_lines` och `TestBackend::with_lines`. Varje utskriven följd innehåller antalet identiska fysiska celler och cellens hela JSON-värde. Testet läser även skärmen genom testterminalens radassertion. I gränsprovet ingår 65 535, 65 536, 65 537 och 131 075 rader samt text där ett brett tecken träffar 16-bitarsgränsen. [Testprogram](../tests/buffer_lines.ddr) · [Ratatui-prov](reference/examples/buffer_lines.rs)

```sh
dunder build --locked --offline tests/buffer_lines.ddr -o build/check-lines
dunder build --locked --offline tests/cell_edits.ddr -o build/cases
build/cases --graphemes >build/graphemes.txt
build/cases --combining-cases >build/marks.txt
oracle=build/reference/release/examples/buffer_lines
for group in short bounds unicode; do
    case "$group" in
        short) set -- ;;
        bounds) set -- --bounds ;;
        unicode) set -- --unicode tests/data/emoji-18.txt tests/data/emoji-variation-18.txt build/graphemes.txt build/marks.txt ;;
    esac
    build/check-lines "$@" >build/actual
    "$oracle" "$@" >build/expected
    cmp build/actual build/expected || exit 1
done
for target in buffer backend; do
    if build/check-lines --outside "$target"; then exit 1; fi
    if "$oracle" --outside "$target"; then exit 1; fi
done
```
