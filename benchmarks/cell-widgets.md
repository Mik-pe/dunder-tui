# Radfyllning och radjämförelse i 0.19.0

Riktextvyn ritar 23 rader med en återanvänd `Fill` och centrerade textspann. Båda implementationerna visar samma 1 100 bilder, cellstilar, markörpositioner och bildnummer. En separat jämförelse läser även celler som döljs bakom en bred symbol. [Dunder](terminal.ddr) · [Ratatui 0.30.2](reference/examples/terminal_bench.rs) · [Prov med Fill och Clear](../tests/cell_widgets.ddr)

Varje par visar median / p95 i µs för en bild inklusive terminalutmatning. Efter två uppvärmningar körs sju processpar i växlande ordning. De första 20 av 220 bilder per arbetslast lämnas utanför tidtagningen. Macen har en Apple M5 och kör även Linux ARM64 i en gäst. Linux x64 kördes på AMD EPYC 9V74 under Hyper-V. Windows Server 2025 x64 använde en Win32-konsolbuffert med VT och UTF-8. [Rådata med minsta och största värde](results/cell-widgets-0.19.0.tsv) · [Nativ x64-körning](https://github.com/Mik-pe/dunder-tui/actions/runs/45308458992)

| System | Bibliotek | Oförändrad | En cell | Rullning | Storleksändring | Full omritning |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| macOS ARM64 | Dunder | 16 / 21 | 17 / 21 | 29 / 33 | 56 / 60 | 54 / 57 |
| | Ratatui | 33 / 42 | 35 / 40 | 34 / 41 | 74 / 85 | 78 / 83 |
| Linux ARM64 | Dunder | 17 / 22 | 22 / 23 | 27 / 35 | 38 / 46 | 37 / 46 |
| | Ratatui | 41 / 48 | 39 / 48 | 41 / 50 | 65 / 74 | 66 / 74 |
| Linux x64 | Dunder | 187 / 203 | 184 / 196 | 228 / 242 | 451 / 474 | 368 / 383 |
| | Ratatui | 73 / 84 | 74 / 82 | 73 / 81 | 122 / 135 | 121 / 134 |
| Windows x64 | Dunder | 904 / 1 056 | 924 / 1 013 | 1 343 / 1 568 | 1 761 / 2 229 | 1 764 / 2 031 |
| | Ratatui | 216 / 308 | 215 / 262 | 243 / 335 | 367 / 474 | 383 / 458 |

Sammanlagt tog varje terminal emot 1 382 478 byte från Dunder och 1 624 225 från Ratatui. Dunder gjorde 83 025 allokeringsanrop på Unix, varav 272 hämtade nytt minne och 82 753 återanvände poolen. Ratatui gjorde 23 188 på macOS, 23 184 på Linux och 23 227 på Windows. Den publicerade Dunder-kompilatorn 0.11.2 saknar allokeringsrapport på Windows. Medianen av tre processers högsta RSS i KiB var 2 048 / 2 464 på macOS, 440 / 1 952 på Linux ARM64 och 1 124 / 2 644 på Linux x64, med Dunder först. På Windows var motsvarande högsta arbetsminne 4 088 / 4 264 KiB.

## Två biblioteksrevisioner på samma maskin

Med samma program och kompilator på Linux x64 tog en oförändrad bild i pseudoterminalen 231 / 246 µs före ändringen och 183 / 201 µs efter. För full omritning var tiderna 398 / 413 respektive 364 / 377 µs. Biblioteket avgränsar strängjämförelser till kolumner där en sträng har lagrats och kontrollerar överlapp bara på rader där breda tecken har skrivits. Vid en separat Callgrind-körning av 220 oförändrade bilder stod `Frame::same_row` för 1,63 % av instruktionerna och `Frame::fill_cells` för 19,81 %. [Före, efter och instruktionsprofil](https://github.com/Mik-pe/dunder-tui/actions/runs/45308458992/job/138157952246)

Bygg samma ligaturprov lokalt med biblioteket från `ece8a697` respektive `v0.19.0`. Kör båda på samma x64-maskin med `TUI_BENCH_SCENE=ligatures`. [Lokala mätkommandon](ligatures.md#upprepa-mätningen)
