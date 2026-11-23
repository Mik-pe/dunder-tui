# Täckning mot Ratatui 0.30.2

Referensen är taggen `ratatui-v0.30.2`, inklusive de stabila tillvalen. Tabellen följer de publika områdena i den versionen. En genomförd jämförelse anges med sitt konkreta prov. [Referenskod](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2) · [Publikt API](https://docs.rs/ratatui/0.30.2/ratatui/) · [Funktionsflaggor](https://docs.rs/crate/ratatui/0.30.2/features)

Funktionsparitet styr ordningen. Nästa användarflöde är vyportar, markörstyrning och en testbackend för program som kombinerar vanlig terminalutskrift med en interaktiv vy. Prestanda mäts tillsammans med beteendet så att en långsammare arbetslast kan prioriteras utifrån vad användaren märker. [Fynd för Dunder-språket och standardbiblioteket](dunder.md)

| Område | Finns i Dunder | Återstår att leverera eller verifiera |
| --- | --- | --- |
| Buffer och Cell | Återanvända bildbuffertar, breda tecken, celldiff, exakt och ungefärlig sammanslagning av linjesymboler, färgutdata och avläsning av rader och cellstilar. | Fristående buffertoperationer med origo, ändrad storlek och sammanslagning. Cellernas regler för att hoppa över eller alltid rita. |
| Stil och färg | Nio attribut, påslag och borttagning, ärvda stilar, återställning, 16 namngivna färger, 256 index, RGB och separat understrykningsfärg. Färger kan läsas från strängar och byggas från HSL eller HSLuv. | Material- och Tailwind-paletter, beständiga dataformat och full täckning för konvertering mellan stilrepresentationer. |
| Text | Span, Line, Text, Paragraph, maskering per skalär, justering, skrollning, ord- och radbrytning och förberedd layout. Grafemgränser enligt UAX #29. | Strängbredder för skriftspecifika ligaturer över grafemgränser, full täckning för sammansatta emoji och alla operationer på fristående texttyper. |
| Layout | Rektangeloperationer, marginaler per axel, sex storleksregler, sju flexlägen, överlappning, centrering och cache. 15 076 jämförda layouter, 1 200 rektangelprov och 1 100 jämförda bilder. | Utökad täckning av överbestämda system med många olika paneler och upprepade ändringar av reglerna. |
| Block, listor och tabeller | Block med 18 kantsymboluppsättningar, egna tecken, individuella kanter, justerade titlar, utfyllnad och skuggor. Flerradiga, stiliserade listposter med beständigt tillstånd, skrollmarginal, markör och valbar ritriktning. Tabeller med sidhuvud, sidfot, kolumnspann, flerradiga celler och tillstånd för rad och kolumn. | Fler prov för extrema marginaler och avstånd. |
| Övriga widgets | Clear och Fill på Frame, Tabs, Gauge, LineGauge, Scrollbar, Sparkline, grupperade BarChart, Paragraph, Chart och canvas med koordinatgränser, lager, nio markörer, former, kartor och etiketter. | Kalender med datumstilar, logotyp och maskot. |
| Egna widgets och tester | Funktioner ritar på en Frame. Tillstånd kan uppdateras vid ritning med `run_stateful`. En Frame kan testas utan terminal. | Gemensamma gränssnitt för widgets med eget tillstånd och utbytbara backends, testbackend med skrollhistorik och markör. |
| Terminal och backend | Alternativ skärm, tangentbord, storleksändringar, återställning, ANSI och Win32. | Fast och infogad vyport, innehåll ovanför vyn, markörstyrning från Frame, regionrensning och regionskrollning, fönstrets pixelmått, livscykel- och felprov. |
| Stabil Rust-integration | Jämförelseprogrammen använder den låsta Rustreferensen. Dunder-appar byggs offline. | Kontrollera Dunder-motsvarigheter till bekvämlighetsmakron, serialisering, cachning och backendval där dessa påverkar appens beteende. |
| Effektivitet och plattformar | Tid, minne, allokeringar och faktiska terminalbyte för en widgetvy på macOS och Linux. Pseudoterminalprov på macOS och Linux ARM64/x86-64. Win32-prov under Wine. | Nativ Windows x64, fler arbetslaster med komplex layout, lägre kostnad för dynamiska färgvärden och paritet på fysisk x86-64. |

Stilprovet genererar en bild med 1 024 celler via Dunders publika API, tolkar terminalströmmen och jämför varje attribut, färg och position med både Ratatuis buffert och Crossterms terminalström. Det täcker alla 512 kombinationer av de nio attributen. Ytterligare 512 färgtripplar konverteras med både HSL och HSLuv i båda programmen. [Dunderprovet](../tests/style_reference.ddr) · [Referens och terminalavkodare](../benchmarks/reference/examples/styles.rs)

```sh
mkdir -p build
dunder build --locked --offline tests/style_reference.ddr -o build/style-reference
build/style-reference | cargo run --quiet --locked --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example styles
```

Fler körbara prov täcker ändrade bildrader, delade SGR-återställningar, överlappande text, färgade understreck och terminalåterställning. [Bibliotekstester](../tests/suite.ddr) · [Unix-terminal](../tests/posix_terminal.ddr) · [Win32-konsol](../tests/windows_console.ddr) · [Prestandamätningar](../benchmarks/README.md)

## Text i 0.4.0

Styckeprovet kombinerar två sorters radbrytning och klippning med rad- och textjustering, två skrollaxlar och åtta bredder. Fyra dokument innehåller bland annat tomrader, kontrolltecken, olika sorters mellanrum, sammansatta emoji och blandade stilar. Dunder ritar varje bild innanför en kant av röda kontrolltecken. Provet läser tecken, position, förgrund, bakgrund, fetstil och kursivering och kontrollerar hela kanten.

Av 4 608 bilder är 4 164 identiska med Ratatui 0.30.2. I 432 fall skiljer sig bilderna efter att ett brett tecken har passerat styckets högerkant i referensen. I ytterligare 12 fall avbryter Ratatui med ett indexfel i en två kolumner bred buffert. Jämförelseprogrammet upprepar ritningen med extra utrymme för att kontrollera när den första överträdelsen sker. Dunder flyttar hela tecknet till nästa rad när det får plats där. En separat jämförelse ger 152 identiska bilder för fristående text och textspann. De 96 referensbilderna från prestandaprovet stämmer också med både vanlig och förberedd rendering. [Dunderprov](../tests/text_reference.ddr) · [Ratatui-referens](../benchmarks/reference/examples/text.rs)

```sh
mkdir -p build
dunder build --locked --offline tests/text_reference.ddr -o build/text-reference
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --example text --target-dir build/rust
build/text-reference | build/rust/release/examples/text
for mode in --widths --inline --benchmark; do
    build/text-reference "$mode" | build/rust/release/examples/text "$mode"
done
```

Alla 766 officiella prov för grafemgränser i Unicode 17.0 och alla 1 111 999 Unicode-skalärer utom kontrolltecken passerar sina respektive jämförelser. Grafemgränserna jämförs med Unicode och varje skalärs cellbredd med Ratatuis `CellWidth`. För att återskapa tabellerna lokalt, packa upp respektive versions UCD i varsin katalog och kör följande från repots rot. Biblioteket och dess tester använder de incheckade tabellerna direkt. [Generator](../tools/unicode.ddr) · [Unicode 17](https://www.unicode.org/Public/17.0.0/ucd/) · [Unicode 18](https://www.unicode.org/Public/18.0.0/ucd/)

```sh
dunder run --locked --offline tools/unicode.ddr -- /sökväg/till/UCD-17 /sökväg/till/UCD-18
```

## Widgets i 0.5.0

Bildprovet ritar på en färgad bakgrund med en kontrollkant runt widgeten. För varje cell jämförs tecken, för- och bakgrundsfärg samt de attribut som används i provet. Av 24 516 bilder är 24 485 identiska över hela bufferten. I resterande 31 bilder skriver Ratatuis stapeldiagram utanför sin tilldelade ruta. Dunder matchar alla celler inuti rutan och behåller omgivningen intakt. Provet inkluderar samtliga 36 tillstånd som används för att mäta diagramvyn. Ytterligare 50 700 kombinationer av linjetecken och sammanslagningsregler ger samma symbol som Ratatui. [Dunderprov](../tests/widget_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/widgets.rs) · [Bildtider och terminalutdata](../benchmarks/widgets.md)

Kör båda jämförelserna från repots rot.

```sh
mkdir -p build
dunder build --locked --offline tests/widget_reference.ddr -o build/widget-reference
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --example widgets --target-dir build/rust
build/widget-reference | build/rust/release/examples/widgets
build/widget-reference --merge | build/rust/release/examples/widgets --merge
```

Blockkanter och vissa widgets tar ett enskilt tecken som symbol. `Frame.fill_symbol` tar även ett grafem med flera Unicode-skalärer. Stapeldiagrammet delar en färgsatt värdeetikett vid en hel grafemgräns och använder terminalkolumner för dess bredd.

## Listor i 0.6.0

Provet utför 18 åtgärder per följd. Det ändrar markering, första synliga post, listans innehåll och terminalens storlek. Bilderna omfattar bland annat tomma listor, nollhöga och för höga poster, blandade radhöjder, två ritriktningar och tre regler för markörens utrymme. Tecken, förgrund, bakgrund, använda attribut, markering och första synliga post kontrolleras efter varje ritning. [Ratatuis list-API](https://docs.rs/ratatui/0.30.2/ratatui/widgets/struct.List.html)

Alla 38 880 listtillstånd matchar Ratatui. I 37 033 fall matchar hela bilden inklusive kontrollkanten. I 1 847 fall skriver referensens markör utanför listans ruta. Där kontrolleras också resultatet med Ratatui i en buffert begränsad till rutan. Dunder bevarar grannytan. Ytterligare 1 320 bilder och tillstånd från prestandaprovets lista med 16 384 poster är identiska. [Dunderprov](../tests/list_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/lists.rs) · [Mätningar](../benchmarks/lists.md)

```sh
mkdir -p build
dunder build --locked --offline tests/list_reference.ddr -o build/list-check
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --target-dir build/rust --example lists
build/list-check | build/rust/release/examples/lists
build/list-check --benchmark | build/rust/release/examples/lists --benchmark
```

På macOS och Linux kan bokhyllans publika exempel provas i en pseudoterminal. Provet startar en separat process, förflyttar markeringen med piltangent, End och Home, ändrar fönstrets bredd och läser tillbaka titeln och anteckningen. Det kontrollerar också återställningen av terminalen. Win32-provet läser den upprepade markören och båda textraderna direkt från konsolbufferten. [Unix-prov](../tests/bokhylla_terminal.ddr) · [Win32-prov](../tests/windows_console.ddr)

```sh
dunder build --locked --offline examples/bokhylla/main.ddr -o build/bokhylla
dunder run --locked --offline tests/bokhylla_terminal.ddr -- build/bokhylla
```

## Layout i 0.7.0

Provet jämför alla paneler och mellanrum, även tomma ytor och överlappande grannar. De 14 096 grundfallen omfattar en kombinationstabell och 2 000 deterministiskt genererade layouter. Ytterligare 980 prov når bland annat 65 535 kolumner, stora fyllnadsvikter, nollvikter, högsta möjliga kvottäljare och mycket stora mellanrum. Rektangelprovet jämför marginaler, förflyttning, storleksändring, snitt, union, inpassning, positioner och rad- och kolumnåtkomst i 1 200 fall. Mätvyn jämför tecken och stilar i samtliga 1 100 bilder. [Dunderprov](../tests/layout_reference.ddr) · [Rustreferens](../benchmarks/reference/examples/layout.rs) · [Mätningar](../benchmarks/layout.md)

När flera placeringar uppfyller motstridiga regler lika väl kan även två Ratatui-processer välja olika rektanglar. För varje avvikelse räknar jämförelseprogrammet ut det lägsta möjliga viktade felet med exakta rationella tal. Det löser sedan samma problem två gånger med båda programmens avrundade cellgränser som extra villkor. Båda måste nå samma minimum och alla mellanrum måste kunna återskapas från kanterna. [Ratatuis beskrivning av motstridiga villkor](https://docs.rs/ratatui/0.30.2/ratatui/layout/struct.Layout.html) · [Kontroll med exakt aritmetik](../benchmarks/reference/src/layout_optimum.rs)

I Dunder har rektanglar signerade 32-bitarskoordinater. En separat gränskontroll visar också en skillnad för `Min(65535)` bredvid `Length(7)` på 100 kolumner. Dunder låter minimikravet få alla 100 kolumner. Ratatui 0.30.2 konverterar minimikravet till ett signerat 16-bitarstal och lämnar sju kolumner åt grannen.

```sh
mkdir -p build
dunder build --locked --offline tests/layout_reference.ddr -o build/layout-check
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example layout --target-dir build/rust
build/layout-check | build/rust/release/examples/layout
for mode in --limits --geometry --frames; do
    build/layout-check "$mode" | build/rust/release/examples/layout "$mode"
done
```

Biblioteket löser vanliga fördelningar direkt. För kombinationer som behöver vägas mot varandra använder det en egen gles simplexmetod. Varje layout kan spara lösningar för upp till 500 olika innermått som standard. `cache_capacity(0)` stänger av återanvändningen. `arrange(area).regions()` lånar resultatet fram till nästa ändring av layouten, `split(area)` ger appen egna rektanglar och `split_into(area, &mut out)` fyller en buffert som appen återanvänder. Alla dessa anrop kan användas offline.

## Tabeller i 0.8.0

Provet kör 20 åtgärder för varje kombination av storlek, tabellkonfiguration och datamängd. Efter varje åtgärd jämförs de ritade cellerna, stilarna, första synliga raden och vald rad och kolumn. Det täcker bland annat tomma och ojämna tabeller, radmarginaler, kolumnspann, alla sju flexlägen, ändrade kolumnbredder, markeringssymboler och byte av data medan en cell är vald. [Dunderprov](../tests/table_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/tables.rs)

I en körning matchade 30 752 av 33 600 bilder hela referensbufferten direkt. För 2 144 bilder valde layoutlösarna olika likvärdiga höjder för sidhuvud, innehåll och sidfot. Provet kontrollerar båda lösningarnas optimala fel med exakt aritmetik och ritar om med Ratatui på de kontrollerade höjderna. I 704 bilder skrev referensen utanför tabellens inneryta. Där jämförs hela bilden med en avgränsad Ratatui-buffert och kontrollkanten måste vara intakt. Alla 33 600 tillstånd matchade. Antalet likvärdiga layouter kan variera mellan körningar.

Ytterligare 1 320 bilder med 4 096 rader matchade referensen cell för cell i prestandaprovets sex arbetslaster. Terminalprovet startar tidtabellen som en egen process, navigerar i båda led, krymper fönstret till två rader och kontrollerar återställningen. [Terminalprov](../tests/table_terminal.ddr) · [Bildtid och terminalutdata](../benchmarks/tables.md)

```sh
mkdir -p build
dunder build --locked --offline tests/table_reference.ddr -o build/table-check
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example tables --target-dir build/rust
build/table-check | build/rust/release/examples/tables
build/table-check --benchmark | build/rust/release/examples/tables --benchmark
dunder build --locked --offline --example avganger -o build/avganger
dunder run --locked --offline tests/table_terminal.ddr -- build/avganger
```

## Diagram och canvas i 0.9.0

Ritprovet ger 8 424 identiska bilder mot Ratatui 0.30.2. Det täcker koordinatomvandling, klippning, linjer med och utan fyllning, rektanglar, cirklar, kartor, markeringsbyte, upprepade lager och stiliserade etiketter. Alla 2 304 prov med subpixelmönster matchar i både tecken och färger. Separat jämförs de exakta flyttalsbitarna för kartornas 1 166 respektive 5 125 koordinatpar. [Dunderprov](../tests/plot_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/plot.rs) · [Ratatuis ritgränssnitt](https://docs.rs/ratatui/0.30.2/ratatui/widgets/canvas/)

Diagramprovet omfattar 4 320 bilder med fyra graftyper, båda axlarna, rubriker, justering, tomma dataserier och samtliga åtta legendpositioner. I 4 104 bilder matchar hela bufferten direkt. I 216 bilder börjar Ratatuis första högerjusterade x-etikett utanför diagrammet. Där verifieras att referensen faktiskt ändrade en kontrollcell. Därefter ritas samma diagram med Ratatui i en buffert med diagrammets gränser och hela bilden jämförs på nytt. Dunder håller även en centrerad, fullhög teckenförklaring inom diagrammet när dess vänsterkant har förskjutits. Alla 1 100 bilder i prestandaprovet matchar cell för cell. [Diagramprov](../tests/chart_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/charts.rs) · [Mätningar](../benchmarks/charts.md)

Proven använder Dunders publika API. Win32-provet läser tillbaka halvblockets båda färger och en ritad diagramlinje. Unixprovet startar den färdigbyggda kartappen, byter longitud och detaljgrad, pausar och återupptar svepet, ändrar terminalstorlek och kontrollerar återställning. [Terminalprov](../tests/coast_terminal.ddr) · [Konsolprov](../tests/windows_console.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/plot_reference.ddr -o build/plot-check
dunder build --locked --offline tests/chart_reference.ddr -o build/chart-check
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example plot --example charts --target-dir build/rust
build/plot-check | build/rust/release/examples/plot
build/plot-check --patterns | build/rust/release/examples/plot --patterns
build/plot-check --coordinates | build/rust/release/examples/plot --coordinates ~/.cargo/registry/src/*/ratatui-widgets-0.3.2/src/canvas/world.rs
build/chart-check | build/rust/release/examples/charts
build/chart-check --benchmark | build/rust/release/examples/charts --benchmark
```

Kartdata ligger i paketet och kan återskapas med `dunder run tools/coastlines.ddr -- world.dat world_110m.txt`. Generatorn läser originalens koordinater och skriver en Dundermodul till standardutgången. Lågupplösningsserien utelämnar slutpunkten (−69,31°, −52,81°) för att matcha referensens urval. [Lågupplösta data](https://gnuplotting.org/data/world.dat) · [Högupplösta data från Natural Earth](https://gnuplotting.org/data/world_110m.txt) · [Data och villkor](../src/COASTLINE-NOTICES.txt)
