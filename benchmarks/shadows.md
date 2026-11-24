# Skuggade paneler

Tre paneler läggs över varandra med färgfilter, skuggtecken och en skugga som halverar RGB-bakgrunden. I skrollprovet flyttas panelerna i sidled. Ytan växlar mellan 96 × 24 och 80 × 24 i storleksprovet. Alla 1 100 bilder kontrolleras mot Ratatui 0.30.2 innan tidtagningen. [Dunder](shadows.ddr) · [Rust](reference/src/shadow_scene.rs)

Tabellen visar median / p95 i mikrosekunder inklusive skrivning till pseudoterminalen på en Apple M5. Varje värde är medianen från sju processpar efter två uppvärmningspar. Startordningen växlar och varje arbetslast har 200 uppmätta bilder per process. ARM64-kolumnerna för Linux avser en lokal VM. Linux x86-64 kördes med översättning i samma VM. [Rådata med spridning per mått, byte per bild och tider utan terminal](results/shadows-dunder-0.11.3-local.tsv)

| Arbete | macOS ARM64 Dunder | Ratatui | Linux ARM64 Dunder | Ratatui | Linux x86-64 översatt Dunder | Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad bild | 14 / 16 | 41 / 45 | 16 / 18 | 66 / 70 | 104 / 114 | 79 / 87 |
| En cell | 15 / 16 | 41 / 45 | 16 / 18 | 65 / 70 | 105 / 118 | 80 / 97 |
| Skrollning | 43 / 47 | 68 / 73 | 35 / 37 | 74 / 80 | 213 / 229 | 94 / 107 |
| Storleksändring | 53 / 60 | 87 / 96 | 40 / 46 | 89 / 99 | 401 / 442 | 134 / 161 |
| Full omritning | 54 / 59 | 86 / 93 | 42 / 44 | 89 / 97 | 337 / 367 | 130 / 153 |

Samtliga tre pseudoterminaler tog emot 2 099 431 byte från Dunder och 2 732 978 byte från Ratatui. På macOS gjorde Dunder 12 079 allokeringsanrop för 570 137 begärda byte. Rustprogrammet gjorde 17 071 för 1 274 921 byte. Högsta residenta minne var 1 792 respektive 2 432 KiB, medianer av tre separata processer. Nativ Linux x86-64 och Windows x64 återstår för tidtagningen av denna scen. Win64-provet läste tillbaka båda skuggorna från konsolen under Wine. [Win32-prov](../tests/windows_console.ddr)

## Kör lokalt

Använd Dunder 0.11.3. Rustreferensens låsta paket behöver finnas i den lokala Cargo-cachen. Dunderprogrammen byggs direkt från repot.

```sh
mkdir -p build
export TUI_BENCH_SCENE=shadows
dunder build --locked --offline tests/shadow_cells.ddr -o build/shadow-cells
dunder build --locked --offline benchmarks/shadows.ddr -o build/shadows
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features style-oracle --example shadow_cells
build/shadow-cells | build/rust/release/examples/shadow_cells
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features layout-cache --example terminal_bench
ref=build/rust/release/examples/terminal_bench
build/shadows --verify | "$ref" --verify
build/measure build/shadows "$ref" build/pty
dunder run --locked --offline tests/shadow_terminal.ddr
```

Mät RSS på macOS med `/usr/bin/time -l` på vardera vanliga programmet med `--allocations`. För allokeringsräknare, bygg sedan Dunderprogrammet med `--runtime-profile` och Rustprogrammet med `--features layout-cache,allocation-counts`. Kör båda med `--allocations`.
