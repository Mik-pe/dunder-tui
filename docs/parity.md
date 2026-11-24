# Täckning mot Ratatui 0.30.2

Referensen är taggen `ratatui-v0.30.2`, inklusive de stabila tillvalen. Tabellen följer de publika områdena i den versionen. En genomförd jämförelse anges med sitt konkreta prov. [Referenskod](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2) · [Publikt API](https://docs.rs/ratatui/0.30.2/ratatui/) · [Funktionsflaggor](https://docs.rs/crate/ratatui/0.30.2/features)

Funktionsparitet styr ordningen. Loggboken använder en inbäddad vy där användaren kan behålla händelser i skalets skrollhistorik. Prestanda mäts tillsammans med beteendet så att en långsammare arbetslast kan prioriteras utifrån vad användaren märker. [Fynd för Dunder-språket och standardbiblioteket](dunder.md)

| Område | Finns i Dunder | Återstår att leverera eller verifiera |
| --- | --- | --- |
| Buffer och Cell | Förskjutet origo, koordinater och linjära index, råa celler och stilar, celldiff och fyra diffregler. Kopiering, storleksändring och sammanslagning inom hela koordinatintervallet. | Minnesgränsen för sammanhängande Dunder-arrayer och avvikelsen i Ratatuis sammanslagning vid stora index beskrivs nedan. |
| Stil och färg | Nio attribut, påslag och borttagning, ärvda stilar, återställning, 16 namngivna färger, 256 index, RGB och separat understrykningsfärg. Färger kan läsas från strängar och byggas från HSL eller HSLuv. Tailwind och Material ger 500 namngivna RGB-värden. | JSON för färger och stilar jämförs nedan. Fler konverteringar mellan backendens stilrepresentationer återstår. |
| Text | Span, Line, Text, Paragraph, byte och omordning av rad- och spannsamlingar, inkrementell redigering, sammanslagning av dokument och läsning av stiliserade grafem, stiländringar, läsning av textens delar, utskrift, maskering per skalär, justering, skrollning, ord- och radbrytning och förberedd layout. Grafemgränser enligt UAX #29. Cellprov för Unicode 18:s emojitest, presentationsvarianter och skriftspecifika ligaturer inom och över grafemgränser. | Fler jämförelser av sammansatta dokument i listor och tabeller. Se de tolv gränsfallen vid fysisk högerkant nedan. |
| Layout | Rektangeloperationer, marginaler per axel, sex storleksregler, sju flexlägen, överlappning, centrering, riktningsbyte och cache. 22 408 jämförda layouter, 2 460 rektangelprov och två vyer med vardera 1 100 bilder. Blandade regler provas med upp till 24 paneler. | Större blandade system och nativa x64-mätningar för frekvensbanden. För stora marginaler och råa rektanglar vid koordinatgränsen, se JSON-provet nedan. |
| Block, listor och tabeller | Block med 18 kantsymboluppsättningar, egna strängar för hörn och sidor, individuella kanter, justerade titlar, utfyllnad och skuggor. För skuggor jämförs 85 050 råa celler och celldiffarnas ordning med referensen. Flerradiga, stiliserade listposter med beständigt tillstånd, skrollmarginal, markör och valbar ritriktning. Tabeller med sidhuvud, sidfot, kolumnspann, flerradiga celler och tillstånd för rad och kolumn. | Fler prov för extrema marginaler och avstånd. |
| Övriga widgets | Fristående Clear och Fill med cellvisa symboler, stil och klippning, Tabs, Gauge, LineGauge, Scrollbar, Sparkline, grupperade BarChart, månadskalender med händelser och egna datumstilar, Paragraph, Chart och canvas med koordinatgränser, lager, nio markörer, former, kartor och etiketter. Ratatuis båda logotyper och maskot med två ögonfärger. | Se beteendeproven per widget nedan. |
| Egna widgets och tester | Frame ritar egna och inbyggda widgets med `render_widget` och `render_stateful_widget`. Tillstånd kan uppdateras vid ritning med `run_stateful`. Egna backends kan anslutas till Terminal. TestBackend har skrollhistorik, markör, fem rensningslägen och regionskrollning. | Bredare testning av specialiserade terminalbackends. |
| Terminal och backend | Alternativ skärm, tangentbord, storleksändringar, återställning, markörstyrning från Frame och Terminal, bildräknare, ANSI och Win32. Fast och inbäddad vyport, explicit storleksanpassning, terminalens tecken- och pixelmått, manuell cellutmatning och byte av arbetsbuffert, loggrader, rensning och regionskrollning. Marköråterställning för egna backends samt återhämtning efter skrivfel och uteblivet markörsvar. Regionskrollning och radmatningsbaserad infogning jämförs separat. | Nativ Windows x64 för de senaste ändringarna i logginfogning. |
| Stabil Rust-integration | Dunders konstruktorer jämförs med alla åtta makron för rik text, tabellrader och layout. Jämförelseprogrammen använder den låsta Rustreferensen. Dunder-appar byggs offline. | Skillnader mellan terminalbackends och exakta gränsfall för layout beskrivs nedan. |
| Effektivitet och plattformar | Tid, minne, allokeringar och terminalbyte på macOS och Linux. Pseudoterminal på macOS ARM64, Linux ARM64 och nativ Linux x86-64. Win32-prov på Windows Server 2025 x64 och under Wine. | Fler arbetslaster med komplex layout, lägre kostnad för dynamiska färgvärden och likvärdig bildtid på x86-64. |

### Stabila funktionsflaggor

| Flaggor i Ratatui 0.30.2 | Motsvarighet och verifiering |
| --- | --- |
| `all-widgets`, `widget-calendar` | `tui.calendar` ingår i biblioteket. Datum, händelser, stil och visade månader jämförs i kalenderprovet. |
| `underline-color`, `palette` | Separat understrykningsfärg ingår i cell- och terminalproven. De två palettkonverteringarna från HSL och HSLuv jämförs i stilprovet. |
| `layout-cache` | `Layout.cache_capacity` styr antal sparade uppdelningar. Layoutproven återanvänder tidigare storlekar efter regelbyten. |
| `scrolling-regions` | Backendens regionskrollning och reservvägen med radmatning jämförs genom skrollhistorik och terminalutdata. |
| `macros` | Alla åtta makron körs i referensen. Deras tomma, uppräknade, formaterade och upprepade former jämförs med Dunder där respektive makro stöder formen. [Konstruktorprov](#bygg-text-tabellrader-och-layout) |
| `serde` | Färger, stilar, attribut, celler, diffregler, buffertar, layouter och geometri läses och skrivs med `std.json`. Detsamma gäller testbackenden, widgettillstånden, Tailwind- och Materialpaletter, horisontell och vertikal justering, kanttyper, kantflaggor och utfyllnad. Stilar, celler, buffertar och testterminaler jämförs med och utan `underline-color`. Referensens `Span`, `Line` och `Text` saknar Serde-format. |
| `crossterm`, `crossterm_0_28`, `crossterm_0_29`, `termina`, `termion`, `termwiz` | Dunder använder POSIX och Win32 direkt. Den låsta Crossterm-referensen och de faktiska terminalproven täcker bland annat ritning, markör och återställning. Skillnader mellan referensens övriga backendval är öppna. |
| `std`, `portable-atomic`, `default` | Dunder bygger offline med sitt standardbibliotek. `portable-atomic` gäller referensens mål utan atomära typer. `default` väljer kalender, Crossterm, layoutcache, makron och understrykningsfärg. |

Flaggorna och deras kopplingar kommer från [det låsta manifestet](https://docs.rs/crate/ratatui/0.30.2/source/Cargo.toml.orig). [Layoutprov](../tests/layout_reference.ddr) · [Färgprov](../tests/style_reference.ddr) · [Kalenderprov](../tests/calendar_reference.ddr) · [Terminalprov](../tests/viewport_reference.ddr)

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

### Bygg text, tabellrader och layout

| Ratatui | Dunder |
| --- | --- |
| `span!` | `Span::default`, `raw`, `styled` och Dunders formatsträngar |
| `line!`, `text!` | `Line` och `Text` har `default`, `strings`, `new` och `repeat` |
| `row!` | `Row::default()` ger höjd noll. `Row::new([])` och `Row::repeat(cell, 0)` har höjd ett. `Cell::from` tar bland annat sträng, spann, rad och dokument. |
| `constraint!`, `constraints!` | `Constraint` med `Length`, `Min`, `Max`, `Percentage`, `Ratio` och `Fill`, arrayer eller `regel.repeat(antal)` |
| `horizontal!`, `vertical!` | `Layout::horizontal` och `Layout::vertical` med samma regler |

Provet jämför 740 bilder med 81 992 råa celler över fem ytstorlekar. Det kontrollerar också textbredder, formatering, rad- och kolumnindelning samt mellanrum. Upprepade rader och celler redigeras efter konstruktion. Raderna före och efter en tom tabellrad synliggör skillnaden i höjd. [Dunder](../tests/constructor_reference.ddr) · [Ratatuis åtta makron](../benchmarks/reference/examples/constructors.rs) · [Avgångstavla med 1 100 jämförda bilder](../benchmarks/manifest.md)

```sh
mkdir -p build
dunder build --locked --offline tests/constructor_reference.ddr -o build/constructors
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features macro-oracle --example constructors --target-dir build/reference
build/reference/release/examples/constructors >build/reference-cells
build/constructors >build/dunder-cells
cmp build/reference-cells build/dunder-cells
dunder run --locked --offline tests/constructor_terminal.ddr
```

### Återställ paletter, justering och kanter

Av 1 893 indata gav båda biblioteken samma sparade värde i 1 108 fall och avvisade övriga 785. Provet jämför varje cell i 492 bilder, totalt 21 014 celler. Det täcker alla inbyggda färgskalor, egna färger, äldre RGB- och indexobjekt, saknade och dubbla fält, alla 256 kantmasker, tolv kanttyper och placering av bred text vid tre rutstorlekar. [Dunderprov](../tests/palette_json.ddr) · [Ratatui-prov med indata](../benchmarks/reference/examples/palette_json.rs)

Läs en kantmask som `block.BorderMask` för att spara även okända bitar. Anropa `edges()` när du ritar ramen. Sparad horisontell justering har tre riktningar. Använd `Option<text.Alignment>` för ett tema där riktningen kan ärvas.

Samma resultat kontrollerades på macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine. I en pseudoterminal bytte en gul etikett i övre vänstra hörnet plats till en cyan etikett nere till höger och ramen bytte till dubbel linje. Win32-provet läste tillbaka tecken och färgattribut från konsolbufferten. Nativ Windows x64 återstår för dessa prov. [Pseudoterminal](../tests/palette_json_terminal.ddr) · [Win32](../tests/windows_console.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/palette_json.ddr -o build/palette-json
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features theme-oracle --example palette_json --target-dir build/reference
build/reference/release/examples/palette_json --cases >build/palette-input
build/reference/release/examples/palette_json <build/palette-input >build/palette-rust
build/palette-json build/palette-input >build/palette-dunder
cmp build/palette-dunder build/palette-rust
dunder run --locked --offline tests/palette_json_terminal.ddr
```

### Färger och stilar i JSON

1 833 indata jämförs med Ratatuis `serde` och `underline-color`. För varje godkänd stil jämförs den sparade JSON-strängen, resultatet av att kombinera två stilar och stilen i två ritade celler. Den ena cellen börjar tom, den andra har färg och attribut. Proven täcker alla 256 färgindex, äldre RGB- och indexobjekt, alla 512 kombinationer av namngivna attribut, extra attributbitar, dubbla fält, saknade värden och felaktiga tal. [Dunderprov](../tests/theme_reference.ddr) · [Ratatui-referens och indata](../benchmarks/reference/examples/themes.rs)

På macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine gav jämförelsen samma resultat för alla indata. På de tre Unix-målen laddade ett pseudoterminalprov två stilar, växlade från RGB till indexfärg efter en tangenttryckning och kontrollerade terminalens återställning. Nativ Windows x64 återstår för temaprovet. [Terminalprov](../tests/theme_terminal.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/theme_reference.ddr -o build/themes
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml \
  --features theme-oracle --example themes --target-dir build/rust
ref=build/rust/release/examples/themes
"$ref" --cases >build/theme-input
"$ref" <build/theme-input >build/expected
build/themes build/theme-input >build/actual
diff -u build/expected build/actual
dunder run --locked --offline tests/theme_terminal.ddr
```

### JSON utan separat understrykningsfärg

Ratatui utan `underline-color` sparar sex fält per cell. En cell i arrayform placerar attributen direkt efter bakgrunden. I objektform ignoreras nyckeln `underline_color` även om den upprepas eller innehåller ett ogiltigt färgvärde. Attributet `UNDERLINED` fungerar i båda formaten. Dunder väljer format per anrop, så att samma program kan läsa båda sorternas bilder. [Formatvalet](../src/snapshot.ddr) · [Terminalprovet](../tests/buffer_json_terminal.ddr)

Referensen byggdes med `--no-default-features --features serde-backend`. Med standardvalet skulle Crossterm slå på understrykningsfärg indirekt. 1 838 färg- och stilfall, 4 538 cell-, buffert- och difffall och 424 testbackendfall gav identiska utdata på macOS ARM64, Linux ARM64 och översatt Linux x86-64. Win64 under Wine matchade samma 6 800 referenssvar och konsolprovet läste tillbaka den gröna etiketten. Nativ Windows x64 återstår för detta format. [Stilprov](../tests/theme_reference.ddr) · [Buffertprov](../tests/buffer_json.ddr) · [Backendprov](../tests/backend_json.ddr) · [Återställningstid och minne](../benchmarks/scrollback.md#json-utan-separat-understrykningsfärg)

```sh
mkdir -p build
ref=benchmarks/reference/Cargo.toml
cargo build --release --locked --offline --manifest-path "$ref" --no-default-features \
  --features serde-backend --example themes --example buffer_json --example backend_json --target-dir build/ref
for exe in themes buffer_json backend_json; do
  src=$exe
  if [ "$exe" = themes ]; then src=theme_reference; fi
  dunder build --locked --offline "tests/$src.ddr" -o "build/$exe"
  "build/ref/release/examples/$exe" --cases >"build/$exe.in"
  "build/ref/release/examples/$exe" <"build/$exe.in" >"build/$exe.rust"
  "build/$exe" "build/$exe.in" --without-underline-color >"build/$exe.dunder"
  cmp "build/$exe.rust" "build/$exe.dunder" || exit 1
done
```

### Jämför testterminalens skärm, skrollhistorik och markör

Ratatuis sex `assert_*`-metoder bestämmer utfallet i 167 fall. De 147 buffertfallen körs i Dunder med både `Frame` och `Buffer`. Av 314 anrop godkändes 98. De övriga 216 gav det förväntade felet för cell, geometri, historik eller markör. Utfallet stämde på macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine. [Testprogram](../tests/backend_assertions.ddr) · [Ratatui-referens](../benchmarks/reference/examples/backend_assertions.rs)

Provet jämför även celler som döljs av ett brett tecken. En oskriven cell och ett uttryckligt mellanslag kan vara lika, medan exempelvis olika understrykningsfärger eller cellflaggor gör att testet visar första avvikande koordinat.

```sh
mkdir -p build
dunder build --locked --offline tests/backend_assertions.ddr -o build/assertions
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features backend-oracle --example backend_assertions --target-dir build/reference
build/reference/release/examples/backend_assertions >build/assertions.tsv
build/assertions --verify build/assertions.tsv
```

### Återställ testterminalen

`testing.TestBackend` läser och skriver Ratatuis skärmbuffert, skrollhistorik, markörens synlighet och dess två 16-bitarskoordinater. `testing.SavedBackend` kan även läsa en rå ögonblicksbild med avvikande geometri. För att använda den som terminal med `to_backend()` måste båda buffertarna börja i origo, ha rätt cellantal och samma radbredd. Markören kan ligga utanför skärmen. [Dunder-API](../src/testing.ddr) · [Ratatuis implementation](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-core/src/backend/test.rs)

418 fall jämför två påföljande ritningar, markörflyttning, fem rensningslägen, skrollning i båda riktningar och storleksändring. Bland fallen ingår ogiltiga indata, färgade textrader, emoji och delvis rensade breda tecken. Totalt jämförs 2 336 fullständiga tillstånd och 1 296 utvalda rader vid historikens gräns på 65 535 rader, sammanlagt 56 616 celler. Alla utdata matchade på macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine. I Unix-pseudoterminalen flyttar nedåtpilen nästa sparade rad till historiken. Win32-provet läser de två bilderna ur konsolbufferten. [Dunderprov](../tests/backend_json.ddr) · [Referens och indata](../benchmarks/reference/examples/backend_json.rs) · [Pseudoterminal](../tests/backend_json_terminal.ddr) · [Prestanda](../benchmarks/scrollback.md)

```sh
mkdir -p build
dunder build --locked --offline tests/backend_json.ddr -o build/backend-json
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features backend-oracle --example backend_json --target-dir build/rust
ref=build/rust/release/examples/backend_json
"$ref" --cases > build/backend-cases
"$ref" < build/backend-cases > build/backend-expected
build/backend-json build/backend-cases > build/backend-actual
cmp build/backend-expected build/backend-actual
dunder run --locked --offline tests/backend_json_terminal.ddr
```

### Sparade markeringar och skrollpositioner

Listor, tabeller och skrollister läser Ratatuis sparade tillstånd via `std.json`. `list.SavedState`, `table.SavedState` och `widgets.SavedScrollState` behåller hela det osignerade 64-bitarsintervallet. De vanliga typerna `ListState`, `TableState` och `ScrollState` kan läsa och skriva samma format inom sina befintliga 32-bitarsgränser. Båda varianterna kan skickas direkt till `render_stateful_widget`. En tom rityta behåller indexet. En synlig lista eller tabell justerar det efter antalet poster och kolumner.

2 545 JSON-indata jämförs med Ratatui 0.30.2. Provet navigerar före första ritningen, ritar 311 giltiga listtillstånd och 619 tabelltillstånd, flyttar markeringen och jämför den nya bilden. För 1 448 giltiga skrolltillstånd jämförs båda axlarna, de fyra kanterna, framåt, bakåt, första och sista position. Totalt jämförs **177 644 celler** med symboler, färger och attribut. Fallen omfattar även fem enumtyper, positionsbaserade fält, saknade och upprepade nycklar samt tal på båda sidor om 32- och 64-bitarsgränserna. Vid mycket stora skrollvärden följer tummens avrundning referensens optimerade 64-bitarsbygge. [Dunderprov](../tests/widget_state_json.ddr) · [Ratatui och indata](../benchmarks/reference/examples/widget_state_json.rs)

Macen och båda Linuxgästerna körde även ett tangentprov som öppnar sista raden och flyttar markeringen uppåt. Win32-provet läser tillbaka listan, tabellen och skrolltummen från konsolbufferten. Samtliga JSON-fall matchade på macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine. [Unix-terminal](../tests/widget_state_terminal.ddr) · [Win32-konsol](../tests/windows_console.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/widget_state_json.ddr -o build/states
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml \
  --features theme-oracle --example widget_state_json --target-dir build/rust
ref=build/rust/release/examples/widget_state_json
"$ref" --cases >build/state-input
"$ref" <build/state-input >build/expected-states
build/states build/state-input >build/actual-states
cmp build/expected-states build/actual-states
dunder run --locked --offline tests/widget_state_terminal.ddr
```

### Sparade celler och bildbuffertar

En `Buffer` kan skapas tom, förifylld eller från strängar, spann och rader. Celler kan lånas, ändras och återställas på plats. Provet jämför sammanslagning av kantsymboler, alla diffregler inklusive äldre `skip`, stiländringar med överlappande attribut, klippt text, oanvända celler under breda tecken, storleksändring och sammanslagning av buffertar med olika origo. Förberett och direkt skrivet innehåll provas även med 5 986 emoji- och presentationssekvenser. [Dunderprov](../tests/cell_edits.ddr) · [Ratatui-prov](../benchmarks/reference/examples/cell_edits.rs)

Alla 7 383 tillstånd, 65 050 fysiska celler och returnerade skrivpositioner stämde på macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine. En rå cell kan vara oskriven eller innehålla ett uttryckligt mellanslag. Här jämförs även den skillnaden. Ett separat prov skrev var och en av Unicodes 1 112 064 skalärer och jämförde symbol och skrivbredd mot Ratatui på samma mål. De lokala Unix-pseudoterminalerna och Win32-konsolen under Wine kontrollerade utmatningen när en extern skrivning träffade en skyddad cell. [Terminalprov](../tests/buffer_terminal.ddr) · [Win32-prov](../tests/windows_console.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/cell_edits.ddr -o build/cells
cargo build --locked --release --offline --manifest-path benchmarks/reference/Cargo.toml --features theme-oracle --example cell_edits --target-dir build/reference
set -- tests/data/emoji-18.txt tests/data/emoji-variation-18.txt
build/cells "$@" >build/cells.out
build/reference/release/examples/cell_edits "$@" <build/cells.out
build/cells --scalars >build/scalars.out
build/reference/release/examples/cell_edits --scalars <build/scalars.out
```

Med Ratatuis `serde` och `underline-color` jämförs 4 520 fall. Provet läser 536 enskilda celler, 517 attributuppsättningar, 16 diffregler och 744 buffertar. Ytterligare 2 700 bildpar jämför ordningen på de ändrade cellerna. I sju prov ritar båda biblioteken färgad text, sparar bilden, läser tillbaka den och skriver på nästa rad. Båda bilderna jämförs byte för byte i JSON. [Dunderprov](../tests/buffer_json.ddr) · [Ratatui och indata](../benchmarks/reference/examples/buffer_json.rs)

En orörd cell, ett explicit mellanslag och en tom sträng behåller var sitt JSON-värde. Orörda celler och mellanslag med samma stil och diffregel är lika vid omritning. Proven täcker också 16-bitars attributmasker, färgade understreck, celler bakom breda symboler, `ForcedWidth` och det äldre fältet `skip`. `tui.Buffer` kan läsa en rå bild med avvikande cellantal. För att bli en ritbar `Frame` måste antalet stämma med ytan och koordinaterna rymmas i terminalen. Formatet utan understrykningsfält återstår att jämföra.

Samtliga 4 520 fall gav samma svar på macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine. I terminalprovet byter en sparad rad från `SAVED 01` till `SAVED 02`. Den nya bildens fjärde cell innehåller ett utropstecken med `skip`, så det tidigare E:et står kvar på skärmen. [Unix-terminal](../tests/buffer_json_terminal.ddr) · [Win32-konsol](../tests/windows_console.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/buffer_json.ddr -o build/cells
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml \
  --features theme-oracle --example buffer_json --target-dir build/rust
ref=build/rust/release/examples/buffer_json
"$ref" --cases >build/cell-input
"$ref" <build/cell-input >build/expected-cells
build/cells build/cell-input >build/actual-cells
cmp build/expected-cells build/actual-cells
dunder run --locked --offline tests/buffer_json_terminal.ddr
```

### Sparade layouter och geometri

Provet jämför 1 035 JSON-indata med Ratatui 0.30.2. Var och en av de 797 giltiga layouterna delas på fem ytor. Samtliga 3 985 indelningar jämförs panel för panel, inklusive mellanrummen. På ytan 22 × 5 jämförs också varje ritat tecken, sammanlagt 87 670 celler. I 144 rektangelfall jämförs bland annat marginaler, kantkoordinater, rader, kolumner, förflyttning av en position och area. Felaktig aritet, dubbla nycklar, okända fält och tal utanför typens intervall ingår. [Dunderprov](../tests/layout_json.ddr) · [Indata och Ratatui-referens](../benchmarks/reference/examples/layout_json.rs)

En rektangel med x = 65 523 och bredd 35 behåller båda värdena när den sparas igen. Vid horisontell indelning slutar panelerna vid 65 535. Vid vertikal indelning behåller de bredd 35. `with_spacing(.Space(n))` och `with_spacing(.Overlap(n))` bevarar hela det namngivna 16-bitarsvärdet, även skillnaden mellan noll mellanrum och noll överlappning. Ratatui 0.30.2 tolkar värdet som ett tecknat 16-bitarstal i lösaren. Därför ger exempelvis `Space(65535)` en cells överlappning. Vanliga `.spacing(1)` ger ett mellanrum på en cell.

Alla indata gav samma resultat på macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine. I pseudoterminal växlar en tangent från två kolumner med mellanrum till överlappande rader. Win32-provet läser cellerna direkt ur konsolbufferten efter båda ritningarna. Nativ Windows x64 återstår för den här revisionen. [Pseudoterminal](../tests/layout_json_terminal.ddr) · [Win32-konsol](../tests/windows_console.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/layout_json.ddr -o build/layout-json
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml \
  --features theme-oracle --example layout_json --target-dir build/rust
ref=build/rust/release/examples/layout_json
"$ref" --cases >build/layout-input
"$ref" <build/layout-input >build/expected
build/layout-json build/layout-input >build/actual
diff -u build/expected build/actual
dunder run --locked --offline tests/layout_json_terminal.ddr
```

## Loggrader i en inbäddad vy

Med och utan skrollregioner jämförs vardera 1 440 vyportstillstånd mot Ratatui 0.30.2. Två ytterligare serier om 45 bilder jämför varje rå cell i skärmen och historiken, inklusive färgat understreck, kombinerande tecken, breda symboler, diffregler och ordningen på ritade celler. Vid överskrivning av en befintlig rad skriver båda biblioteken även celler märkta `Skip`. [Vyportprov](../tests/viewport_reference.ddr) · [Cellprov](../tests/insert_cells.ddr) · [Ratatuis implementation](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-core/src/terminal/inline.rs)

En buffrad backend provar skrivfel före och efter varje operation vid två vyhöjder och tre skrollsätt. Nästa lyckade ritning återställer vyn och markören. I pseudoterminalen sätts först en begränsad skrollregion. Sedan stängs utmatningsfilen under ett skrollanrop. Efter att filen öppnats igen provas ritning, radmatning, markörflytt, markörfråga och avslut var för sig. Ett separat prov avslutar direkt efter nio loggrader och kontrollerar var skalet får tillbaka markören. [Buffrat felprov](../tests/scroll_errors.ddr) · [Pseudoterminalprov](../tests/scroll_terminal.ddr) · [Win32-konsolprov](../tests/windows_console.ddr)

Kör lokalt med Dunder 0.12.0. Rustkommandona använder referensprogrammets låsta paket från den lokala Cargo-cachen.

```sh
mkdir -p build
dunder test --locked --offline
dunder build --locked --offline tests/scroll_terminal.ddr -o build/scroll-terminal
build/scroll-terminal
dunder build --locked --offline tests/viewport_reference.ddr -o build/views
dunder build --locked --offline tests/insert_cells.ddr -o build/cells
for mode in feeds regions; do
  features=style-oracle
  view_arg=--line-feeds
  cell_arg=
  if [ "$mode" = regions ]; then
    features=style-oracle,terminal-oracle
    view_arg=
    cell_arg=--regions
  fi
  cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml \
    --features "$features" --example viewports --example insert_cells --target-dir build/rust
  build/views $view_arg >build/view-states && build/rust/release/examples/viewports <build/view-states || exit 1
  build/cells $cell_arg >build/insert-states && build/rust/release/examples/insert_cells <build/insert-states || exit 1
done
```

## Stora buffertar i 0.24.0

`Frame::try_at(area)` och `frame.try_resize(area)` tar en rektangel och returnerar ett fel om lagringen blir för stor. Efter en nekad storleksändring går det att fortsätta använda bufferten och dess tidigare innehåll. `index_of(x,y)` och `position_at(index)` räknar från buffertens eget origo. Axlarna kan var för sig nå 65 535. En fast vy i en större terminal behöver bara utrymme för cellerna i vyn.

Provet jämför raderna i nio rektanglar med totalt 2 283 794 celler. I 90 storleksändringar jämförs även varje rå cell, stil, diffregel och ordningen på ändringarna. När en ny radgräns delar ett brett tecken bevaras symbolen i sin ursprungliga cell. Ett separat pseudoterminalprov skriver i kolumn 65 534, ändrar den med ett tangenttryck och läser av den återställda terminalen. [Buffertprov](../tests/large_buffers.ddr) · [Ratatui-referens](../benchmarks/reference/examples/large_buffers.rs) · [Terminalprov](../tests/wide_terminal.ddr)

Vid sammanslagning av två rader med startkolumn 62 003 placerar den optimerade Ratatui-versionen `D` på (58 467, 65 531). Dunder bevarar källpositionen (62 003, 65 532). Jämförelsen kontrollerar båda positionerna och hela resultatet mot koordinatvis kopiering. En hel yta på 65 535 × 65 535 celler ryms fortfarande inte. Dunder 0.11.2 har högst 1 GiB per sammanhängande minnesblock och enbart stilfältet behöver åtta byte per cell. [Sammanslagningsprov](../benchmarks/reference/examples/large_buffers.rs) · [Dunders allokerare](https://github.com/Mik-pe/dunder/blob/v0.11.2/selfhost/runtime/runtime.ddr)

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

## Blockkanter med egna symboler

`Block.border_set` tar åtta strängar för ramens sidor och hörn. `border_type` återställer hela uppsättningen till den valda typen. En ram kan skickas till `frame.render_widget` eller ritas med `block.render` för att även få tillbaka innerytan. `lines.merge_symbols` slår ihop två strängar med samma tre regler som ramen.

Av 9 216 bilder matchar 9 192 Ratatuis buffert och ordnade celldiff direkt. I 24 fall försöker Ratatui skriva i kolumn 65 535 när en bred titel når den högra koordinatgränsen. Provet kräver just det indexfelet och ritar om samma klippta ruta med en skyddskolumn. Alla 45 synliga celler och diffen stämmer även där. De 414 720 celljämförelserna täcker sexton kombinationer av kanter, tre sammanslagningsregler, egna och inbyggda symboler samt titlar ovanpå färger, breda tecken och diffregler. Dessutom matchar 1 323 fristående sammanslagningar av strängar. [Dunderprov](../tests/border_cells.ddr) · [Ratatui-prov](../benchmarks/reference/examples/border_cells.rs)

```sh
mkdir -p build
dunder build --locked --offline tests/border_cells.ddr -o build/borders
cargo build --offline --locked --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example border_cells --target-dir build/rust
build/borders | build/rust/release/examples/border_cells
build/borders --merge | build/rust/release/examples/border_cells --merge
```

## Symboler i staplar och mätare

`Sparkline.bar_set` och `BarChart.bar_set` tar nio strängar från tom till full stapel. `Sparkline.missing_text` anger symbolen för ett saknat mätvärde. `LineGauge.set_symbols` väljer fylld och tom symbol. När en stränguppsättning är vald används den för varje cell. En etikett som börjar i andra kolumnen av ett brett tecken behåller tecknet i kolumnen till vänster.

Referensprovet jämför 1 080 bildrutor med förifylld text, stilar och diffregler. Alla 70 200 råceller och ordningen på celldiffarna stämmer med Ratatui, även efter sex överlagringar. Provet innehåller bland annat tomma, breda och sammansatta symboler i sista kolumnen, saknade mätvärden, vänstervänd sparkline samt vågräta och lodräta staplar. [Dunderprov](../tests/meter_cells.ddr) · [Ratatui-prov](../benchmarks/reference/examples/meter_cells.rs) · [Interaktivt terminalprov](../tests/meter_terminal.ddr)

```sh
dunder build --locked --offline tests/meter_cells.ddr -o build/meter-cells
cargo build --offline --locked --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example meter_cells --target-dir build/rust
build/meter-cells | build/rust/release/examples/meter_cells
```

## Bredd vid delade Unicode-sekvenser

För varje indatasträng jämför provet hela strängen och båda delarna vid varje Unicode-skalärgräns. Samlingen innehåller 5 244 emoji, 742 presentationsvarianter, 2 175 ligaturfall och tio korta kontrollfall. Alla 48 059 kolumnbredder och spannbredder matchar referensen. För de 13 delsträngar som innehåller kontrolltecken verifieras dessutom att `std.terminal.columns` ger `.None`. [Dunderprov](../tests/standard_columns.ddr) · [Ratatui-prov](../benchmarks/reference/examples/standard_columns.rs)

```sh
dunder run --locked --offline tools/ligatures.ddr > build/ligatures.txt
dunder build --locked --offline tests/standard_columns.ddr -o build/columns
cargo build --offline --locked --release --manifest-path benchmarks/reference/Cargo.toml --example standard_columns --target-dir build/rust
set -- tests/data/emoji-18.txt tests/data/emoji-variation-18.txt build/ligatures.txt
build/columns "$@" | build/rust/release/examples/standard_columns "$@"
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

`Frame.fill_symbol` tar även ett grafem med flera Unicode-skalärer. Stapeldiagrammet delar en färgsatt värdeetikett vid en hel grafemgräns och använder terminalkolumner för dess bredd.

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

## Skuggor

Provet lägger nio skuggeffekter över redan ritade celler, direkt och via `Block`. Buffertarna innehåller breda tecken, emoji, individuella diffregler och understrykningsfärg. I 1 890 bilder jämförs varje cells symbol, tre färger, attribut och diffregel samt ordningen på ändringarna. För egna effekter kontrolleras när de körs och vilka rektanglar de får. [Dunderprov](../tests/shadow_cells.ddr) · [Ratatui-prov](../benchmarks/reference/examples/shadow_cells.rs) · [Bildtid och reproduktion](../benchmarks/shadows.md)

## Layout i 0.7.0

Provet jämför alla paneler och mellanrum, även tomma ytor och överlappande grannar. De 14 096 grundfallen omfattar en kombinationstabell och 2 000 deterministiskt genererade layouter. Ytterligare 980 prov når bland annat 65 535 kolumner, stora fyllnadsvikter, nollvikter, högsta möjliga kvottäljare och mycket stora mellanrum. Rektangelprovet jämför marginaler, förflyttning, storleksändring, snitt, union, inpassning, positioner och rad- och kolumnåtkomst i 2 460 fall. Av dessa ligger 1 260 vid koordinatgränsen eller använder stora förflyttningar och marginaler. Mätvyn jämför tecken och stilar i samtliga 1 100 bilder. [Dunderprov](../tests/layout_reference.ddr) · [Rustreferens](../benchmarks/reference/examples/layout.rs) · [Mätningar](../benchmarks/layout.md)

När flera placeringar uppfyller motstridiga regler lika väl kan även två Ratatui-processer välja olika rektanglar. För varje avvikelse räknar jämförelseprogrammet ut det lägsta möjliga viktade felet med exakta rationella tal. Det löser sedan samma problem två gånger med båda programmens avrundade cellgränser som extra villkor. Båda måste nå samma minimum och alla mellanrum måste kunna återskapas från kanterna. [Ratatuis beskrivning av motstridiga villkor](https://docs.rs/ratatui/0.30.2/ratatui/layout/struct.Layout.html) · [Kontroll med exakt aritmetik](../benchmarks/reference/src/layout_optimum.rs)

På Linux x64 returnerade Ratatui en gång flera överlappande paneler för nio regler i `SpaceAround` med noll mellanrum och bredd 65 535. Två paneler upptog hela ytan. Det sparade fallet kontrollerar att Dunders uppdelning når det exakta optimumet och att referensens uppdelning faller utanför. Protokollet räknar sådana överlappande referensutfall separat. Om Dunders uppdelning missar optimum underkänns provet. [Ursprunglig körning med båda uppdelningarna](https://github.com/Mik-pe/dunder-tui/actions/runs/45334545608/job/138243420132) · [Regressionsprov med de sparade rektanglarna](../benchmarks/reference/examples/layout.rs)

Förflyttning, storleksändring och marginaler håller rektangelns högra och nedre kant inom 65 535. En rektangelliteral med negativt origo kan användas för klippning. Till och med 0.20.1 tog `Min(65535)` samtliga 100 kolumner bredvid `Length(7)`. Från 0.20.2 delar biblioteken upp samma yta i 93 och 7 kolumner.

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

## Emojipresentation i 0.18.1

😶‍🌫️ med U+FE0F och 😶‍🌫 utan väljaren består båda av ett grafem. I Ratatui 0.30.2 tar den första två terminalkolumner och den andra tre. I 0.18.0 gav 1 950 av de 5 244 sekvenserna i Unicodes emojitest annan bredd eller placering än referensen. I 0.18.1 matchar alla. Det gäller även de 742 registrerade paren av grundtecken och väljare för text- eller emojipresentation. [Unicodes emojitest](https://www.unicode.org/Public/18.0.0/emoji/emoji-test.txt) · [Presentationsvarianter](https://www.unicode.org/Public/18.0.0/ucd/emoji/emoji-variation-sequences.txt) · [Unicode-licens](../src/UNICODE-LICENSE.txt)

För varje post jämför provet antalet grafem, symbolens cellbredd, ett textspanns bredd och åtta ritade rader. Fyra rader ritar ett stiliserat spann följt av ett utropstecken. Fyra rader skriver direkt till bufferten med en bokstav före samma text. Bredderna 1, 2, 3 och 10 provar både klippning och platsen för nästa tecken. Tecken, position och stil jämförs över hela bilden, inklusive cellerna utanför ritytan. Ett separat prov läser de faktiska markörkommandona från en pseudoterminal när bara bokstaven efter en emoji ändras. [Dunderprov](../tests/text_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/text.rs) · [Terminalprov](../tests/posix_terminal.ddr) · [Mätning av loggvyn](../benchmarks/emoji.md)

Båda datafilerna under `tests/data` innehåller första fältet från varje datarad i respektive Unicode-fil, i originalets ordning. Proven går att upprepa med filerna i repot.

```sh
mkdir -p build
dunder build --locked --offline tests/text_reference.ddr -o build/text
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --example text --target-dir build/rust
for corpus in tests/data/emoji-18.txt tests/data/emoji-variation-18.txt; do
    build/text --emoji "$corpus" > build/emoji-cells
    build/rust/release/examples/text --emoji "$corpus" < build/emoji-cells
done
dunder run --locked --offline tests/posix_terminal.ddr
```

## Textbredd och ligaturer i 0.18.2

Ett spann med arabiska `لا` mäter en kolumn. Om bokstäverna läggs i varsitt spann blir radens bredd två. `Span.width()` och `GlyphRun.text_width()` mäter texten som helhet. `GlyphRun.width()` summerar bredden hos de grafem som ritas. En efterföljande etikett, radens justering och klippning kan därför bero på var en stilgräns läggs.

I 0.18.1 avvek 1 027 av 2 175 fall från Ratatui 0.30.2. I 0.18.2 matchar samtliga. Provet omfattar arabiska, hebreiska, buginesiska, khmer, lisu, fornturkiska, tifinagh och kirat rai, samt kontrolltecken och japanska halvbreddsmarkeringar. Det läser varje buffertcell, även under ett brett grafem. Varje fall ritas vid fyra klippbredder med fristående spann, buffertskrivning och tre radjusteringar. På Unix kontrollerar ett separat prov att en ändring i spannet efter `لا` skickar markören till rätt kolumn i pseudoterminalen. [Dunderprov](../tests/text_reference.ddr) · [Ratatui-prov](../benchmarks/reference/examples/text.rs) · [Terminalprov](../tests/posix_terminal.ddr)

```sh
mkdir -p build
dunder run --locked --offline tools/ligatures.ddr > build/ligatures.txt
dunder build --locked --offline tests/text_reference.ddr -o build/text
cargo build --release --locked --manifest-path benchmarks/reference/Cargo.toml --example text --target-dir build/rust
build/text --ligatures build/ligatures.txt > build/ligature-cells
build/rust/release/examples/text --ligatures build/ligatures.txt < build/ligature-cells
```

## Markören när en terminal släpps i 0.18.3

En terminal som döljer markören återställer den när den lämnar sitt scope. De 21 förloppen omfattar bland annat en tidig retur, ett ägarbyte, överskrivning av en terminal samt ett skrivfel efter en lyckad döljning. Provet jämför backendanropens ordning och fel på standardfel med Ratatui 0.30.2. I 0.18.2 avvek tolv förlopp. I 0.18.3 matchar alla. [Dunderprov](../tests/cursor_lifetime.ddr) · [Ratatui-prov](../benchmarks/reference/examples/cursor_lifetime.rs)

Unixprovet läser döljning och återställning i pseudoterminalens byteföljd. På Windows sätter provet en markörstorlek på 47 procent, döljer markören och läser tillbaka både storlek och synlighet i konsolen före och efter att terminalen släpps. Båda x64-systemen bygger proven med Dunder 0.11.2 och jämför dessutom 2 175 Unicode-textfall och 1 100 färdigritade bilder med Rustreferensen. [Pseudoterminal](../tests/posix_terminal.ddr) · [Win32-konsol](../tests/windows_console.ddr) · [Nativ körning](https://github.com/Mik-pe/dunder-tui/actions/runs/45302071651)

## Cellvis fyllning och rensning i 0.19.0

`widgets.Fill` skriver sin sträng till varje enskild cell, inklusive celler bakom breda symboler och cellen i sista kolumnen. Befintliga diffregler finns kvar och stilen kombineras med cellens ursprungliga stil. `widgets.Clear` återställer även diffregeln. Båda klipper mot buffertens koordinater och kan skickas till `frame.render_widget`. För geometrisk upprepning av ett helt grafem finns `frame.fill_symbol` kvar.

Provet jämför 1 512 buffertar och ordningen på deras ändrade celler med Ratatui. Det täcker 14 symboler, bland annat tom sträng, arabiska ligaturer, kombinerande tecken, två ASCII-tecken och sammansatta emoji. Det läser symbol, färg, attribut och diffregel på varje position, även bakom en bred symbol. Unixprovet tolkar terminalströmmen och Win32-provet läser tillbaka fem tecken från konsolen efter överlagrad fyllning och rensning. [Celljämförelse](../tests/cell_widgets.ddr) · [Referens](../benchmarks/reference/examples/cell_widgets.rs) · [Nativ plattformsverifiering](https://github.com/Mik-pe/dunder-tui/actions/runs/45308458992)

Ratatui 0.30.2 kan gå utanför bufferten när en bakgrundsfärgad bred symbol i sista cellen ersätts med en smal och diffiteratorn avancerar igen. Kör Rustprovet med `--last-cell` för ett exempel med en enda cell. Dunder lämnar den sista ändringen och avslutar itereringen. Celljämförelsen kontrollerar referensen till och med sista giltiga positionen. [Regressionsprov](../tests/suite.ddr) · [Bildtider och terminalbyte](../benchmarks/cell-widgets.md)

## Redigerbar text i 0.20.0

Provet bygger först en tom text, lägger till spann och rader, kortar den bredaste raden och ändrar stilar på alla tre nivåer. Sedan jämförs konstruktion av spann, rader och dokument från åtta indata, bland annat tom text, blandade radslut, en arabisk ligatur och en flaggemoji. Sammanlagt matchar 360 cellbilder och 112 rader med bredder, antal element, källtext och formaterad text Ratatui 0.30.2. Varje bild omfattar också kanten utanför ritytan. [Dunderprov](../tests/text_edits.ddr) · [Ratatui-prov](../benchmarks/reference/examples/text_edits.rs)

```sh
mkdir -p build
dunder build --locked --offline tests/text_edits.ddr -o build/text-edits
build/text-edits >build/dunder-edits
cargo run --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml --example text_edits >build/ratatui-edits
diff -u build/ratatui-edits build/dunder-edits
```

## Trånga layouter och riktningsbyte

1 284 ytterligare fall jämför varje panel och mellanrum när 12 eller 24 regler konkurrerar om 17, 79 eller 191 celler. De omfattar båda axlarna, sju flexlägen, sex enskilda regeltyper och blandningar av dem. Fallen vid 9–12 paneler provar precis när `SpaceAround` börjar krympa sina yttre mellanrum. Med tio paneler om åtta celler på 100 kolumner blir ytteravståndet två. Med elva på 110 kolumner blir det ett. [Dunderprov](../tests/layout_reference.ddr) · [Ratatui och kontroll av alternativa lösningar](../benchmarks/reference/examples/layout.rs)

På macOS ARM64 blev 1 248 uppdelningar identiska och 36 andra uppdelningar nådde samma exakta optimum. På Linux ARM64 blev motsvarande antal 1 245 och 39. Referensprogrammet fick starta om efter 17 tidsgränser och ett lösarfel på macOS samt 23 tidsgränser och två lösarfel på Linux. Alla 1 284 fall fick ett svar på båda systemen.

För lika krav kan panelerna beräknas direkt från den tillgängliga bredden. I `SpaceAround` är ett inre mellanrum normalt dubbelt så brett som ett yttre. När alla panelers sammanlagda kostnad för en förlorad cell överstiger kostnaden för att göra det yttre mellanrummet mindre krymper mellanrummen tills panelerna ryms eller de inre når den begärda bredden. Den vanliga lösaren hanterar återstående kombinationer. [Implementation](../src/layout.ddr) · [Bildtid för tolv frekvensband](../benchmarks/bands.md)

Ratatui 0.30.2 gav i en körning 19 tomma paneler för 24 stycken `Length(6)` på en höjd av 79 med `SpaceEvenly` och avstånd två. Det finns 29 rader kvar till panelerna när mellanrummen har fått sina 50. Provet bevarar det observerade utfallet och verifierar med rationell aritmetik att fördelningen med en eller två rader per panel når optimum. Det underkänner det omvända fallet. Om Ratatui ger den observerade typen av fördelning, rapporterar `ObjectiveUnbounded` eller stannar före sitt svar körs det större gränsfallet i en egen referensprocess med högst fyra försök. Varje godkänt fall kräver en uppdelning från Ratatui. [Sparade panelhöjder och kontrollen i båda riktningar](../benchmarks/reference/examples/layout.rs)

Alla sex konstruktorerna för listor med regler, deras textrepresentation och standardregeln `Percentage(100)` provas separat. Dunder och Ratatuis publika, utfasade `Constraint::apply` ger samma 16-bitarsvärde för 59 regler vid samtliga 65 536 längder. Det omfattar nollnämnare, tal större än 2²⁴ och procentuell avrundning med enkel precision. Ytterligare 36 uppdelningar följer en layout genom riktningsbyten och återbesök på samma kvadratiska yta. På macOS ARM64, Linux ARM64, översatt Linux x86-64 och Win64 under Wine kontrollerades även de ritade frekvensbanden. Tolv bokstäver byter från kolumner till rader i pseudoterminalen och i Win32-konsolbufferten. [Regelprovet](../tests/layout_reference.ddr) · [Pseudoterminal](../tests/layout_json_terminal.ddr) · [Win32](../tests/windows_console.ddr)

På macOS ARM64 gav även Dunder 0.12.1 från den lokala taggen `57e9c4d` samma 1 284 uppdelningar, alla regelvärden och samtliga 1 100 mixerbilder. Bibliotekets 100 tester, riktningsbytet i terminalen och ett fristående konsumentpaket kördes offline. Tiderna i mixerns mätning gäller 0.12.0.

```sh
mkdir -p build
dunder build --locked --offline tests/layout_reference.ddr -o build/layout
cargo build --release --locked --offline --manifest-path benchmarks/reference/Cargo.toml \
  --features layout-cache --example layout --target-dir build/reference
ref=build/reference/release/examples/layout
build/layout --many-panels | "$ref" --many-panels
build/layout --rules >build/dunder-rules
"$ref" --rules >build/reference-rules
cmp build/dunder-rules build/reference-rules
dunder run --locked --offline tests/layout_json_terminal.ddr
```

## Minsta storlek över 32 767 i 0.20.2

Ratatui 0.30.2 tolkar `Min` som ett signerat 16-bitarstal när lösaren sätter en undre gräns. Från 32 768 är den gränsen negativ och uppfylld redan vid bredd noll. I flexläget `Legacy` använder Ratatui dessutom det ursprungliga osignerade värdet som önskad storlek med lägre prioritet. Dunder följer båda reglerna. [Ratatuis lösare](https://github.com/ratatui/ratatui/blob/ratatui-v0.30.2/ratatui-core/src/layout/layout.rs) · [Dunders lösare](../src/layout.ddr)

Provet ändrar reglerna i samma layout och fortsätter använda fyra tidigare ritade storlekar. Det jämför varje panel och mellanrum för sex gränsvärden, alla sju flexlägen, båda riktningarna och upp till nio paneler. I 0.20.1 låg 3 768 av 6 048 layouter utanför referensens exakta optimum. I 0.20.2 nådde alla optimum. På macOS gav en körning 5 950 identiska uppdelningar och 98 andra verifierat likvärdiga uppdelningar. [Dunderprov](../tests/layout_reference.ddr) · [Oberoende kontroll med rationella tal](../benchmarks/reference/src/layout_optimum.rs)

```sh
mkdir -p build
dunder build --locked --offline tests/layout_reference.ddr -o build/layout
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features layout-cache --example layout --target-dir build/rust
build/layout --minimums | build/rust/release/examples/layout --minimums
```

Vid upprepning kunde Ratatuis lösare stanna i `kasuari::Solver::optimize`. Därför kör jämförelseprogrammet högst 64 gränsfall i varje referensprocess. Processen lämnar ett resultat direkt efter varje avslutad jämförelse. Om en grupp tar över fem sekunder skrivs det första obesvarade fallet ut och körs igen i en ny process. Två stopp vid samma fall underkänner provet. I tio körningar på macOS ARM64 fick alla 60 480 jämförelser ett svar. Fall 2 429 behövde en ny referensprocess i en av körningarna.

## De tio animationerna i releasepaketen

Varje scen ritas vid 0 och 1,75 sekunder på en canvas med 72 × 40 pixlar. Galleritestet läser de 1 440 cellerna i den andra bilden via `Frame`, kontrollerar halvblockets tecken och båda RGB-färgerna och jämför celldiffens koordinater i ritordning med de pixlar som ändrades. [Galleritest](../examples/motion/tests.ddr)

Terminalprovet startar den nedladdade `motion`-appen, läser scennamnen från skärmen, pausar, bläddrar genom alla tio scener och återupptar animationen. Vid avslut kontrolleras terminalens återställning. 0.20.2-paketen klarade provet i pseudoterminal på macOS ARM64 och Linux ARM64/x64 samt i Win32-konsol på Windows x64. För att prova en utgåva lokalt, ange sökvägen till den uppackade `motion`-binären som första argument till terminalprovet på respektive plattform. [Unixprov](../tests/motion_terminal.ddr) · [Win32-prov](../tests/windows_console.ddr) · [Körning på x64](https://github.com/Mik-pe/dunder-tui/actions/runs/45318968188)

```sh
(cd examples/motion && dunder test --locked --offline)
mkdir -p build
dunder build --locked --offline tests/motion_terminal.ddr -o build/motion-test
build/motion-test /sökväg/till/motion
```

## Sammanslagna dokument och stiliserade grafem i 0.21.0

`Text.append` tar över rader och spann från nästa dokument. Det första dokumentet bestämmer övergripande stil och justering. Efter redigering, sammanslagning och ytterligare ändringar jämförs text, terminalbredd och 456 renderade bilder med Ratatui. 96 av bilderna prövar själva sammanslagningen. [Dunderprov](../tests/text_edits.ddr) · [Ratatuis dokument och bildbuffert](../benchmarks/reference/examples/text_edits.rs)

För `Span` och `Line` ger både `styled_graphemes` och `visit_graphemes` lånade symboler. Varje spann har egna grafemgränser och egen stil, även när det börjar med ett kombinerande märke. 6 920 fall omfattar Unicode 17:s gränsprov, Unicode 18:s emoji och presentationsvarianter samt styrtecken. Spannen delas på olika skalärgränser. 75 792 avläsningar jämförs med Ratatuis publika iteratorer. Varje jämförelse kontrollerar symbolens UTF-8-byte, tre färger och påslagna och borttagna attribut. För iteratorerna jämförs också behandlingen av blanksteg. [Dunderprov](../tests/styled_graphemes.ddr) · [Ratatui-prov](../benchmarks/reference/examples/styled_graphemes.rs)

I Dunder 0.11.2 lånar ett returnerat element iteratorn. Förbruka elementet innan nästa anrop till `next`. Samma begränsning kan återskapas med enbart `std.text.graphemes`. [Minimalt program](../tests/dogfood/compiler/retained_graphemes.ddr)

```sh
mkdir -p build
dunder build --locked --offline tests/styled_graphemes.ddr -o build/graphemes
cargo build --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example styled_graphemes --target-dir build/rust
build/graphemes tests/data/emoji-18.txt tests/data/emoji-variation-18.txt | build/rust/release/examples/styled_graphemes
```

## Logotyp och maskot i 0.22.0

`tui.artwork` ritar logotypen i 15 × 2 eller 27 × 2 celler och maskoten i 32 × 16 celler. I 280 bilder jämförs 87 360 celler och varje ordnad celldiff med Ratatui 0.30.2. Provet täcker bland annat halvblock med genomskinlig bakgrund, byte av ögonfärg, förskjutet origo och klippning genom breda tecken. Befintliga attribut, understrykningsfärger och diffregler ingår i jämförelsen. [Dunderprovet](../tests/artwork.ddr) · [Referensprovet](../benchmarks/reference/examples/artwork.rs)

```sh
dunder build --locked --offline tests/artwork.ddr -o build/artwork
build/artwork | cargo run --quiet --locked --offline --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example artwork
```

Affischen kan också köras i en riktig terminal. Pseudoterminalprovet läser logotypens tecken, båda ögonfärgernas styrsekvenser och återställningen. Win32-provet läser de faktiska tecknen och bakgrundsfärgen ur den aktiva konsolbufferten. [Pseudoterminal](../tests/artwork_terminal.ddr) · [Windowskonsol](../tests/windows_console.ddr)

Affischen och den skrollande programlistan jämförs även i 1 100 kompletta terminalbilder. Prestandaprovet mäter samma fem arbetslaster på alla fyra målplattformar. [Mätresultat och körinstruktioner](../benchmarks/artwork.md) · [Nativa prov av de publicerade x64-paketen](https://github.com/Mik-pe/dunder-tui/actions/runs/45327111173)

## Redigering av rader och spann i 0.23.0

Elva redigeringssteg tar bort, flyttar och byter ut rader och spann i samma dokument. Efter varje steg jämförs text, bredd, justering och tolv bilder av dokumentet. Ytterligare 1 188 styckebilder jämförs med direkt och förberedd rendering över sex bredder, tre skrollpositioner och tre radbrytningslägen. Tillsammans med de tidigare redigeringsproven ger det 1 776 bilder. [Dunderprov](../tests/text_edits.ddr) · [Ratatui-prov](../benchmarks/reference/examples/text_edits.rs)

Med `.wide_wrap(.Overhang)` kan ett brett tecken börja i sista kolumnen, som i Ratatui. I styckeprovet matchar alla fysiska celler i de 4 596 fall som Ratatui kan rita på den angivna ytan. För de tolv fall där Ratatui avbryter matchar Dunder den synliga delen av en bredare referensbuffert. Standardvalet `.NextLine` behåller radbrytningen från 0.4.0. Ett separat prov ritar upp till fyra lager över förifyllda celler. Det jämför 1 296 bilder, 31 536 råa celler och samtliga ordnade celldiffar, inklusive tidigare tecken under breda symboler, understrykningsfärg och cellernas diffregler. [Styckeprov](../tests/text_reference.ddr) · [Lagerprov](../tests/paragraph_cells.ddr) · [Ratatuis celljämförelse](../benchmarks/reference/examples/paragraph_cells.rs)

Den publicerade körlistan testades tillsammans med galleriet och maskoten i pseudoterminal på macOS ARM64 och Linux ARM64/x64 och i Win32-konsol på Windows x64. Testet flyttar en rad, markerar den som klar, byter pass två gånger och läser tillbaka ordning och markering från skärmen. [Unixprov](../tests/rundown_terminal.ddr) · [x64-körning](https://github.com/Mik-pe/dunder-tui/actions/runs/45330011065) · [Körlistans bildtider på fyra plattformar](../benchmarks/collections.md)
