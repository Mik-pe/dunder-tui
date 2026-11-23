# dunder-tui

Bygg interaktiva terminalappar i Dunder för macOS, Linux och Windows. Skriv en funktion som ritar appens tillstånd och en som tar emot tangenter. Biblioteket sköter inmatning, storleksändringar och återställning av terminalen. Vid omritning skickas bara ändrade celler.

## Prova

Med [Dunder](https://github.com/Mik-pe/dunder#installera-och-kör) 0.10 eller 0.11 installerat kan du köra räknaren direkt i en terminal.

```sh
git clone https://github.com/Mik-pe/dunder-tui.git
cd dunder-tui
dunder run examples/counter/main.ddr
```

Tryck ↑ eller ↓ för att ändra talet och `q` för att avsluta. Kör `dunder run examples/bokhylla/main.ddr` för ett exempel med två paneler och en rullande lista. Prova att göra fönstret lägre än listan.

## Animationer

Kör galleriet med tio animationer. Välj en scen med ← och → eller siffrorna 1–9 och 0. Mellanslag pausar och återupptar. Avsluta med `q`.

```sh
dunder run examples/motion/main.ddr
```

![Tio animationer från galleriet](docs/motion.webp)

Starta en viss scen med exempelvis `dunder run examples/motion/main.ddr -- koi`. Lista scenerna med `dunder run examples/motion/main.ddr -- --list`.

| Tangent | Scen | Tangent | Scen |
| --- | --- | --- | --- |
| 1 | [Polarnatt](examples/motion/aurora.ddr) | 6 | [Efterglöd](examples/motion/afterglow.ddr) |
| 2 | [Prisma](examples/motion/prism.ddr) | 7 | [Medusa](examples/motion/medusa.ddr) |
| 3 | [Koidammen](examples/motion/koi.ddr) | 8 | [Tesserakt](examples/motion/tesseract.ddr) |
| 4 | [Händelsehorisont](examples/motion/horizon.ddr) | 9 | [Glödbädd](examples/motion/ember.ddr) |
| 5 | [Harmonograf](examples/motion/plotter.ddr) | 0 | [Drivande atlas](examples/motion/atlas.ddr) |

För en egen animation tar `tui.animate(&mut state, 30, draw, update, tick)` hand om bildklockan. `tick` får förfluten tid i millisekunder och returnerar `true` när något behöver ritas. En sen bild går direkt till aktuell tid. Tangentbordet fungerar under tiden. Med `import tui.canvas` kan du rita pixlar och linjer i RGB. `Canvas.render` visar två vertikala pixlar per teckencell. Galleriet använder 24-bitarsfärg i moderna terminaler och närmaste konsolfärg i äldre Windowskonsoler.

## Skriv en egen app

Skapa ett projekt med `dunder init min-app`. Lägg till beroendet i projektets `dunder.toml`.

```toml
[dependencies]
tui = { git = "https://github.com/Mik-pe/dunder-tui.git", tag = "v0.3.0", capabilities = ["stdio", "unsafe"] }
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

Textbredden följer Unicode 17 för breda tecken och kombinerande märken. Vanliga emoji-sekvenser, hudtoner och flaggpar hålls ihop vid klippning. Terminalens typsnitt och Unicode-stöd avgör hur tecknen visas. [Unicode Character Database](https://www.unicode.org/Public/17.0.0/ucd/) · [Licens för breddtabellerna](src/UNICODE-LICENSE.txt)
