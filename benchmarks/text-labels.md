# Textetiketter som värden

Varje omgång behandlar 128 etiketter. Korta etiketter har ett enda spann. Blandade dokument innehåller två rader med färg, CRLF, sammansatt emoji och östasiatiska tecken. Maskprovet byter ut varje Unicode-skalär mot `●`. Vid uppslag använder programmet ett separat konstruerat dokument som nyckel för varje etikett. [Formatering](text_labels.ddr) · [Uppslag](text_keys.ddr) · [Ratatui 0.30.2](reference/examples/text_labels.rs)

Tabellen visar median/p95 i mikrosekunder per omgång på macOS ARM64 med Dunder 0.12.1.

| Arbete | Dunder före | Dunder nu | Ratatui |
| --- | ---: | ---: | ---: |
| Formatera kort text | 10 / 10 | 4 / 4 | 3,29 / 3,38 |
| Formatera blandade dokument | 34 / 36 | 21 / 25 | 16,96 / 17,21 |
| Maskera text | 20 / 20 | 8 / 9 | 11,71 / 12,13 |
| Slå upp dokument | – | 26 / 28 | 17,79 / 20,33 |

Varje process värmde upp med fem omgångar och tog tid på 60. Processordningen roterades nio gånger på samma värd. Tabellen visar medianen från de sista sju körningarna. Rådata innehåller variationen mellan körningarna, kontrollsummor och profilbyggenas allokeringsanrop. För korta etiketter sjönk Dunders anrop från 68 252 till 1 692 inklusive uppbyggnad. [Alla mätningar för macOS och Linux](results/text-labels-0.12.1.tsv)

I små Linuxprocesser kunde `getrusage` rapportera noll för högsta RSS. Kolumnen `retained_rss_kib` visar i stället hur mycket resident minne processen hade direkt efter mätningen, medan etiketterna fortfarande fanns kvar. Värdet är medianen av tre processer, läst med `ps -o rss` på macOS och från `Rss` i `/proc/PID/smaps_rollup` på Linux. Linux x86-64 använde processoröversättning.

## Kör lokalt

```sh
mkdir -p build
dunder build --locked --offline benchmarks/text_labels.ddr -o build/labels
dunder build --locked --offline benchmarks/text_keys.ddr -o build/keys
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --target-dir build/reference --features theme-oracle --example text_labels
for mode in short mixed mask; do
    build/labels "$mode"
    build/reference/release/examples/text_labels "$mode"
done
build/keys
build/reference/release/examples/text_labels lookup
```

För att mäta föregående revision, exportera `73e4121` och kopiera samma `benchmarks/text_labels.ddr` till dess `benchmarks/`. Bygg med samma kompilator. För allokeringsprofilen, ange `--runtime-profile` vid Dunderbygget och `--features theme-oracle,allocation-counts` vid Rustbygget. För resident minne, lägg till `--resident` som sista programargument. Processen skriver sin mätning och inväntar en rad eller EOF på standardinmatningen medan etiketterna finns kvar.

Cell- och uppslagsprovet kan köras separat med samma låsta Rustpaket i Cargos lokala cache.

```sh
dunder build --locked --offline tests/text_values.ddr -o build/values
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --target-dir build/reference --features theme-oracle --example text_values
build/values >build/dunder-values
build/reference/release/examples/text_values >build/ratatui-values
cmp build/dunder-values build/ratatui-values
```
