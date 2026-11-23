# Vad TUI-arbetet visar om Dunder

## Två kompilatorfynd från den utbytbara terminalbackenden

Två olika specialiseringar av samma generiska typ kan var för sig använda en generisk metod som anropar en annan. Tillsammans får det andra anropet E2003, oberoende av ordningen. Terminalen delar nu upp arbetet i förberedelse, ritning och publicering. En separat reproduktion på två rader visar att `unwrap()` på `Result<(),string>` ger E1002 även utan generiska metoder i användarens kod. Båda fallen har verifierats med den publicerade Dunder 0.11.2 på macOS ARM64. [Metodprov](../tests/dogfood/compiler/nested_methods.ddr) · [Resultatprov](../tests/dogfood/compiler/unwrap_unit.ddr)

## Kontrollerat med den publicerade 0.11.2

Åtta av de nio tidigare punkterna har fått ändrat beteende i Dunder. Bibliotekets eget test och en fristående konsument läser nu samma inbäddade text och byte. En muterande metod kan lämna en delad vy. En fortsatt mutation medan vyn används ger E4003. För ett temporärt lån visar E4005 också var adressen tas i diffkoden. Formateraren bevarar båda generiska fälten. Standardbibliotekets lånade grafem matchar 766 Unicode-prov, och UTF-8-avkodaren skiljer en delad tangentsekvens från ogiltiga byte. [Dunder 0.11.2](https://github.com/Mik-pe/dunder/releases/tag/v0.11.2) · [Textprov](../tests/stdlib_text.ddr)

Terminalprovet startar två barn på varsin pseudoterminal med `spawn_with_handles`. Barnen ser 40 × 10 och 23 × 7 celler, tar emot varsin tangent och lämnar tillbaka sina ursprungliga terminalinställningar när `RawMode` går ur räckvidd. Den nionde punkten gällde prestanda. Båda postrepresentationerna ger fortfarande 16 387 allokeringsanrop och samma summa. Antalet färska allokeringar är 4 099 respektive fyra. Dunder har publicerat en jämförelse av sex representationer och skrivsätt. [Terminalprov](../tests/pty_handles.ddr) · [Dunders postmätning](https://github.com/Mik-pe/dunder/tree/v0.11.2/benchmarks/record-push)

Två behov kvarstår i diagram- och terminalarbetet. Föräldern måste använda termios för att ändra en separat terminal innan barnet startar. I ett isolerat färgprov kostar en kortlivad `Some(color)` ett extra allokeringsanrop per avläsning. På 65 536 avläsningar blev det 65 795 anrop mot 259 vid direkt användning av en lånad eller ägd färg. Alla gav summan 549 755 781 120. Diagrammet återanvänder nu förberedda stilar. [PTY-start](../tools/pty.ddr) · [Färgprov](../benchmarks/language/colors.ddr) · [Diagrammätning](../benchmarks/charts.md)

## Fynd till och med Dunder 0.11.1

Genomgången omfattar arbetshistoriken från de första terminalbackendarna till layoutmotorn, versionerna 0.1–0.7. Åtta fynd har en användning utanför TUI-biblioteket. Det första är ett reproducerat paketeringsfel. De följande två gäller språkets låneregler. Fyra gäller standardbiblioteket och ett gäller kodgenereringen. Den självbyggda kompilatorn var Dunder 0.11.1 på macOS ARM64. [Granskad Dunder-revision](https://github.com/Mik-pe/dunder/tree/d723527782b946df71eac3f304c5366245a32f1a)

| Prioritet | Fynd | Vad Dunder kan ta vidare |
| --- | --- | --- |
| 1 | **Inbäddade filer får olika basväg.** Med `root = "src/lib.ddr"` hittar paketets test `@embed_file("src/data.txt")`. Samma uttryck ger E3099 hos en konsument med lokalt paketberoende. Byter man till `"data.txt"` fungerar konsumenten och bibliotekets eget test faller. Båda riktningarna reproducerades på 0.10.0, 0.11.0 och 0.11.1. | Låt manifestets katalog bestämma basen genom hela paketkedjan. Kör samma källa direkt, som test och som beroende. [Minimala paket](../tests/dogfood/embed) · [Laddaren](https://github.com/Mik-pe/dunder/blob/d723527/selfhost/driver/modules.ddr) |
| 2 | **En metod som uppdaterar en cache kan inte returnera ett delat lån från `&mut self`.** Signaturen i provet nedan ger E4003 på alla tre versioner. Bokhyllan behöver därför kedjan `arrange(area).regions()`. | Låt ett exklusivt inlån ge ett delat utlån med samma källa. Ett språkprov behöver även avvisa fortsatt mutation medan resultatet används. [Dunders typkontroll](https://github.com/Mik-pe/dunder/blob/d723527/selfhost/frontend/frontend_types.ddr) · [Bibliotekets anrop](../examples/bokhylla/main.ddr) |
| 3 | **Diagnostiken för ett temporärt lån pekar bara på anropet.** `frame.ansi_diff(&tui.Frame::new(3,2), &mut out)` ger E4005 om en rå adress. Med `previous := tui.Frame::new(3,2)` och `&previous` byggs och körs samma diff på alla tre versioner. | Visa vilken parameter eller vidarebefordran som leder till adressen och när ett namngivet värde löser problemet. En eventuell ändring av låneregeln behöver egna livstidstester. [Diffens implementation](../src/tui.ddr) |
| 4 | **Unicode-grafem.** Text, maskering och markörplacering behöver olika sätt att räkna text. Biblioteket implementerar utökade grafemgränser, inklusive regionala indikatorer, kombinerande tecken och emoji med ZWJ. Det har 766 officiella testfall för Unicode 17. | Ge `std.text` en grafemiterator som kan returnera lånade UTF-8-intervall. Befintlig skaläriteration är redan användbar. Terminalbredder och ritregler behöver kunna väljas av konsumenten. [Segmentering](../src/width.ddr) · [Testdata](../tests/grapheme_cases.ddr) |
| 5 | **Stegvis avkodning av UTF-8.** En halv tangent från en bytekälla behöver kunna vänta på nästa läsning. `std.bytes.decode_range` producerar en ägd sträng eller ett fel. Tangentavkodaren kontrollerar prefixet själv och tar sedan ut första skalären ur strängen. | En prefixavkodare för `&[]u8` kan ge en skalär och byteantal, ofullständig indata eller ogiltig indata. Det behövs även för strömmande parsrar. [Tangentavkodare](../src/input.ddr) · [Nuvarande std-API](https://github.com/Mik-pe/dunder/blob/d723527/stdlib/bytes/utf8.ddr) |
| 6 | **Terminalstorlek och återställning av konsolläge.** POSIX-backenden använder termios och plattformsspecifika ioctl-anrop. Windows-backenden sparar och återställer konsollägen med Win32. Båda operationerna behövs också i en enkel lösenordsprompt eller REPL. | En liten terminalresurs i standardbiblioteket kan exponera storlek, raw mode och återställning med ett tydligt ägarskap. Färger, layout och widgetar hör hemma i paketet. [POSIX](../src/posix.ddr) · [Windows](../src/windows.ddr) |
| 7 | **Anslut ett barns stdio till ett redan öppet handtag.** `std.process.Stdio` erbjuder Inherit, Pipe och Null. PTY-provet måste tillfälligt byta förälderns fd 0 och 1 med `dup2` medan barnet startas. | Stöd ett valt handtag per barnström med klara äganderegler. Prova två samtidiga barn med olika PTY och oförändrad förälder. Den nativa Windows-motsvarigheten behöver eget prov. [PTY-start](../tools/pty.ddr) · [Processgränssnitt](https://github.com/Mik-pe/dunder/blob/d723527/stdlib/process.ddr) |
| 8 | **Små poster i täta arrayer.** I ett isolerat prov byggs 4 × 4 096 poster med två skalärfält. Båda representationerna gav summan 33 595 386. På 0.11.1 gjorde båda 16 387 allokeringsanrop. Vanliga poster behövde 4 099 färska allokeringar och `@c_layout` behövde fyra. | Använd provet för att följa temporära postvärden vid `push` och undersöka tät lagring även för vanliga värdeposter. Redovisa poolåterbruk och färska allokeringar var för sig. [Körbart prov](../benchmarks/language/records.ddr) · [Dunders minnesmodell](https://github.com/Mik-pe/dunder/blob/d723527/docs/memory.md) |

Tabellarbetet i 0.8.0 gav ett nionde fynd. Med två generiska fält av samma typ lägger `dunder fmt` i 0.11.1 in mellanslag runt vinkelparenteserna på sista fältet. Alla nio punkter fördes in med prov eller acceptanskriterier i språkets arbetslista. [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/eaf0971d24467e91bfa565507dede10732e795b4/docs/next-steps.md)

## Återskapa

Kör paketeringsprovet från repots rot. Med 0.11.2 ska båda kommandona lyckas. Till och med 0.11.1 lyckades bibliotekets eget test medan konsumenten fick E3099. Båda använder enbart lokala filer.

```sh
(cd tests/dogfood/embed/library && dunder test --locked --offline)
(cd tests/dogfood/embed/app && dunder run --locked --offline)
```

Den minsta reproduktionen för delat återlån är följande.

```dunder
struct Store { values: []i32 }
impl Store {
    fn refresh(&mut self) -> &[]i32 from self {
        self.values.push(7)
        &self.values
    }
}
fn main() { mut s := Store { values: [] }; assert(s.refresh()[0] == 7) }
```

Kör kompilerings- och minnesprovet med den kompilator som ska utvärderas.

```sh
mkdir -p build
dunder build --locked --offline --runtime-profile benchmarks/language/records.ddr -o build/records
build/records plain
build/records flat
dunder build --locked --offline --runtime-profile benchmarks/language/colors.ddr -o build/colors
for mode in borrowed owned optional packed; do build/colors "$mode"; done
dunder test --locked --offline tests/stdlib_text.ddr
dunder run --locked --offline tests/pty_handles.ddr
```

## Tidigare observationer som kunde förklaras

- Ett fast `[string; 2]` accepteras av en funktion som tar `&[]string` med `&names[0..names.len]`. Den historiska E3001-diagnostiken gällde den direkta formen `&names`.
- `for r in text.runes()` har redan en strömmande väg. En sparad `runes()`-array och en iterator över grafem fyller andra behov. [Strängsemantik](https://github.com/Mik-pe/dunder/blob/d723527/docs/memory.md)
- Den minsta positiva skillnaden i 100 000 anrop till `std.time.monotonic_ns()` var 1 000 ns på den provade Macen. Layoutprovet mäter därför grupper om 64 delningar. Det räcker inte för att dra en slutsats om kompilatorns klockkostnad på andra system.
- Anropet till `next_run` låg kvar efter ett namnbyte i TUI-koden. Det kunde rättas på anropsstället. De dokumenterade fallen där Ratatui ritar utanför en ruta gäller referenswidgetarnas beteende. [Beteendeproven](parity.md)
