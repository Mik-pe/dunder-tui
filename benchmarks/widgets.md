# Diagramvy i 0.5.0

Vyn består av tre flikar, en färgsatt sparkline, tolv namngivna staplar, paneler och en mätare. Ritningen sker via båda bibliotekens publika API. Vyn är 96 × 24 celler och ändras till 80 × 24 varannan bild i storleksprovet. Ett prov ändrar ett enda tecken, ett rullar signalen ett sampel åt gången och ett växlar färg på hela bakgrunden. Alla 36 kombinationer av bredd, fas, tema och markör har jämförts cell för cell. [Dunder](widgets_scene.ddr) · [Ratatui](reference/src/widgets_scene.rs) · [Bildjämförelsen](../docs/parity.md#widgets-i-050)

Bildtiden innefattar tömning, ritning från förberedd widgetdata, diff och ANSI-kodning. I PTY-provet ingår också varje skrivning till en pseudoterminal i råläge. Läsaren räknar fysiska byte. Skärmemulatorns presentationstid ingår inte i mätningen. Programmen bygger med Dunder 0.11 och Rust 1.99.1 i releaseprofil. Efter två uppvärmningskörningar följer sju körningar med växlande startordning. Varje fall har 20 uppvärmningsbilder och 200 mätbilder.

## Med skrivning till terminal

Varje fält visar medianen av körningarnas p50 / p95 i mikrosekunder. macOS 27.2.1 och Linuxgästerna kördes på samma Apple M5. Linux ARM64 körde inbyggd ARM64-kod. Linux x86-64 körde x86-64-kod via CPU-översättning. [Samtliga resultat och spridning i nanosekunder](results/widgets-0.5.0.tsv)

| Ändring | macOS Dunder | macOS Ratatui | Linux ARM64 Dunder | Linux ARM64 Ratatui | Linux x86-64 Dunder | Linux x86-64 Ratatui | Byte/bild Dunder / Ratatui |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Oförändrad | 29 / 30 | 40 / 44 | 30 / 31 | 42 / 46 | 276 / 290 | 72 / 76 | 0 / 19 |
| Ett tecken | 34 / 38 | 40 / 45 | 36 / 40 | 42 / 46 | 283 / 296 | 72 / 85 | 21 / 44 |
| Skrollning | 68 / 73 | 78 / 84 | 52 / 56 | 58 / 64 | 426 / 445 | 107 / 114 | 3 123 / 5 948 |
| Ny bredd | 91 / 101 | 119 / 138 | 62 / 70 | 68 / 82 | 523 / 564 | 126 / 150 | 7 503 / 14 485 |
| Full omritning | 104 / 113 | 151 / 164 | 77 / 83 | 100 / 112 | 636 / 661 | 194 / 211 | 7 952 / 15 406 |

Alla tre plattformarna gav exakt 4 113 141 byte för Dunder och 7 939 285 byte för Ratatui över 1 100 bilder inklusive uppvärmning. Dunders median och p95 var lägre i de fem terminalfallen på ARM64. På den översatta x86-64-processorn var Ratatui snabbare i samtliga fall. Mätning på en fysisk x86-64-processor återstår.

När programmen slutade tidtagningen före terminalskrivningen tog ny bredd på macOS 56 / 61 µs för Dunder och 50 / 54 µs för Ratatui. Linux ARM64 gav 56 / 60 respektive 50 / 56 µs. Mellan de sju Mac-körningarna varierade p50 för det fallet mellan 55 och 61 µs för Dunder och mellan 49 och 51 µs för Ratatui. Övrig systemlast påverkade särskilt p95. Resultatfilen innehåller även en upprepad Mac-serie och en separat serie med samtidig kompilering.

## Allokeringar och minne

Allokeringsräknarna omfattar start, skapande av widgetdata och alla 1 100 bilder, med bildklockan avstängd. Dunders runtime räknar även återbruk av poolminne. Rust använder en global räknande allokerare. Högsta RSS kommer från separata körningar av samma program med tidtagning. Tabellen anger medianen för fem sådana körningar på macOS och tre på Linux. RSS för x86-64 omfattar översättningens kostnad i gästprocessen.

| Plattform | Anrop Dunder / Ratatui | Begärda byte Dunder / Ratatui | Högsta RSS i KiB Dunder / Ratatui |
| --- | ---: | ---: | ---: |
| macOS ARM64 | 20 368 / 31 122 | 1 171 876 / 1 748 326 | 2 176 / 2 688 |
| Linux ARM64 | 20 368 / 31 118 | 1 171 876 / 1 748 133 | 664 / 2 152 |
| Linux x86-64, översatt | 20 368 / 31 118 | 1 171 876 / 1 748 135 | 6 060 / 5 568 |

Dunders pool återanvände 13 750 allokeringar per körning. Samma två program fick identisk storlek på terminalströmmen i varje upprepning och på varje mätt arkitektur.

## Kör själv på macOS eller Linux

Kör från repots rot. Dunderbiblioteket och Dunderproven bygger offline. Rustreferensens låsta paket kan hämtas en gång med Cargo och sedan byggas med `--offline`.

```sh
mkdir -p build
dunder build --locked --offline benchmarks/widgets.ddr -o build/widgets
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --example widget_bench
build/measure build/widgets build/rust/release/examples/widget_bench build/pty
```

Varje resultatrad anger läge, bibliotek, fall, p50 och p95 i ns, medelbyte per bild, lägsta och högsta p50 samt lägsta och högsta p95. Utan sista argumentet mäts enbart minnesläget. För en enskild körning kan `build/pty build/widgets build/widgets.ansi` även spara hela terminalströmmen.

```sh
dunder build --locked --offline --runtime-profile benchmarks/widgets.ddr -o build/widgets-profile
build/widgets-profile --allocations
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust-profile --features allocation-counts --example widget_bench -- --allocations
```

För högsta RSS, kör varje tidsmätt binär med `/usr/bin/time -l` på macOS eller `/usr/bin/time -v` på Linux. [Mätprogram](widgets.ddr) · [PTY-drivrutin](pty.ddr) · [Upprepning och sammanställning](measure.ddr)
