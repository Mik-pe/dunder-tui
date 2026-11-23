# dunder-tui

Bygg interaktiva terminalappar i Dunder för macOS, Linux och Windows. Skriv en funktion som ritar appens tillstånd och en som tar emot tangenter. Biblioteket sköter inmatning, storleksändringar och återställning av terminalen. Vid omritning skickas bara ändrade celler.

## Prova

Vill du se animationerna direkt? Packa upp paketet för din plattform och kör `./motion` eller `.\motion.exe` i en terminal. Samma paket innehåller kustkartan `kustlinjer`, signalvisualiseraren `signal`, boklistan `bokhylla` och tidtabellen `avganger`. [Ladda ner för macOS, Linux eller Windows](https://github.com/Mik-pe/dunder-tui/releases/latest)

Med Dunder 0.11.1 eller senare installerat kan du starta exemplen med deras namn. [Installera Dunder](https://github.com/Mik-pe/dunder#installera-och-kör)

```sh
git clone https://github.com/Mik-pe/dunder-tui.git
cd dunder-tui
dunder run --example counter
```

Tryck ↑ eller ↓ för att ändra talet och `q` för att avsluta. Kör `dunder run --example bokhylla` för att bläddra bland flerradiga bokposter med piltangenterna. Gör fönstret smalare så får boklistan hela bredden.

Lista alla exempel med `dunder --list-examples`. På Dunder 0.10.0 och 0.11.0 kan du ange filen direkt, exempelvis `dunder run examples/motion/main.ddr`.

## Animationer

Kör galleriet med tio animationer. Välj en scen med ← och → eller siffrorna 1–9 och 0. Mellanslag pausar och återupptar. Avsluta med `q`.

```sh
dunder run --example motion
```

![Tio animationer från galleriet](docs/motion.webp)

Starta en viss scen med exempelvis `dunder run --example motion -- koi`. Lista scenerna med `dunder run --example motion -- --list`.

| Tangent | Scen | Tangent | Scen |
| --- | --- | --- | --- |
| 1 | [Polarnatt](examples/motion/aurora.ddr) | 6 | [Efterglöd](examples/motion/afterglow.ddr) |
| 2 | [Prisma](examples/motion/prism.ddr) | 7 | [Medusa](examples/motion/medusa.ddr) |
| 3 | [Koidammen](examples/motion/koi.ddr) | 8 | [Tesserakt](examples/motion/tesseract.ddr) |
| 4 | [Händelsehorisont](examples/motion/horizon.ddr) | 9 | [Glödbädd](examples/motion/ember.ddr) |
| 5 | [Harmonograf](examples/motion/plotter.ddr) | 0 | [Drivande atlas](examples/motion/atlas.ddr) |

För en egen animation tar `tui.animate(&mut state, 30, draw, update, tick)` hand om bildklockan. `tick` får förfluten tid i millisekunder och returnerar `true` när något behöver ritas. En sen bild går direkt till aktuell tid. Tangentbordet fungerar under tiden. Med `import tui.canvas` kan du rita pixlar och linjer i RGB. `Canvas.render` visar två vertikala pixlar per teckencell. Galleriet använder 24-bitarsfärg i moderna terminaler och närmaste konsolfärg i äldre Windowskonsoler.

### Signal

Växla mellan vågform och frekvensspektrum med Tab. Piltangenterna ändrar frekvens och amplitud. Den turkosa mätaren visar signalens effektivvärde. Varje stapel i spektrumet beräknas från samma 128 sampel som vågformen. Mellanslag fryser bilden.

```sh
dunder run --example signal
```

Signalen syntetiseras i appen och kräver ingen ljudenhet. [Källkod](examples/signal/main.ddr)

### Kustlinjer

`dunder run --example kustlinjer` visar en världskarta och ett diagram över antalet kustpunkter per fem längdgrader. Piltangenterna flyttar meridianen, Tab växlar kartans upplösning och mellanslag pausar svepet. [Exemplets källkod](examples/kustlinjer/main.ddr)

![En orange meridian sveper över en turkos världskarta och markerar motsvarande intervall i diagrammet](docs/kustlinjer.webp)

## Skriv en egen app

Skapa ett projekt med `dunder init min-app`. Lägg till beroendet i projektets `dunder.toml`.

```toml
[dependencies]
tui = { git = "https://github.com/Mik-pe/dunder-tui.git", tag = "v0.10.0", capabilities = ["stdio", "unsafe"] }
```

Spara följande i `src/main.ddr` och kör `dunder run` i projektet. Biblioteket använder de beviljade kapabiliteterna för att läsa tangentbordet och styra terminalen.

```dunder
capability stdio
import tui
import sys

fn draw(frame: &mut tui.Frame, count: &i32) {
    frame.label(frame.area(), f"Antal: {count}   ↑ öka   Esc avsluta")
}

fn update(count: &mut i32, key: tui.Key) -> bool {
    match key {
        .Up => { count += 1 },
        .Escape => { return false },
        _ => {},
    }
    true
}

fn main() -> i32 {
    mut count := 0
    match tui.run(&mut count, draw, update) {
        .Ok => 0,
        .Err(message) => { _ := sys.eprint(message); 1 },
    }
}
```

`draw` får en tom bild vid start, efter en tangent och när fönstret byter storlek. Returnera `false` från `update` för att lämna appen. Ctrl+C avslutar också. Om in- eller utmatningen är omdirigerad returnerar `run` ett fel.

## Markör och testterminal

Anropa `frame.set_cursor_position(x,y)` där markören ska synas. Det sista anropet i en bild bestämmer positionen. I nästa bild är markören dold tills ritfunktionen begär den igen. `frame.count()` börjar på noll och räknar visade bilder. I `bokhylla` öppnar **e** den valda bokens anteckning för redigering. Piltangenterna flyttar markören ett grafem i taget, Enter behåller texten i sessionen och Esc återgår till boken. [Redigeraren](examples/bokhylla/editor.ddr)

Samma ritfunktion går att prova utan en ansluten terminal.

```dunder
import tui
import tui.testing as testing

fn draw(frame: &mut tui.Frame, text: &mut string) {
    frame.label(frame.area(),text)
    frame.set_cursor_position(2,0)
}

fn main() {
    mut terminal := tui.Terminal<testing.TestBackend>::new(testing.TestBackend::new(8,2))
    mut text := "Hej"
    assert(terminal.draw(&mut text,draw).is_ok())
    assert(terminal.backend().buffer().line(0) == "Hej     ")
    assert(terminal.backend().cursor_visible())
    assert(terminal.backend().cursor_position() == (2,0))
}
```

Implementera `tui.Backend` med `size` och `present` för att ansluta en egen utmatning. `present` får aktuell och föregående bild. `frame.ansi_diff(&previous,&mut output)` skriver ändrade celler och markörkommandon till en bytebuffert. Med `try_draw` kan ritfunktionen returnera ett fel före utmatningen. Om backenden rapporterar ett skrivfel får den en hel bild nästa gång. `completed()` ger den senast lyckade bilden. Anropa `invalidate()` för att tvinga fram en hel omritning efter externa ändringar. [Backend med felprov](tests/suite.ddr) · [Testterminalens skrollhistorik](src/testing.ddr)

## Bygg en vy

| Anrop | Användning |
| --- | --- |
| `frame.area()` | Hela ritytan i terminalkolumner och rader. |
| `area.rows(n)`, `area.cols(n)` | Dela av de första `n` raderna eller kolumnerna. Båda delarna returneras och ryms i ytan. |
| `area.inset(n)` | Lämna en marginal innanför ytan. |
| `frame.panel(area, title, style)` | Rita en ram och få tillbaka innerytan för innehållet. |
| `frame.label(area, text)`, `frame.text(area, text, style)` | Visa text med radbrytningar. Långa rader klipps vid kanten. |
| `frame.list(area, &items, selected, highlight)` | Visa listan med markerad rad. När markeringen passerar nederkanten följer vyn med. |
| `tui.ink(.Cyan).bold()` | Välj textfärg. `.background(.Blue)` och `.inverted()` kan kombineras med färg och fetstil. |

En egen widget är en vanlig funktion som tar `&mut tui.Frame` och en `tui.Rect`. Använd `tui.ink(tui.rgb(0x82efc5))` för RGB eller `tui.ink(.Indexed(197))` för palettfärg. Testa den utan terminal med `tui.Frame::new(20, 4)` och läs en färdig rad med `frame.line(0)`. [Körbara exempel på layout och textklippning](tests/suite.ddr)

### Responsiv layout

En sidomeny kan ta 18 kolumner och lämna resten till innehållet. Spara layouten i appens tillstånd så att den kan återanvända beräkningen mellan bilder.

```dunder
import tui.layout as layout

mut columns := layout.Layout::horizontal([.Length(18), .Fill(1)]).spacing(1)
panes := columns.split(frame.area())
frame.label(panes[0], "Bibliotek")
frame.label(panes[1], "Mina böcker")
```

`vertical` delar på höjden. Använd `Percentage(40)`, `Ratio(1,3)`, `Min(8)` eller `Max(24)` för andra storleksregler. `flex(.Center)` centrerar en grupp och `margin(2,1)` lämnar två kolumner och en rad runt den. Negativt mellanrum låter paneler överlappa. För tät omritning ger `columns.arrange(area).regions()` en lånad vy med både paneler och mellanrum. [Bokhyllans responsiva vy](examples/bokhylla/main.ddr) · [Jämförelse med Ratatui](docs/parity.md#layout-i-070)

## Mätare, diagram och paneler

Med `import tui.widgets as ui` kan du rita en mätare i valfri del av en bild.

```dunder
mut level := ui.Gauge::percent(37)
level.fractional = true
level.bar_style = tui.plain().foreground_rgb(0x8dffdb).background_rgb(0x23354b)
level.render(&mut frame, area)
```

| Modul | Byggstenar |
| --- | --- |
| `tui.widgets` | `Gauge`, `LineGauge`, `Sparkline`, `Tabs` och `Scrollbar` med eget skrolltillstånd. |
| `tui.barchart` | Horisontella och vertikala staplar med grupper, etiketter och färger per stapel. |
| `tui.block` | Ramar med egna kanttecken, flera titlar, marginal innanför kanten och skuggor. Delade kanter kan slås ihop. |

`area.take_rows(n)` tar ut de första raderna och låter `area` fortsätta direkt under dem. `take_cols(n)` gör motsvarande för kolumner. Använd `frame.fill_symbol(area, "🌊", style)` för att upprepa hela grafem eller `frame.clear_area(area)` för att tömma ytan före en överlagring. [Exempel med flikar, paneler och diagram](examples/signal/main.ddr)

## Rita i egna koordinater

Med `tui.plot` kan du ge ritytan egna x- och y-gränser. Lägg flera former i ett lager med `point`, `line`, `filled_line`, `rectangle`, `circle` och `map`. Anropa `layer()` för att lägga nästa form ovanpå. Det finns nio markörtyper, bland annat braille, halvblock med två färger och egna tecken.

```dunder
import tui.plot as plot

mut view := plot.Plot::new([-10.0,10.0],[-5.0,5.0])
p := view.begin(&mut frame,area)
p.line(-12.0,-3.0,12.0,3.0,.Cyan)
p.set_marker(.HalfBlock)
p.circle(0.0,0.0,3.0,.Yellow)
view.finish(&mut frame)
```

Med `tui.chart` delar flera dataserier samma axlar. Varje serie kan ritas som punkter, linje, staplar eller fylld yta. För en interaktiv app kan du spara diagrammet i tillståndet och ändra `graph.datasets[0].data` eller axelgränserna före nästa ritning.

```dunder
import tui
import tui.chart as chart
import tui.text as rich

mut samples := chart.Dataset::new([(0.0,1.0),(1.0,3.0),(2.0,2.0)])
samples.graph_type = .Line
samples.marker = .Braille
samples.style = tui.ink(.Cyan)
mut graph := chart.Chart::new([move samples])
graph.x_axis = chart.Axis::new([0.0,2.0])
graph.y_axis = chart.Axis::new([0.0,4.0])
graph.y_axis.labels = [rich.Line::raw("0"),rich.Line::raw("4")]
graph.render(&mut frame,area)
```

`tui.animate_stateful` låter ritfunktionen ta `&mut S` så att diagram och ritytor kan återanvända sina buffertar mellan bilder. [Kustkartans båda vyer](examples/kustlinjer/main.ddr) · [Jämförelse med Ratatui](docs/parity.md#diagram-och-canvas-i-090)

## Listor med markering

Spara listan och dess skrolltillstånd i din app. En post kan innehålla flera rader, färgade textspann och breda tecken.

```dunder
import tui
import tui.list as lists
import tui.text as rich

mut books := lists.List::new([lists.Item::raw("Nattåget\nRoman"), lists.Item::raw("Molnatlas\nNatur")])
books.marker = rich.Line::raw("▸ ")
books.highlight = tui.plain().background_rgb(0x224451)
books.scroll_padding = 1
mut cursor := lists.ListState::new()
cursor.first()
```

Rita med `books.render_stateful(&mut frame, area, &mut cursor)`. Anropa `cursor.next()` och `cursor.previous()` från tangenthanteraren. När vyn krymper justerar listan `cursor.offset` så att hela den markerade posten syns om den ryms. `cursor.last()` hittar sista posten vid nästa ritning, även om innehållet har bytts med `books.set_items(...)`. En lista från nederkanten använder `books.direction = .BottomToTop`. [Bokhyllans källkod](examples/bokhylla/main.ddr) · [Mätning med 16 384 poster](benchmarks/lists.md)

## Tabeller

Kör `dunder run --example avganger` för en tidtabell med flerradiga avgångar. Piltangenterna byter rad eller kolumn. Home och End hoppar till första eller sista avgången. [Exemplets källkod](examples/avganger/main.ddr)

```dunder
import tui.table as table

mut departures := table.Table::new([table.Row::strings(["19:42","Malmö C"]),
    table.Row::strings(["20:08","Åre"])],[.Length(7),.Fill(1)])
departures.header = .Some(table.Row::strings(["Tid","Destination"]))
departures.cell_highlight = tui.plain().background_rgb(0x25384c).bold()
mut cursor := table.TableState::new()
cursor.select_cell(.Some((0,1)))
```

Spara tabell och markering i appens tillstånd. Rita med `departures.render_stateful(&mut frame, area, &mut cursor)`. En `Cell` kan innehålla rik text och spänna över flera kolumner. Ange `row.height` för flerradigt innehåll. Ändra en befintlig cell med `set_cell` och ersätt hela datamängden med `set_rows`. [Tabellprov](docs/parity.md#tabeller-i-080)

## Rik text och radbrytning

Med `import tui.text as rich` kan en rad blanda färger, attribut och breda tecken. Det här stycket kan sparas i appens tillstånd och ritas med `artikel.render(&mut frame, area)`.

```dunder
artikel := rich.Paragraph::new(rich.Text::new([
    rich.Line::raw("Nattjournal").align(.Center),
    rich.Line::new([
        rich.Span::styled("18.00  ", tui.ink(.LightCyan).bold()),
        rich.Span::raw("Koi simmar under bron. 🌙"),
    ]),
])).wrap(.Trim)
```

`.Trim` radbryter vid ord och tar bort inledande blanksteg. `.Preserve` behåller indrag och `.Clip` klipper långa rader. `set_scroll(rad, kolumn)` flyttar läspositionen. Horisontell skrollning gäller vänsterjusterad, klippt text. För en lång text som skrollas ofta kan `artikel.prepare(bredd)` beräkna radbrytningarna en gång. Den förberedda texten har `height()`, `set_top(rad)` och `resize(bredd)`.

Kör `dunder run examples/reader/main.ddr` för att prova en läsare. Piltangenterna skrollar, Home och End hoppar mellan början och slutet. Läsaren använder `tui.run_stateful` så att ritfunktionen kan anpassa skrollpositionen till fönstrets storlek. [Läsarens källkod](examples/reader/main.ddr)

## Stilar som kan kombineras

Lägg bakgrunden en gång och låt varje text välja sina attribut. En färg eller ett attribut som inte anges ärvs från cellen. `Style::reset()` återställer samtliga fält till terminalens grundstil.

```dunder
frame.apply_style(frame.area(), tui.plain().background(tui.rgb(0x1c162d)))
rubrik := tui.ink(.LightCyan).bold().underlined().underline_color(.Indexed(199))
frame.text(frame.area(), "Polarnatt", rubrik)
```

Använd `base.patch(overlay)` för att kombinera två stilar, `.without(.Bold)` för att ta bort ett attribut och `frame.style_at(x, y)` för att läsa resultatet i ett test. För färgteman finns `tui.Color::parse("#a875ff")`, `Color::from_hsl(h, s, l)` och `Color::from_hsluv(h, s, l)`. Understrykningsfärg, kursiv och blinkning visas när terminalen stöder dem.

För animerade färger tar `foreground_rgb(0x54dcbe)`, `background_rgb(...)` och `underline_index(199)` färgdata direkt. Det undviker temporära färgvärden i ritloopen.

I 0.3 används `tui.plain()` och stilmetoderna för att ersätta gamla `Style { ... }`-literaler. `.White` behåller färg 7, även kallad `.Gray`. `.BrightWhite` är färg 15. Ett palettindex kan skiljas från en namngiven färg i `style_at`.

## Plattformar och tester

| Mål | Terminal |
| --- | --- |
| `arm64-macos` | Interaktiv TTY med stöd för ANSI och alternativ skärm. |
| `arm64-linux`, `x86_64-linux` | Samma terminalgränssnitt. Appen använder Linux systemanrop direkt. |
| `x86_64-windows` | Win32-konsol med egen skärmbuffert och tangentbordshändelser via `ReadConsoleInputW`. |

Kör `dunder test examples/motion/tests.ddr` för alla tio animationerna och `dunder test` för layout, rullning, Unicode och tangentavkodning. På macOS och Linux öppnar `dunder run tests/posix_terminal.ddr` en pseudoterminal och kontrollerar bland annat omritning, storleksändring, Ctrl+C och återställning. På Windows öppnar `dunder run tests/windows_console.ddr` en separat konsol och kontrollerar inmatning, skärminnehåll, färg och återgång till föregående skärm.

I en pseudoterminal på en Mac med M5 skickade de tio scenerna 29,9–30,1 bilder per sekund vid 100 × 30 teckenceller och ett mål på 30. Galleriets scenbyte, paus och återställning av terminalen har provats på macOS och Linux. Windowsbinären har provats under Wine. Körning på en nativ Windowsvärd återstår. [Renderingsmätning mot Ratatui](benchmarks/README.md) · [Täckning och återstående arbete](docs/parity.md)

Korsbygg exempelvis en Windowsapp med `dunder build examples/counter/main.ddr --target x86_64-windows -o counter.exe`.

Text delas vid grafemgränserna i Unicode 17.0. För enskilda teckens cellbredd används Unicode 18.0 med Ratatuis terminalanpassningar. Emoji, kombinerande tecken och flaggor hålls ihop vid radbrytning och klippning. Terminalens typsnitt och Unicode-stöd påverkar hur de visas. [Unicode 17](https://www.unicode.org/Public/17.0.0/ucd/) · [Unicode 18](https://www.unicode.org/Public/18.0.0/ucd/) · [Licens för tabellerna](src/UNICODE-LICENSE.txt)

Biblioteket och exemplen kan byggas med Dunder 0.10.0. Pseudoterminalproven och proven för det nya standardbiblioteket använder Dunder 0.11.2 eller senare. [Reproducerade språk- och standardbiblioteksfynd](docs/dunder.md)
