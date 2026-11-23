# Täckning mot Ratatui 0.30.2

Referensen är taggen `ratatui-v0.30.2`, inklusive de stabila tillvalen. Tabellen följer de publika områdena i den versionen. En genomförd jämförelse anges med sitt konkreta prov. [Referenskod](https://github.com/ratatui/ratatui/tree/ratatui-v0.30.2) · [Publikt API](https://docs.rs/ratatui/0.30.2/ratatui/) · [Funktionsflaggor](https://docs.rs/crate/ratatui/0.30.2/features)

| Område | Finns i Dunder | Återstår att leverera eller verifiera |
| --- | --- | --- |
| Buffer och Cell | Återanvända bildbuffertar, breda tecken, celldiff, färgutdata och avläsning av rader och cellstilar. | Fristående buffertoperationer med origo, ändrad storlek och sammanslagning. Cellernas regler för att hoppa över eller alltid rita och sammanslagning av linjesymboler. |
| Stil och färg | Nio attribut, påslag och borttagning, ärvda stilar, återställning, 16 namngivna färger, 256 index, RGB och separat understrykningsfärg. Färger kan läsas från strängar och byggas från HSL eller HSLuv. | Material- och Tailwind-paletter, beständiga dataformat och full täckning för konvertering mellan stilrepresentationer. |
| Text | Text med radslut, kolumnklippning och stöd för flera sorters sammansatta tecken. | Span, Line, Text, maskerad text, justering, full grafemsegmentering, ord- och radbrytning, textskrollning. |
| Layout | Rektanglar, marginal runt en yta, delning av rader och kolumner. | Fler rektangeloperationer, riktning, marginaler per axel, längd-/kvot-/min-/max-/procent-/fyllnadsregler, flex, mellanrum och layoutcache. |
| Block, listor och tabeller | En ram med titel och en lista som följer markeringen. | Ramtyper, symboler, titlar, kantkombinationer, fyllning och skuggor. Flerradiga listposter, listtillstånd, tabeller, rader, celler och tillstånd för rad och kolumn. |
| Övriga widgets | RGB-canvas med punkter, linjer och halvblock. | Paragraph, Clear, Fill, Tabs, Gauge, LineGauge, Scrollbar, Sparkline, BarChart, Chart, kalender med datumstilar, logotyp och maskot. Canvas med koordinatgränser, lager, markörer, former, kartor och etiketter. |
| Egna widgets och tester | Funktioner ritar på en Frame. Tillstånd kan hållas i appen. En Frame kan testas utan terminal. | Gemensamma gränssnitt för widgets med eget tillstånd och utbytbara backends, testbackend med skrollhistorik och markör. |
| Terminal och backend | Alternativ skärm, tangentbord, storleksändringar, återställning, ANSI och Win32. | Fast och infogad vyport, innehåll ovanför vyn, markörstyrning från Frame, regionrensning och regionskrollning, fönstrets pixelmått, livscykel- och felprov. |
| Stabil Rust-integration | Jämförelseprogrammen använder den låsta Rustreferensen. Dunder-appar byggs offline. | Kontrollera Dunder-motsvarigheter till bekvämlighetsmakron, serialisering, cachning och backendval där dessa påverkar appens beteende. |
| Effektivitet och plattformar | Repeterbara tid-, minnes- och utdataprov på macOS. Pseudoterminalprov på macOS och Linux ARM64/x86-64. Win32-prov under Wine. | Nativ Windows x64, fler representativa arbetslaster med Unicode, layout och widgets, färre allokeringar vid konstruktion av färgvärden per rad och jämförbara mätningar på fler värdar. |

Stilprovet genererar en bild med 1 024 celler via Dunders publika API, tolkar terminalströmmen och jämför varje attribut, färg och position med både Ratatuis buffert och Crossterms terminalström. Det täcker alla 512 kombinationer av de nio attributen. Ytterligare 512 färgtripplar konverteras med både HSL och HSLuv i båda programmen. [Dunderprovet](../tests/style_reference.ddr) · [Referens och terminalavkodare](../benchmarks/reference/examples/styles.rs)

```sh
mkdir -p build
dunder build --locked --offline tests/style_reference.ddr -o build/style-reference
build/style-reference | cargo run --quiet --locked --release --manifest-path benchmarks/reference/Cargo.toml --features style-oracle --example styles
```

Fler körbara prov täcker ändrade bildrader, delade SGR-återställningar, överlappande text, färgade understreck och terminalåterställning. [Bibliotekstester](../tests/suite.ddr) · [Unix-terminal](../tests/posix_terminal.ddr) · [Win32-konsol](../tests/windows_console.ddr) · [Prestandamätningar](../benchmarks/README.md)
