# Täckning mot Ratatui 0.30.2

Referensen är taggen `ratatui-v0.30.2`, inklusive de stabila tillvalen. Tabellen följer de publika områdena i den versionen. En genomförd jämförelse anges med sitt konkreta prov. [Referenskod](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2) · [Publikt API](https://docs.rs/ratatui/0.30.2/ratatui/) · [Funktionsflaggor](https://docs.rs/crate/ratatui/0.30.2/features)

Funktionsparitet styr ordningen. Loggboken använder en inbäddad vy där användaren kan behålla händelser i skalets skrollhistorik. Prestanda mäts tillsammans med beteendet så att en långsammare arbetslast kan prioriteras utifrån vad användaren märker. [Fynd för Dunder-språket och standardbiblioteket](dunder.md)

| Område | Finns i Dunder | Återstår att leverera eller verifiera |
| --- | --- | --- |
| Buffer och Cell | Återanvända bildbuffertar, breda tecken, celldiff, exakt och ungefärlig sammanslagning av linjesymboler, färgutdata och avläsning av rader och cellstilar. Buffertar med förskjutet origo, rektangulär kopiering, linjär storleksändring och tillämpning av en diff på en delyta. Buffertsammanslagning, avläsning av en hel cell, fyra cellregler och strömmande besök av ändringar. | Större ytor än bibliotekets nuvarande gräns på 4 096 celler per axel och 1 048 576 celler totalt. |
| Stil och färg | Nio attribut, påslag och borttagning, ärvda stilar, återställning, 16 namngivna färger, 256 index, RGB och separat understrykningsfärg. Färger kan läsas från strängar och byggas från HSL eller HSLuv. Tailwind och Material ger 500 namngivna RGB-värden. | Beständiga dataformat och full täckning för konvertering mellan stilrepresentationer. |
| Text | Span, Line, Text, Paragraph, maskering per skalär, justering, skrollning, ord- och radbrytning och förberedd layout. Grafemgränser enligt UAX #29. | Strängbredder för skriftspecifika ligaturer över grafemgränser, full täckning för sammansatta emoji och alla operationer på fristående texttyper. |
| Layout | Rektangeloperationer, marginaler per axel, sex storleksregler, sju flexlägen, överlappning, centrering och cache. 15 076 jämförda layouter, 1 200 rektangelprov och 1 100 jämförda bilder. | Utökad täckning av överbestämda system med många olika paneler och upprepade ändringar av reglerna. |
| Block, listor och tabeller | Block med 18 kantsymboluppsättningar, egna tecken, individuella kanter, justerade titlar, utfyllnad och skuggor. Flerradiga, stiliserade listposter med beständigt tillstånd, skrollmarginal, markör och valbar ritriktning. Tabeller med sidhuvud, sidfot, kolumnspann, flerradiga celler och tillstånd för rad och kolumn. | Fler prov för extrema marginaler och avstånd. |
| Övriga widgets | Clear och Fill på Frame, Tabs, Gauge, LineGauge, Scrollbar, Sparkline, grupperade BarChart, månadskalender med händelser och egna datumstilar, Paragraph, Chart och canvas med koordinatgränser, lager, nio markörer, former, kartor och etiketter. | Logotyp och maskot. |
| Egna widgets och tester | Frame ritar egna och inbyggda widgets med `render_widget` och `render_stateful_widget`. Tillstånd kan uppdateras vid ritning med `run_stateful`. Egna backends kan anslutas till Terminal. TestBackend har skrollhistorik, markör, fem rensningslägen och regionskrollning. | Bredare testning av specialiserade terminalbackends. |
| Terminal och backend | Alternativ skärm, tangentbord, storleksändringar, återställning, markörstyrning från Frame och Terminal, bildräknare, ANSI och Win32. Fast och inbäddad vyport, explicit storleksanpassning, terminalens tecken- och pixelmått, manuell cellutmatning och byte av arbetsbuffert, loggrader, rensning och regionskrollning. Återställning efter skrivfel och uteblivet markörsvar har provats. | Marköråterställning för egna backends när terminalen släpps, fel mitt i skrollning och nativ Windowskonsol. |
| Stabil Rust-integration | Jämförelseprogrammen använder den låsta Rustreferensen. Dunder-appar byggs offline. | Kontrollera Dunder-motsvarigheter till bekvämlighetsmakron, serialisering, cachning och backendval där dessa påverkar appens beteende. |
| Effektivitet och plattformar | Tid, minne, allokeringar och faktiska terminalbyte för en widgetvy på macOS och Linux. Pseudoterminalprov på macOS och Linux ARM64/x86-64. Win32-prov under Wine. | Nativ Windows x64, fler arbetslaster med komplex layout, lägre kostnad för dynamiska färgvärden och paritet på fysisk x86-64. |

Stilprovet genererar en bild med 1 024 celler via Dunders publika API, tolkar terminalströmmen och jämför varje attribut, färg och position med både Ratatuis buffert och Crossterms terminalström. Det täcker alla 512 kombinationer av de nio attributen. Ytterligare 512 färgtripplar konverteras med både HSL och HSLuv i båda programmen. [Dunderprovet](../tests/style_reference.ddr) · [Referens och terminalavkodare](../benchmarks/reference/examples/styles.rs)

```sh
mkdir -p build
dunder build --locked --offline tests/style_reference.ddr -o build/style-reference
build/style-reference | cargo run --quiet --locked --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example styles
```

Palettprovet placerar 500 färger i ett rutnät på 25 × 20 celler. Varje färg används en gång som förgrund, bakgrund och understrykningsfärg. Cellerna jämförs med de namngivna konstanterna i Ratatui 0.30.2. Tailwind bidrar med 22 skalor med 11 nyanser, Material med 19 skalor med 10 nyanser och 16 skalor med fyra accenter. Båda har därutöver svart och vitt i RGB. [Dunderprov](../tests/palette_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/palettes.rs) · [Färgkällor](../src/PALETTE-NOTICES.txt)

```sh
mkdir -p build
dunder build --locked --offline tests/palette_reference.ddr -o build/palettes
build/palettes >build/dunder-colors
cargo run --locked --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example palettes >build/ratatui-colors
diff -u build/ratatui-colors build/dunder-colors
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

## Terminal, markör och skrollhistorik i 0.10.0

I 60 körningar med 45 operationer vardera jämförs skärmens och skrollhistorikens celler, för- och bakgrund, fetstil, markörens synlighet och position samt numret på senast visad bild. Proven ritar om, byter storlek, flyttar markören, rensar fem slags områden, lägger till rader och skrollar regioner i båda riktningar. Alla 2 700 tillstånd matchar Ratatui 0.30.2 med stöd för skrollregioner. Separata prov täcker 65 535 sparade rader, kombinerande tecken, breda tecken, understrykningsfärg och återhämtning efter skrivfel. [Dunderprov](../tests/terminal_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/terminal.rs) · [Bibliotekstester](../tests/suite.ddr)

Samma PTY-drivrutin kör en Dunderapp och en Ratatui-app med Crossterm. Den läser både text och markör efter förflyttning utan celländringar, omritning under en synlig markör, döljning och storleksändring. Den kontrollerar att terminalens läge återställs när programmet avslutas. Detta har körts på macOS ARM64 och båda Linuxmålen. Win32-provet under Wine läser dessutom den aktiva och den återställda skärmbuffertens markörinformation från konsolens API. [PTY-prov](../tests/cursor_terminal.ddr) · [Windowsprov](../tests/windows_console.ddr) · [Tid, byte, minne och allokeringar](../benchmarks/terminal.md)

```sh
mkdir -p build
dunder build --locked --offline tests/terminal_reference.ddr -o build/terminal-states
dunder build --locked --offline tests/cursor_app.ddr -o build/cursor-app
dunder build --locked --offline tests/cursor_terminal.ddr -o build/cursor-pty
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features terminal-oracle --example terminal --target-dir build/rust
build/terminal-states | build/rust/release/examples/terminal
build/cursor-pty build/cursor-app
build/cursor-pty build/rust/release/examples/terminal --live
```

## Fasta och inbäddade vyportar i 0.11.0

90 startlägen ger 1 440 jämförelser med Ratatui 0.30.2. Provet fyller skärmen med kontrolltecken före start och jämför skärmens och skrollhistorikens tecken, röd förgrund, fetstil, vyportens rektangel och markörens position och synlighet efter varje operation. Det omfattar bredare och smalare skärm, flyttad fast vy, en vy som fyller hela fönstret och fler loggrader än skärmens höjd. [Dunderprov](../tests/viewport_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/viewports.rs)

Samma pseudoterminalprov kör den fasta vyn och loggboken mot både Dunder och Crossterm. Det läser av cellerna efter upprepade infogningar och jämför terminalinställningarna före och efter. Dunderprovet skickar också tangenttryckningar runt ett markörsvar och låter en markörförfrågan löpa ut. På macOS provades loggboken dessutom i en tmux-panel med sex rader. Hela historiken innehöll skalets förtext, händelserna 1–8 i ordning, loggboken och skalets eftertext. [PTY-drivrutin](../tests/viewport_terminal.ddr) · [Win32-prov](../tests/windows_console.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/viewport_reference.ddr -o build/viewports
dunder build --locked --offline tests/viewport_terminal.ddr -o build/viewport-pty
dunder build --locked --offline tests/fixed_app.ddr -o build/fixed
dunder build --locked --offline --example loggbok -o build/loggbok
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features terminal-oracle --example viewports --target-dir build/rust
build/viewports | build/rust/release/examples/viewports
build/viewport-pty build/fixed build/loggbok
build/viewport-pty build/rust/release/examples/viewports build/rust/release/examples/viewports --reference
```


## Månadskalender i 0.12.0

Referensen använder det stabila tillvalet `widget-calendar`. 4 800 månader från 1600 till 1999 ger samma veckodag, antal dagar, veckor, bredd och höjd. Datumprovet omfattar även föregående dag och dagnumret efter byte till en kortare månad. Bildprovet kombinerar 36 datum, 16 inställningar och nio ytstorlekar. Det jämför varje tecken, förgrund, bakgrund och fyra textattribut, inklusive datum med överlappande stilar och ett eget veckoslutsregister. [Datum- och bildprov](../tests/calendar_reference.ddr) · [Ratatui-referens](../benchmarks/reference/examples/calendar.rs)

Av 5 184 bilder är 2 982 identiska i hela bufferten. I 2 202 skriver referensen datum nedanför kalenderns inneryta. Dunder klipper vid nederkanten för att skydda ramen och nästa widget. Referensprovet återställer cellerna utanför innerytan till deras förväntade innehåll efter att ramen ritats och kräver därefter identiska bilder. [Kalenderns publika API](../src/calendar.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/calendar_reference.ddr -o build/calendar-states
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features calendar-oracle --example calendar --target-dir build/rust
build/calendar-states | build/rust/release/examples/calendar
build/calendar-states --dates | build/rust/release/examples/calendar --dates
```


## Bildbuffert och celldiff i 0.13.0

`Frame.merge` använder båda buffertarnas absoluta koordinater. Den sista bufferten bestämmer innehållet där ytorna överlappar och tomrummet mellan dem fylls med standardceller. Om ytan skulle överskrida buffertgränsen returneras ett fel och ursprungsbufferten bevaras. 2 880 jämförelser mot Ratatui omfattar tomma, angränsande, överlappande och upprepat sammanslagna ytor. Varje cell jämförs med sitt tecken, förgrund, bakgrund, understrykningsfärg, attribut och cellregel. Det omfattar även fallet där högerhalvan av ett brett tecken täcks. [Dunderprov](../tests/merge_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/merges.rs)

`Frame.visit_changes` besöker ändrade koordinater i ritordning och kan avbrytas när besökaren returnerar `false`. `diff_positions` samlar samma koordinater i en array. Båda använder den gemensamma höjden och rapporterar fel om origo eller bredd skiljer sig. De fyra cellreglerna är `.None`, `.Skip`, `.AlwaysUpdate` och `.ForcedWidth(1..65535)` inklusive båda gränserna. Ett prov jämför 10 000 ordnade par av buffertar mot Ratatuis `diff_iter`, avkodar Dunders terminalutdata och kontrollerar koordinater, tecken och stilar för varje skrivning. Det omfattar bland annat variationsväljaren U+FE0F, breda tecken, regler som täcker nästa rad och borttagning av en regel. [Dunderprov](../tests/buffer_reference.ddr) · [Referens och avkodare](../benchmarks/reference/examples/buffers.rs)

En andra skrivare ändrar två celler i en pseudoterminal. Nästa bild skyddar den första, återställer den andra och kan sedan återta båda cellerna. Win32-provet gör samma ändring i en konsolbuffert och läser tillbaka tecknen. Vid storleksändring rensas helskärmsvyn innan den nya bilden skickas. [Pseudoterminal](../tests/buffer_terminal.ddr) · [Win32](../tests/windows_console.ddr) · [Bildtider och terminalutdata](../benchmarks/buffers.md)

```sh
mkdir -p build
dunder build --locked --offline tests/buffer_reference.ddr -o build/buffers
dunder build --locked --offline tests/merge_reference.ddr -o build/merges
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example buffers --example merges --target-dir build/rust
build/buffers | build/rust/release/examples/buffers
build/merges | build/rust/release/examples/merges
```

## Fönstermått och storleksanpassning i 0.15.0

På macOS och Linux frågar `window_size()` först processens styrterminal och använder standardutmatningen om styrterminalen inte går att öppna. Operativsystemet lämnar både tecken- och pixelmått i samma svar. En lyckad fråga kan ge noll på valfri axel. När båda terminalvägarna saknas returneras ett fel. Win32-backenden ger ett fel för pixelmått, precis som den jämförda Crossterm-versionen. Egna backends som bara implementerar `size` återanvänder dessa cellmått och ger noll på båda pixelaxlarna. Testbackenden rapporterar 640 × 480 pixlar oavsett cellmått. [Ratatuis backendgränssnitt](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-core/src/backend.rs) · [Frågan i Dunder](../src/posix.ddr)

Ett terminalprov startar båda implementationerna i egna sessioner. Det ändrar först enbart pixelmåtten och sedan både celler och pixlar. Det provar även 16-bitarsgränserna, nollstorlek, en omdirigerad standardutmatning med och utan styrterminal och två olika terminaler i samma process. I tmux på macOS gav båda programmen 60 × 16 celler och 960 × 512 pixlar, följt av 79 × 27 celler och 1 264 × 864 pixlar efter storleksändring. Ett separat prov jämför 1 440 tillstånd för helskärm, fast och inbäddad vy. Det läser storleken utan att rita, ändrar storlek före första ritningen och jämför vyport, markör, skrollhistorik och varje cells tecken och stil med Ratatui. [Terminalprov](../tests/window_terminal.ddr) · [Dunderprogram](../tests/window_probe.ddr) · [Ratatui-program](../benchmarks/reference/examples/viewports.rs)

```sh
mkdir -p build
dunder build --offline --locked tests/viewport_reference.ddr -o build/views
dunder build --offline --locked tests/window_probe.ddr -o build/window-probe
dunder build --offline --locked tests/window_terminal.ddr -o build/window-terminal
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --features terminal-oracle --example viewports --target-dir build/rust
build/views --window | build/rust/release/examples/viewports --window
build/window-terminal build/window-probe
build/window-terminal build/rust/release/examples/viewports
```

Den optimerade terminalen jämfördes även för oförändrad bild, en ändrad cell, rullning, storleksändring och full omritning. [Bildtid, terminalutdata, allokeringar och minne i 0.15.0](../benchmarks/buffers.md#ommätning-i-0150)

## Markören mellan bilder i 0.16.0

Testet jämför 1 440 förlopp mot Ratatui 0.30.2. Det läser celler, stilar, skrollhistorik, markör och vyport efter direkta marköranrop, storleksändringar och infogade rader. Ett separat terminalprov följer 13 steg i helskärm och i en fast vy som går utanför skärmens kant. En misslyckad ritfunktion lämnar den senaste lyckade bilden åtkomlig. Vid nästa lyckade ritning återställs markören även om alla celler är oförändrade. Efter ett muterbart backendlån skickar ritningen bildens önskade markörläge på nytt. I ANSI-läget kan det kosta ett extra synlighetskommando på sex byte. [Markörprovet](../tests/direct_cursor_terminal.ddr) · [Ratatuis terminal](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-core/src/terminal/cursor.rs)

På macOS och Linux använder ett terminalprov ett svar som slutar på exakt den 65 536:e byten efter markörfrågan. Alla 65 530 inklistrade tecken framför svaret och tangenten efter svaret går att läsa i ordning. Om kön fylls av enbart tangentdata får appen ett fel. Efter att appen läst 128 tecken lyckas nästa markörfråga och återstoden av inklistringen finns kvar. På Win32 kontrolleras markörposition, storlek och synlighet mot konsolens egna uppgifter. Windowsprovet kördes under Wine. [Win32-prov](../tests/windows_console.ddr) · [Läsning av en bytekö](../benchmarks/language/byte_queue.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/direct_cursor_app.ddr -o build/direct
dunder build --locked --offline tests/direct_cursor_terminal.ddr -o build/probe
dunder build --locked --offline tests/viewport_reference.ddr -o build/views
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --features terminal-oracle --example viewports --example terminal --target-dir build/rust
build/views --direct-cursor | build/rust/release/examples/viewports --direct-cursor
build/probe ./build/direct
build/probe ./build/direct --queued-reply
build/probe ./build/rust/release/examples/terminal
```

För samma 1 100 bilder finns median, p95, spridning, allokeringar och terminalbyte för de fem arbetslasterna. [Terminalmätning i 0.16.0](../benchmarks/buffers.md#ommätning-i-0160)

## Manuell rendering i 0.17.0

`get_frame()` öppnar den aktuella arbetsbufferten med en ny markörbegäran. Cellerna finns kvar mellan anrop. `current_buffer_mut()` ger direkt åtkomst till samma celler. Skrivningarna syns när en celldiff skickas till backenden eller när appen visar en hel bild med `draw`, `apply_buffer` eller `apply_buffer_with_cursor`. De tre sistnämnda anropen uppdaterar markören och ökar bildräknaren efter lyckad utmatning.

Det går också att skicka diffen med `flush()`, byta arbetsbuffert med `swap_buffers()` och tömma backendens utmatning var för sig. En ren celldiff behåller markörens synlighet. Vid textutmatning kan själva skrivningen flytta markören. En egen backend kan implementera `draw_cells` och använda `frame.ansi_cells` för exakt den utmatningen. För tidigare backends som enbart implementerar `present` ger manuell cellutmatning ett fel. Om cellskrivningen misslyckas finns arbetsbuffertens innehåll kvar. Om backendens avslutande tömning misslyckas är nästa arbetsbuffert tom och bildräknaren står kvar. Rita om innehållet så skickas en hel bild.

Ett prov läser både den väntande bilden och den faktiska testskärmen vid 24 steg för 90 olika terminaler. Det jämför celltext och stil, skrollhistorik, markör, bildnummer och vyport med Ratatui 0.30.2. Stegen omfattar upprepad cellutmatning, buffertbyte, skärmändring, loggrad och övergång till vanlig ritning. Pseudoterminalprovet följer båda implementationerna genom tio steg i helskärm och i en förskjuten fast vy. På Win32 läser provet tillbaka konsolens tecken och markör via systemets egna anrop. [Referensens buffertoperationer](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-core/src/terminal/buffers.rs) · [Ratatuis fullständiga ritning](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-core/src/terminal/render.rs) · [Test av buffrad utmatning och skrivfel](../tests/suite.ddr)

```sh
mkdir -p build
dunder build --offline --locked tests/viewport_reference.ddr -o build/views
dunder build --offline --locked tests/manual_app.ddr -o build/manual-app
dunder build --offline --locked tests/manual_terminal.ddr -o build/manual-pty
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --features terminal-oracle --example viewports --example terminal --target-dir build/rust
build/views --manual | build/rust/release/examples/viewports --manual
build/manual-pty ./build/manual-app
build/manual-pty ./build/rust/release/examples/terminal
build/rust/release/examples/terminal --final-flush-error
```

Median, p95, allokeringar, minne och antal terminalbyte för 1 100 bilder finns redovisade per plattform och arbetslast. [Mätning i 0.17.0](../benchmarks/buffers.md#ommätning-i-0170)

## Sammanhängande kurvor och fyllda ytor i 0.18.0

I 2 475 jämförelser ritar Dunder samma kurva med ett anrop som Ratatui ritar med `Points` följt av en `Line` eller `FilledLine` per segment. Provet kombinerar nio markörer, fem ytstorlekar och fyra baslinjer med tomma kurvor, enstaka och upprepade punkter, riktningsbyten och linjer som korsar diagrammet med båda ändarna utanför. Det jämför symboler, färger, attribut och kantceller efter att ytterligare en färg lagts ovanpå. Ett separat prov kontrollerar breda egna markörer och uppehåll i kurvan vid icke-ändliga koordinater.

För en sammanhängande serie återanvänder ritaren föregående skärmpunkt. Fyllda segment räknar ut första och sista cellens punktmask och fyller de hela cellerna däremellan. Diagrammet kan skriva varje färdigt lager till bilden direkt. Detta kräver inga sparade kopior av appens koordinatserier mellan bilder. [Dunderprov](../tests/plot_reference.ddr) · [Ratatui-referens](../benchmarks/reference/examples/plot.rs) · [Bildtid före och efter samt mot Ratatui](../benchmarks/buffers.md#ommätning-i-0180)

```sh
mkdir -p build
dunder build --locked --offline tests/plot_reference.ddr -o build/plot-check
cargo build --locked --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example plot --target-dir build/rust
build/plot-check --paths | build/rust/release/examples/plot --paths
```
