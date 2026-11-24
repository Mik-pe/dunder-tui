# Bygg en buffert från textrader

`Buffer::with_lines` får 24 redan skapade strängar. Varje rad består av åtta omväxlande textstycken. ASCII- och Unicode-bilderna är 96 × 24 celler. Den tredje bilden är 64 × 24 och innehåller arabiska ligaturer, japanska ljudmärken, emoji, tabb, LF och CRLF. Tiden omfattar breddmätning, allokering, fyllning av hela bufferten och läsning av måtten och en cell. Tidtagningen slutar före frigörandet. Konstruktorn skriver noll terminalbyte. [Dunder](line_input.ddr) · [Ratatui 0.30.2](reference/examples/line_input.rs)

## Mätningar

Råbufferten skapar nu tomma celler direkt när den växer. Vid skrivning av blandad text behandlas sammanhängande utskrivbara ASCII-tecken tillsammans. Sista tecknet före en Unicode-skalär går genom grafemläsaren så att exempelvis en siffra och dess presentationsväljare hamnar i samma cell.

Tabellen visar median / p95 i mikrosekunder per färdig buffert, jämfört med `d772f40` på samma maskin. Varje värde är medianen av sju processers p50 respektive p95. [Alla 189 mätningar](results/line-storage-0.12.1.tsv) · [Föregående jämförelse](results/line-input-0.12.1.tsv)

| Plattform | Innehåll | Dunder före | Dunder nu | Ratatui |
| --- | --- | ---: | ---: | ---: |
| macOS ARM64 | ASCII | 58 / 130 | 50 / 104 | 35,9 / 104,7 |
| macOS ARM64 | CJK och diakritiska tecken | 188 / 443 | 138 / 304 | 42 / 112,6 |
| macOS ARM64 | Blandat med radslut | 204 / 460 | 194 / 419 | 35,4 / 82,1 |
| Linux ARM64 | ASCII | 59,3 / 124,3 | 48,3 / 70,6 | 45,5 / 58,8 |
| Linux ARM64 | CJK och diakritiska tecken | 187,9 / 411,4 | 136,6 / 270,7 | 50 / 81,9 |
| Linux ARM64 | Blandat med radslut | 199,8 / 283,5 | 182,8 / 349,3 | 39,5 / 54 |
| Översatt Linux x64 | ASCII | 870,4 / 1 336,1 | 659,6 / 993,1 | 65 / 109,2 |
| Översatt Linux x64 | CJK och diakritiska tecken | 1 958 / 2 996 | 1 397,7 / 2 025,7 | 81,8 / 147,1 |
| Översatt Linux x64 | Blandat med radslut | 1 909,6 / 2 793,3 | 1 696,3 / 2 538,9 | 64,9 / 89,7 |

På Linux ARM64 låg de sju processmedianerna för CJK-provet inom 183–190 µs före ändringen, 135,3–138,7 µs efteråt och 49,5–51 µs i Ratatui. För texten med radslut sjönk medianen på samma plattform medan p95 ökade från 283,5 till 349,3 µs. På macOS varierade CJK-medianerna mellan 132 och 288 µs efter ändringen. Nativa tider för Windows x64 och Linux x64 återstår.

Över 330 buffertbyggen sjönk Dunders allokeringsanrop från 3 363 975 till 2 687 145 inklusive 615 för uppstart och indata. Dunders räknare omfattar återanvändning ur minnespoolen. Efter avdrag för uppstart gjorde Rust 13 860 anrop till sin globala allokerare. Medianen för högsta RSS över fem oprofilerade processer var 2 080 / 2 352 KiB på macOS, 384 / 1 724 KiB på Linux ARM64 och 5 704 / 5 012 KiB på översatt Linux x64, med Dunder först. [Alla resursvärden](results/line-storage-resources-0.12.1.tsv)

Vid omskrivning av en befintlig buffert på macOS låg ASCII kvar på 21 µs i median. När text och stil ändrades i varje bild sjönk medianen från 109 till 106 µs, med p95 på 168 µs i båda körningarna. Ratatui tog 36,8 / 70,5 µs i det rörliga provet. [Alla 105 mätningar](results/line-storage-writes-macos-0.12.1.tsv) · [Arbetslaster och körkommandon](cell-writes.md)

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
git archive d772f4072c959adc2ea2677e1ed89eb27259d695 | tar -x -C "$baseline"
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
