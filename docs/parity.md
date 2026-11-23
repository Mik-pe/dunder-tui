# Täckning mot Ratatui 0.30.2

Referensen är taggen `ratatui-v0.30.2`, inklusive de stabila tillvalen. Tabellen följer de publika områdena i den versionen. En genomförd jämförelse anges med sitt konkreta prov. [Referenskod](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2) · [Publikt API](https://docs.rs/ratatui/0.30.2/ratatui/) · [Funktionsflaggor](https://docs.rs/crate/ratatui/0.30.2/features)

| Område | Finns i Dunder | Återstår att leverera eller verifiera |
| --- | --- | --- |
| Buffer och Cell | Återanvända bildbuffertar, breda tecken, celldiff, färgutdata och avläsning av rader och cellstilar. | Fristående buffertoperationer med origo, ändrad storlek och sammanslagning. Cellernas regler för att hoppa över eller alltid rita och sammanslagning av linjesymboler. |
| Stil och färg | Nio attribut, påslag och borttagning, ärvda stilar, återställning, 16 namngivna färger, 256 index, RGB och separat understrykningsfärg. Färger kan läsas från strängar och byggas från HSL eller HSLuv. | Material- och Tailwind-paletter, beständiga dataformat och full täckning för konvertering mellan stilrepresentationer. |
| Text | Span, Line, Text, Paragraph, maskering per skalär, justering, skrollning, ord- och radbrytning och förberedd layout. Grafemgränser enligt UAX #29. | Strängbredder för skriftspecifika ligaturer över grafemgränser, full täckning för sammansatta emoji och alla operationer på fristående texttyper. |
| Layout | Rektanglar, marginal runt en yta, delning av rader och kolumner. | Fler rektangeloperationer, riktning, marginaler per axel, längd-/kvot-/min-/max-/procent-/fyllnadsregler, flex, mellanrum och layoutcache. |
| Block, listor och tabeller | En ram med titel och en lista som följer markeringen. | Ramtyper, symboler, titlar, kantkombinationer, fyllning och skuggor. Flerradiga listposter, listtillstånd, tabeller, rader, celler och tillstånd för rad och kolumn. |
| Övriga widgets | Paragraph och RGB-canvas med punkter, linjer och halvblock. | Clear, Fill, Tabs, Gauge, LineGauge, Scrollbar, Sparkline, BarChart, Chart, kalender med datumstilar, logotyp och maskot. Canvas med koordinatgränser, lager, markörer, former, kartor och etiketter. |
| Egna widgets och tester | Funktioner ritar på en Frame. Tillstånd kan uppdateras vid ritning med `run_stateful`. En Frame kan testas utan terminal. | Gemensamma gränssnitt för widgets med eget tillstånd och utbytbara backends, testbackend med skrollhistorik och markör. |
| Terminal och backend | Alternativ skärm, tangentbord, storleksändringar, återställning, ANSI och Win32. | Fast och infogad vyport, innehåll ovanför vyn, markörstyrning från Frame, regionrensning och regionskrollning, fönstrets pixelmått, livscykel- och felprov. |
| Stabil Rust-integration | Jämförelseprogrammen använder den låsta Rustreferensen. Dunder-appar byggs offline. | Kontrollera Dunder-motsvarigheter till bekvämlighetsmakron, serialisering, cachning och backendval där dessa påverkar appens beteende. |
| Effektivitet och plattformar | Repeterbara tid-, minnes- och utdataprov på macOS. Pseudoterminalprov på macOS och Linux ARM64/x86-64. Win32-prov under Wine. | Nativ Windows x64, fler representativa arbetslaster med layout och widgets, färre allokeringar vid konstruktion av färgvärden per rad och jämförbara mätningar på fler värdar. |

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
