# Diagramvy i 0.5.0

## Vågräta staplar på main med Dunder 0.12.1

Med `--horizontal` får de tolv staplarna egna rader och värden på upp till nio siffror. Rullning ändrar både signalen och stapelvärdena. För varje arbetslast ritas 20 uppvärmningsbilder och 200 mätbilder. Tabellen visar medianen av sju körningars p50 och p95 i mikrosekunder vid skrivning till pseudoterminal. TSV-filen innehåller även mätningarna i minne och lägsta och högsta uppmätta p50 och p95. [Alla fem arbetslaster](results/horizontal-bars-0.12.1.tsv) · [Celljämförelse av alla 36 skärmtillstånd](../docs/parity.md#widgets-i-050)

| Plattform | Oförändrad, Dunder | Oförändrad, Ratatui | Full omritning, Dunder | Full omritning, Ratatui | Högsta RSS i KiB, Dunder / Ratatui¹ |
| --- | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | 34 / 74 | 48 / 77 | 89 / 145 | 145 / 235 | 2 144 / 2 864 |
| Linux ARM64 | 34 / 75 | 54 / 101 | 62 / 140 | 143 / 240 | 552 / 2 204 |
| Linux x86-64 med CPU-översättning | 242 / 439 | 88 / 188 | 553 / 983 | 211 / 400 | 6 108 / 5 616 |

¹ Medianen av tre separata processers högsta RSS. Alla körningar gjordes på samma Apple M5. Dunder skrev 4 009 072 byte och Ratatui 6 987 963 byte till pseudoterminalen över 1 100 bilder på varje plattform.

På macOS gav samma scen med den tidigare stapelritaren 82 923 allokeringsanrop och 3 040 529 begärda byte. När siffrorna skrevs direkt i cellerna blev det 22 423 anrop och 1 201 401 byte på alla tre plattformar. Ratatui gav 17 928 anrop och 1 795 238 byte på macOS samt 17 924 anrop och 1 795 045 byte på Linux. Dunder hade lägre medianbildtid i samtliga fem PTY-fall på ARM64. På översatt x86-64 var Ratatui snabbare i samtliga fem. [Tidigare stapelritare](https://github.com/Mik-pe/dunder-tui/blob/9aee09bb692c96999b99565ff35068640c465388/src/barchart.ddr)

```sh
mkdir -p build
dunder build --locked --offline benchmarks/widgets.ddr -o build/widgets
dunder build --locked --offline benchmarks/measure.ddr -o build/measure
dunder build --locked --offline benchmarks/pty.ddr -o build/pty
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --features layout-cache --example widget_bench
build/measure build/widgets build/rust/release/examples/widget_bench build/pty -- --horizontal
dunder build --locked --offline --runtime-profile benchmarks/widgets.ddr -o build/widgets-profile
build/widgets-profile --horizontal --allocations
cargo run --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust-profile --features layout-cache,allocation-counts --example widget_bench -- --horizontal --allocations
```

## Den vertikala diagramvyn i 0.5.0

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

## Mätare och stapelsymboler med Dunder 0.11.3

Samma 36 widgettillstånd och fem bildändringar mättes lokalt mot Ratatui 0.30.2. Tabellen visar p50 / p95 i mikrosekunder vid skrivning till en pseudoterminal på macOS ARM64. Varje värde är medianen av sju alternerade körningar efter två uppvärmningskörningar. Resultatfilen innehåller även min- och maxvärden för varje serie och motsvarande mätning utan terminalskrivning. [Alla 60 mätserier](results/widget-cells-dunder-0.11.3-local.tsv)

| Ändring | Dunder | Ratatui |
| --- | ---: | ---: |
| Oförändrad | 27 / 71 | 40 / 87 |
| Ett tecken | 29 / 31 | 41 / 85 |
| Rullning | 131 / 164 | 140 / 173 |
| Ny bredd | 163 / 216 | 187 / 254 |
| Full omritning | 204 / 244 | 253 / 375 |

I Linuxgästen på ARM64 tog en oförändrad bild 66 / 76 µs mot Ratatuis 43 / 95 µs. På översatt Linux x86-64 hade Ratatui lägre p50 och p95 för alla fem fall. Värddatorn körde också andra virtuella maskiner och intervallen mellan upprepningarna var breda. Varje körning på alla tre plattformarna skrev exakt 4 113 141 byte för Dunder och 7 939 285 för Ratatui inklusive uppvärmning.

På macOS gjorde Dunder 23 858 allokeringsanrop och begärde 1 189 668 byte. Rustreferensen gjorde 31 122 anrop och begärde 1 748 420 byte. Medianen av högsta RSS över fem separata körningar var 2 128 KiB respektive 2 960 KiB.

## Egna blockkanter med Dunder 0.11.3

Diagramvyn mättes om med samma förberedda widgetdata och fem bildändringar. På macOS ARM64 tog full omritning till en pseudoterminal 204 / 237 µs för Dunder och 247 / 297 µs för Ratatui (p50 / p95). På Linux ARM64 hade Dunder lägre p50 i alla fem fall. För rullning var p95 128 µs mot Ratatuis 122 µs. På översatt Linux x86-64 var Ratatui snabbare i samtliga fall. Mätfilen innehåller spridningen mellan körningarna, både med och utan terminalskrivning. [60 mätserier](results/border-cells-dunder-0.11.3-local.tsv)

Dunder skrev 4 113 141 byte mot Ratatuis 7 939 285 på varje plattform. På Mac var antalet allokeringsanrop 23 786 respektive 31 122 och den begärda mängden 1 180 308 respektive 1 748 420 byte. Medianen av högsta RSS över fem körningar var 2 144 respektive 2 960 KiB. Det separata konstruktionsprovet hämtar alla åtta symboler i varje ram, med växling mellan 18 kantsymboluppsättningar. [Konstruktionsprov för `--runtime-profile`](border_sets.ddr)

## Samma diagramvy med Dunder 0.12.0

Med den publicerade kompilatorn hade Dunder lägre p50 och p95 i alla fem PTY-fall på macOS och Linux ARM64. På översatt Linux x86-64 hade Ratatui lägre p50 och p95 i alla fem. Vid full omritning på Mac tog Dunder 99 / 109 µs mot Ratatuis 150 / 168 µs. Motsvarande tider på översatt x86-64 var 704 / 739 µs och 209 / 239 µs. Systemlasten varierade mellan mättillfällena för kompilatorversionerna. Varje rad i resultattabellen har en referens körd vid samma tillfälle. [Alla bildtider, byte och intervall](results/border-cells-dunder-0.12.0-local.tsv)

På Mac gav profilerna 18 777 allokeringsanrop och 1 105 036 begärda byte för Dunder mot 31 122 och 1 748 420 för Ratatui. Medianen för högsta RSS var 2 144 respektive 2 688 KiB. De 1 100 bilderna inklusive uppvärmning gav 4 113 141 respektive 7 939 285 terminalbyte på varje plattform.

## Efter stöd för sparade teman

I diagramvyn på macOS ARM64 tog full omritning till en pseudoterminal 100 / 112 µs med Dunder och 149 / 167 µs med Ratatui (p50 / p95). På Linux ARM64 blev tiderna 71 / 77 respektive 125 / 136 µs. På översatt Linux x86-64 tog samma bild 690 / 723 µs med Dunder och 208 / 241 µs med Ratatui. Varje redovisad tid är medianen från sju körningar efter två uppvärmningar. [Samtliga fem arbetslaster, byte och spridning](results/themes-dunder-0.12.0-local.tsv)

På varje plattform skrev diagramvyn 4 113 141 terminalbyte med Dunder och 7 939 285 med Ratatui under 1 100 bilder. Mac-profilerna gav 18 777 allokeringsanrop och 1 105 036 begärda byte för Dunder, respektive 31 122 och 1 748 330 för Ratatui. Medianen för högsta RSS över fem körningar var 2 144 och 2 704 KiB. Bildtiden gäller ritning från redan förberedda stilar.

## Efter stöd för sparade bildbuffertar

För en celländring i pseudoterminalen på macOS ARM64 blev bildtiden 71 / 88 µs i Dunder och 51 / 95 µs i Ratatui (p50 / p95). Spridningen i de sju körningarnas medianer var 33–73 respektive 45–80 µs. På Linux ARM64 tog samma ändring 37 / 91 respektive 46 / 96 µs. På översatt Linux x86-64 var skillnaden större, 290 / 604 respektive 76 / 185 µs. Båda Linuxgästerna kördes lokalt på Macen. [Alla fem arbetslaster och körningarnas spridning](results/buffer-json-dunder-0.12.0-local.tsv)

De 1 100 bilderna gav 4 113 141 byte till terminalen med Dunder och 7 939 285 med Ratatui på varje värd. På Macen gjorde Dunder 18 777 allokeringsanrop och begärde 1 105 036 byte. Ratatui gjorde 31 122 anrop och begärde 1 748 370 byte. Medianen för högsta RSS i fem processer var 2 144 respektive 2 880 KiB. Tidtagningen omfattar ritning av en förberedd diagramvy. JSON-inläsning sker utanför den ritloopen.
