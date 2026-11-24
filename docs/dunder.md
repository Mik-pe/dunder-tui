# Vad TUI-arbetet visar om Dunder

## Omprov med 0.11.3

Biblioteket använder nu `std.terminal.window_size` för Unix-terminalens cell- och pixelmått, `std.array.resize` i bildbufferten, `std.array.append_move` för ägda textrader och `std.utf16.append` i Win32-konsolen. De tidigare minimiproven för två skilda fältlån, tilldelning i en matchgren, härledd generisk mottagare, två generiska mottagartyper, direkt jämförelse av arrayvyer och uppackning av `Result<(),string>` går nu att bygga och köra på macOS. Två samtidigt levande grafem från samma iterator och två olika ägare i en lokal post ger fortfarande E4003. [Minimiproven](../tests/dogfood/compiler) · [Dunder 0.11.3](https://github.com/Mik-pe/dunder/releases/tag/v0.11.3)

Kalendern använder även `std.time.local_from_unix` på macOS och Linux. Åtta tidpunkter gav samma datum i tre tidszoner med båda Unixbyggena och på Macen. I Win32-provet under Wine gav systemets kalenderomvandling 1969-12-31 för tidsstämpeln −1 i UTC, medan standardbiblioteket gav `.None`. Windowsversionen av kalendern behåller därför sitt Win32-anrop. [Jämför med `--compare -315619200 -1 0 1711924200 32535216000`](../tests/calendar_clock.ddr)

Ett bildrutnät med 24 rader om 96 heltal jämförs med `==`, `std.array.equal` och `std.bytes.equal_memory`. Varannan bild skiljer sig i en rads sista kolumn. På macOS ARM64 tog jämförelserna 9,85, 9,91 och 1,53 µs per bild. På Linux ARM64 i lokal VM tog de 8,43, 10,12 och 1,14 µs. Tiderna är medianer av sex processmedianer med växlande ordning. Varje process kontrollerar summan 330 880. Därför använder renderaren fortfarande bytejämförelse för de tätt lagrade heltalen. [Körbart prov med lägena `eq`, `array` och `memory`](../benchmarks/language/row_equality.ddr)

## Arraykrympning och bildtid

När en bildbuffert blir mindre kan biblioteket kapa arrayerna med `truncate`. På Linux ARM64 tog krympning från 65 536 till 16 heltal 0,32 µs i median. `std.array.resize` tog 136 µs för samma innehåll. Provet kontrollerar prefix, kapacitet och att det går att lägga till ett nytt värde. [Körbart prov](../benchmarks/language/shrink_arrays.ddr) · [Förslag i Dunder](https://github.com/Mik-pe/dunder/blob/e896509250e36d5f275c18b71713790228a18edd/docs/next-steps.md#krymp-arrayer-med-truncate-i-stdarrayresize)

På macOS ARM64 rapporterade Dunders monotona klocka steg på 1 000 ns. Därför mäter buffertprovet 512 storleksändringar per tidsintervall. En separat kontroll jämför klockans rapporterade upplösning med observerade tidssteg. [Klockprov](../benchmarks/language/monotonic_clock.ddr) · [Förslag i Dunder](https://github.com/Mik-pe/dunder/blob/e896509250e36d5f275c18b71713790228a18edd/docs/next-steps.md#exponera-klockans-upplösning-för-korta-tidsmätningar)

## Paket och barnprocesser på Windows

På Windows Server 2025 x64 stoppar Dunder 0.11.2 ett paket med noll beroenden när den öppnar Git-lagret. Samma källfiler går att bygga med filingången utanför manifestet. Ett separat fel avslutar ett program som startar ett barn via `std.process` med kod 101. Process-ID och tråd-ID läses som ett enda heltal efter `CreateProcessW`. [Minimalt processprov](../tests/dogfood/compiler/windows_child.ddr) · [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/2c3d893/docs/next-steps.md#verifiera-windows-som-kompilatorvärd)

## Terminalbredd för lånade grafem

I Dunder 0.11.2 ger `std.text.graphemes` ett element för både 😶‍🌫️ och 😶‍🌫. Ändå behöver en terminalapp placera nästa bokstav två respektive tre kolumner längre fram. En breddfunktion i standardbiblioteket skulle låta terminalappar använda den lånade vyn direkt. TUI-provet kontrollerar bland annat var bokstaven hamnar efter en omritning i en pseudoterminal. [Grafemprovet](../tests/stdlib_text.ddr) · [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/4d72641f341c500c4c6f8421836409838b717964/docs/next-steps.md#mät-ett-grafem-i-terminalkolumner)

## Läs koordinatpar genom lån

I Dunder 0.11.2 ger `a := points[i]` en ägd kopia av koordinatparet, medan `a := &points[i]` lånar elementet. Det fristående provet summerar två angränsande punkter i taget. Med 1 100 genomgångar av 96 punkter gav kopievarianten 209 000 tupelkopior och lånevarianten noll. Båda gav summan 836 000. Två diagramserier gjorde samma läsning i TUI-mätningen. Enbart ändringen till lån tog bort 418 000 allokeringsanrop från den körningen. [Mätprogram](../benchmarks/language/borrowed_points.ddr) · [Förslag till Dunders kompilator](https://github.com/Mik-pe/dunder/blob/1d4d815f1e7cd60ca7e68f2eed97f44eeb0bc24e/docs/next-steps.md#undvik-kopior-vid-läsning-av-koordinatpar)

## Skicka vidare ett lyckat Result

När terminalen började tömma backendens utmatning efter varje bild ökade antalet allokeringsanrop. Genom att skicka tillbaka dess lyckade resultat med `move` kunde terminalen undvika 1 100 nykonstruerade resultat i mätningen med 1 100 bilder. I det isolerade provet anropar två program samma operation och kontrollerar samma felmeddelande. Den variant som bygger en ny `.Ok` gör ett extra poolåterbruk per lyckat anrop på Dunder 0.11.2. [Resultatprov](../benchmarks/language/unit_result.ddr) · [Förslag för kompilatorn](https://github.com/Mik-pe/dunder/blob/99e96aba89d6913e0b7c0411deeff435204fa16e/docs/next-steps.md#återanvänd-lyckade-resultat-vid-vidarebefordran)

## Stegvis läsning ur en bytekö

Med Dunder 0.11.2 tog det 105 ms i median att tömma 16 KiB en byte i taget med `std.bytes.consume_prefix` på macOS ARM64. Med en lånad vy och en läsposition tog det 53 µs. Terminalen använder nu en läsposition och packar ihop återstående byte när den behöver läsa mer. Skillnaden märks när stora inklistringar kommer samtidigt som ett markörsvar. [Körbart mätprov](../benchmarks/language/byte_queue.ddr) · [Förslag till standardbiblioteket](https://github.com/Mik-pe/dunder/blob/94e98cc01e8b112aa20d3b8095a0391a65664c90/docs/next-steps.md#läs-stegvis-ur-en-bytekö)

## Terminalstorlek i pixlar

`std.terminal.size()` i Dunder 0.11.2 hämtar ett Unix-svar med fyra 16-bitarsfält men exponerar bara kolumner och rader. I en pseudoterminal kunde pixelmåtten ändras från 0 × 0 till 813 × 719 medan teckenmåtten förblev 45 × 8. Om standardbiblioteket exponerar de två pixelmåtten från samma systemanrop kan appar använda dem även när bara en axel är känd. [Terminalprovet](../tests/window_terminal.ddr) · [Standardbiblioteket i 0.11.2](https://github.com/Mik-pe/dunder/blob/v0.11.2/stdlib/terminal.ddr) · [Nästa steg för terminalhandtag](https://github.com/Mik-pe/dunder/blob/main/docs/next-steps.md#styr-en-separat-terminal-via-dess-handtag)

## Lånet av en vyport vid matchning

Vid val av vyport behöver terminalen ibland anropa en muterande metod. Om den matchar direkt på sitt enumfält säger E4003 att en samling ändras under iteration. Samma program med en lokal kopia av enumvärdet går att köra med 0.11.2. [Reproduktion](../tests/dogfood/compiler/match_receiver.ddr) · [Fungerande variant](../tests/dogfood/compiler/match_receiver_copy.ddr) · [Förslag till diagnostik](https://github.com/Mik-pe/dunder/blob/50ffae36757ec86c53933e3491d9f472211ff933/docs/next-steps.md#förklara-lånet-när-en-metod-matchar-på-ett-eget-fält)

## Typnamn i diagnostiken för konstanter

En konstant palett byggd som en post med två heltal ger E3007 i Dunder 0.11.2. I diagnostiken kallas posten `__dunder_module_0` trots att källkoden säger `Scale`. Både `check` och `build` reproducerar felet. En fast array med samma värden fungerar. Bibliotekets paletter använder sådana skrivskyddade tabeller. På macOS ARM64 rapporterade `--runtime-profile` noll allokeringar för 65 536 uppslag via `tailwind.rgb` med växlande familj och nyans. [Postprov](../tests/dogfood/compiler/record_constant.ddr) · [Arrayprov](../tests/dogfood/compiler/array_constant.ddr) · [Uppslagsmätning](../benchmarks/language/palette_lookup.ddr) · [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/8d8f882/docs/next-steps.md#visa-källkodens-typnamn-i-fel-för-konstanter)

## Lokala uppslagstabeller

På macOS ARM64 med Dunder 0.11.2 gav 1 048 576 anrop till en funktion med en fast lokal array 4 194 305 allokeringsanrop. En funktion som läser samma element från en modulkonstant gav ett anrop för hela processen. Kontrollsumman var 581 894 144 i båda fallen. Linjesammanslagningen och Win32-konsolens färgval läser nu sina tabeller från modulkonstanter. Alla 50 700 linjesammanslagningar matchar Ratatui. [Jämförelseprogram](../benchmarks/language/constant_tables.ddr) · [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/a930e412c358456d1e64646c302b88f4bddbace0/docs/next-steps.md#dela-oföränderliga-lokala-uppslagstabeller)

## En vy över två lånade värden

En celldiff behöver läsa två bildbuffertar samtidigt. I 0.11.2 går det att lagra två delade lån i en post om de kommer från samma array. Två lokala ägare ger E4003 även när båda lever tills posten har använts. Biblioteket kan besöka skillnaderna med en funktionsparameter. En lånad iterator över två oberoende ägare skulle också kunna användas för jämförelse av dokument och sorterade sekvenser. [Två lokala ägare](../tests/dogfood/compiler/paired_views.ddr) · [Gemensam ägare](../tests/dogfood/compiler/paired_views_one_owner.ddr)

## Lokal tid i kalendern

Kalendern markerar dagens datum enligt datorns tidszon. I 0.11.2 erbjuder `std.time` UTC-omvandling. Kalendern använder `localtime_r` på macOS och Linux och `SystemTimeToTzSpecificLocalTime` på Windows för att få lokalt datum. Tidsstämpeln `1711924200` gav 31 mars i UTC och 1 april i Stockholm på macOS och Linux. Språkets arbetslista innehåller även kontroll av årsskifte och övergång till sommartid. [Körbart prov](../tests/calendar_clock.ddr) · [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/5d8459bef0600eb5a87080fc0d507d1c83daf777/docs/next-steps.md#hämta-lokal-kalendertid-för-ett-givet-ögonblick)

## Tre förbättringar från vyportarna

- **Plattformsvillkor före länkning.** På macOS länkar 0.11.2 in `kernel32.dll` för `if target_is("windows") && enabled`. Det hindrar programmet från att starta. Två nästlade villkor ger rätt binär. [Minimalt program](../tests/dogfood/compiler/target_condition.ddr)
- **Typinferens mellan moduler.** Om en fabrik returnerar en generisk mottagare inuti ett `Result` kan nästa generiska metodanrop få `dunder check` att avslutas med kod 103 utan diagnostik. Både lokal typannotering och definition i samma modul har verifierats som fungerande. [Tvåmodulsprov](../tests/dogfood/compiler/inferred_receiver)
- **Gemensam UTF-16-kodning.** Windowskonsolen och standardbibliotekets sökvägar bygger samma surrogatpar. Konsolen skickar en längd till operativsystemet medan sökvägsfunktionen avvisar inre nollor och avslutar med en nolla. [Konsolskrivaren](../src/windows.ddr) · [Sökvägsprovet i Dunder](https://github.com/Mik-pe/dunder/blob/v0.11.2/stdlib/sys/winpath.ddr)

Förbättringarna och deras acceptansprov finns i [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/d744946bda92e0342e5baa0feb4a32a2e780f64e/docs/next-steps.md).

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
dunder build --locked --offline --runtime-profile benchmarks/language/constant_tables.ddr -o build/tables
build/tables local
build/tables shared
dunder build --locked --offline --runtime-profile benchmarks/language/palette_lookup.ddr -o build/palette-lookup
build/palette-lookup
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

## Ändra element i en lånad array

I körlistan följer stil och status med när en programpunkt flyttas. Ett fristående prov i Dunder 0.11.2 visar vad som händer när flera poster ska ändras på plats. För `for cue in &mut cues` föreslår kompilatorn ett delat lån (E4003). Då misslyckas tilldelningen till `cue.played` (E4002). En indexloop ändrar båda posterna och profileringen registrerar noll array- och postkloner. Dunders nästa steg innehåller både en konkret diagnosförbättring och ett prov för direkt iteration med ett muterbart lån per varv. [Reproduktion](../tests/dogfood/compiler/mutable_elements.ddr) · [Indexloopen](../tests/dogfood/compiler/indexed_mutable_elements.ddr) · [Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/ec524f5/docs/next-steps.md#visa-hur-en-loop-kan-ändra-elementen-i-en-lånad-array)

## Byta plats på två ägda poster

Med tre flyttar kan två lokalt ägda poster byta plats. Bakom två muterbara lån avvisas motsvarande flytt med E4003. Där går det att använda två anrop till `replace` och en tom post. Vid 4 095 byten av poster med två arrayer gav den varianten 12 304 allokeringsanrop. Både kontrollen och lokala flyttar gav 19, och arrayens `swap` gav 21. Alla fyra körningar kontrollerade slutordningen och summan 16 383 på både macOS och Linux ARM64 med Dunder 0.11.2. [Mätprogram](../benchmarks/language/swap_storage.ddr) · [Avvisat lån](../tests/dogfood/compiler/swap_borrowed.ddr) · [Föreslaget språkstöd](https://github.com/Mik-pe/dunder/blob/a82c7d2/docs/next-steps.md#byt-två-ägda-värden-utan-ett-mellanvärde)

## Jämförelse av lånade arrayintervall

Två lånade intervall av heltal går att jämföra med `==` när de först binds till lokala namn. Skriver man båda intervallen direkt som operander föreslår Dunder 0.11.2 att lägga till ett `&` trots att det redan finns. Den namngivna varianten kördes med noll arraykloner och noll kopierade byte. [Båda fallen och föreslagen uppföljning](https://github.com/Mik-pe/dunder/blob/a08b0e3/docs/next-steps.md#jämför-två-lånade-arrayintervall-direkt)

## Konstanta strängtabeller

Maskotens 32 pixelrader har olika längd. Dunder 0.11.2 avvisar en modulkonstant av typen `[string; 4]` med E3007. Maskoten väljer därför en strängliteral per rad med `match`. På macOS ARM64 gav 65 536 uppslag i fyra lokala strängar 131 073 allokeringsanrop mot ett för `match`, inklusive uppstart. Båda summerade längderna till 131 072. [Mätprogram](../benchmarks/language/string_rows.ddr) · [Förbättring i Dunders nästa steg](https://github.com/Mik-pe/dunder/blob/ade0095/docs/next-steps.md#dela-oföränderliga-lokala-uppslagstabeller)
