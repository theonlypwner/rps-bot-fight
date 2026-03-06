# Rock-Paper-Scissors Bot Fight

I ported [sparemind/RockPaperScissorsBots](https://github.com/sparemind/RockPaperScissorsBots) to Rust.

I don't remember the motivation for doing so.

## Improvements

### Command Line Options

The number of games and rounds are now specified at runtime, instead of being hardcoded.

### ReflectiveBot Fix

There seems to be a bug in `ReflectiveBot`, causing the predictor score to be updated incorrectly. `ReflectiveBot2` fixes it.

### History Bots Speedup

`HistoryBot`, `MetaBot`, and `ReflectiveBot` are optimized by using a suffix automaton (SA) and link cut tree (LCT) instead of a naive search every time a move is made.

This code also fixes a fencepost error in the original code, which ignores the first character when searching for suffixes that appear earlier.

| Algorithm | Time Per Move | Total Time | Space |
|-----------|---------------|------------|-------|
| Naive     | O(n^2)        | O(n^3)     | O(n)  |
| SA        | O(n)          | O(n^2)     | O(n)  |
| SA+LCT    | O(log n)      | O(n log n) | O(n)  |

A LCT improves the time complexity, but it has a higher constant factor.
The code's hybrid algorithm initially uses SA only but later builds the LCT and switches to it after the number of moves grows large enough.

### Parallelism Speedup

Using all CPU cores with multithreading speeds up the program.

## Example Tournament Output

```text
Playing tournament with:
        1000 round long games
        10 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won         Rounds Won              Nemesis                Rounds Lost to Nemesis
==============================================================================================================
MetaBot                106/140 (75.7%)    86166/140000 (61.5%)   MarkovBot               3378/10000 (33.8%)
ReflectiveBot2         106/140 (75.7%)    83972/140000 (60.0%)   MetaBot                 6594/10000 (65.9%)
MarkovBot              102/140 (72.9%)    78316/140000 (55.9%)   RandomDummy             3359/10000 (33.6%)
HistoryBot             100/140 (71.4%)    76440/140000 (54.6%)   ReflectiveBot2          9991/10000 (99.9%)
BiasBot                 89/140 (63.6%)    74419/140000 (53.2%)   HistoryBot              3900/10000 (39.0%)
DeBruijnDummy           89/140 (63.6%)    48201/140000 (34.4%)   BiasBot                 3373/10000 (33.7%)
ReflectiveBot           82/140 (58.6%)    49694/140000 (35.5%)   MetaBot                 6723/10000 (67.2%)
RandomDummy             67/140 (47.9%)    46645/140000 (33.3%)   HistoryBot              3400/10000 (34.0%)
DecayingFrequencyBot    63/140 (45.0%)    72166/140000 (51.5%)   BiasBot                 6656/10000 (66.6%)
FrequencyBot            55/140 (39.3%)    62008/140000 (44.3%)   DecayingFrequencyBot    7116/10000 (71.2%)
PaperDummy              46/140 (32.9%)    29616/140000 (21.2%)   ScissorsDummy          10000/10000 (100.0%)
FlatBot                 41/140 (29.3%)    35044/140000 (25.0%)   DecayingFrequencyBot    6729/10000 (67.3%)
PatternDummy            35/140 (25.0%)    29763/140000 (21.3%)   HistoryBot              9905/10000 (99.0%)
RockDummy               24/140 (17.1%)    23516/140000 (16.8%)   PaperDummy             10000/10000 (100.0%)
ScissorsDummy           19/140 (13.6%)    26210/140000 (18.7%)   RockDummy              10000/10000 (100.0%)
```

With larger sizes, it still runs quite quickly.

```text
Playing tournament with:
        100000 round long games
        10 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won         Rounds Won                  Nemesis                Rounds Lost to Nemesis
====================================================================================================================
ReflectiveBot2         121/140 (86.4%)    9619955/14000000 (68.7%)   MetaBot                 637269/1000000 (63.7%)
MetaBot                112/140 (80.0%)    9445793/14000000 (67.5%)   RandomDummy             333642/1000000 (33.4%)
HistoryBot             109/140 (77.9%)    8476256/14000000 (60.5%)   ReflectiveBot2          999995/1000000 (100.0%)
MarkovBot              101/140 (72.1%)    9450439/14000000 (67.5%)   ReflectiveBot2          549725/1000000 (55.0%)
BiasBot                 92/140 (65.7%)    7551982/14000000 (53.9%)   ReflectiveBot           487968/1000000 (48.8%)
ReflectiveBot           78/140 (55.7%)    5204361/14000000 (37.2%)   ReflectiveBot2          799992/1000000 (80.0%)
RandomDummy             68/140 (48.6%)    4666524/14000000 (33.3%)   FlatBot                 334399/1000000 (33.4%)
DecayingFrequencyBot    65/140 (46.4%)    6930500/14000000 (49.5%)   MarkovBot               921013/1000000 (92.1%)
FrequencyBot            62/140 (44.3%)    6182877/14000000 (44.2%)   MarkovBot               986044/1000000 (98.6%)
DeBruijnDummy           55/140 (39.3%)    3387164/14000000 (24.2%)   ReflectiveBot2          993243/1000000 (99.3%)
PaperDummy              48/140 (34.3%)    2992169/14000000 (21.4%)   ScissorsDummy          1000000/1000000 (100.0%)
FlatBot                 38/140 (27.1%)    3257872/14000000 (23.3%)   DecayingFrequencyBot    666472/1000000 (66.6%)
PatternDummy            33/140 (23.6%)    2978798/14000000 (21.3%)   HistoryBot              999914/1000000 (100.0%)
ScissorsDummy           27/140 (19.3%)    2588877/14000000 (18.5%)   RockDummy              1000000/1000000 (100.0%)
RockDummy               25/140 (17.9%)    2311551/14000000 (16.5%)   PaperDummy             1000000/1000000 (100.0%)
```

```text
Playing tournament with:
        12000 round long games
        100 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won           Rounds Won                  Nemesis          Rounds Lost to Nemesis
================================================================================================================
ReflectiveBot2         1180/1400 (84.3%)   11086932/16800000 (66.0%)   MetaBot           767744/1200000 (64.0%)
MetaBot                1163/1400 (83.1%)   11210929/16800000 (66.7%)   BiasBot           401875/1200000 (33.5%)
HistoryBot             1069/1400 (76.4%)   10038145/16800000 (59.8%)   ReflectiveBot2   1199933/1200000 (100.0%)
MarkovBot              1020/1400 (72.9%)   10720073/16800000 (63.8%)   ReflectiveBot2    407585/1200000 (34.0%)
BiasBot                 928/1400 (66.3%)    9126587/16800000 (54.3%)   ReflectiveBot     540310/1200000 (45.0%)
ReflectiveBot           888/1400 (63.4%)    6032618/16800000 (35.9%)   ReflectiveBot2    827936/1200000 (69.0%)
RandomDummy             716/1400 (51.1%)    5601260/16800000 (33.3%)   FrequencyBot      400967/1200000 (33.4%)
DecayingFrequencyBot    614/1400 (43.9%)    8591880/16800000 (51.1%)   BiasBot           799865/1200000 (66.7%)
FrequencyBot            567/1400 (40.5%)    7387847/16800000 (44.0%)   MarkovBot        1066858/1200000 (88.9%)
DeBruijnDummy           539/1400 (38.5%)    4207273/16800000 (25.0%)   ReflectiveBot2   1132749/1200000 (94.4%)
PaperDummy              420/1400 (30.0%)    3554232/16800000 (21.2%)   ScissorsDummy    1200000/1200000 (100.0%)
PatternDummy            405/1400 (28.9%)    3779597/16800000 (22.5%)   HistoryBot       1199160/1200000 (99.9%)
FlatBot                 350/1400 (25.0%)    3978931/16800000 (23.7%)   FrequencyBot      800861/1200000 (66.7%)
ScissorsDummy           221/1400 (15.8%)    3187357/16800000 (19.0%)   RockDummy        1200000/1200000 (100.0%)
RockDummy               214/1400 (15.3%)    2742977/16800000 (16.3%)   PaperDummy       1200000/1200000 (100.0%)
```

```text
Playing tournament with:
        3000 round long games
        1000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won             Rounds Won                  Nemesis          Rounds Lost to Nemesis
==================================================================================================================
ReflectiveBot2         11717/14000 (83.7%)   27113491/42000000 (64.6%)   MetaBot          1954001/3000000 (65.1%)
MetaBot                11629/14000 (83.1%)   27377740/42000000 (65.2%)   HistoryBot       1012276/3000000 (33.7%)
HistoryBot             11032/14000 (78.8%)   24433993/42000000 (58.2%)   ReflectiveBot2   2999352/3000000 (100.0%)
MarkovBot              10951/14000 (78.2%)   25416810/42000000 (60.5%)   ReflectiveBot2   1001947/3000000 (33.4%)
ReflectiveBot           8807/14000 (62.9%)   15009675/42000000 (35.7%)   MetaBot          2034606/3000000 (67.8%)
BiasBot                 8684/14000 (62.0%)   22729387/42000000 (54.1%)   ReflectiveBot    1254003/3000000 (41.8%)
RandomDummy             6894/14000 (49.2%)   13996304/42000000 (33.3%)   ReflectiveBot2   1001138/3000000 (33.4%)
DecayingFrequencyBot    6161/14000 (44.0%)   21541783/42000000 (51.3%)   BiasBot          1998662/3000000 (66.6%)
FrequencyBot            5653/14000 (40.4%)   18661693/42000000 (44.4%)   MarkovBot        2220921/3000000 (74.0%)
DeBruijnDummy           5150/14000 (36.8%)   11635635/42000000 (27.7%)   ReflectiveBot2   2327594/3000000 (77.6%)
PaperDummy              4228/14000 (30.2%)    8927731/42000000 (21.3%)   ScissorsDummy    3000000/3000000 (100.0%)
PatternDummy            3978/14000 (28.4%)    9367439/42000000 (22.3%)   HistoryBot       2991220/3000000 (99.7%)
FlatBot                 3538/14000 (25.3%)   10212432/42000000 (24.3%)   PaperDummy       2001671/3000000 (66.7%)
RockDummy               2276/14000 (16.3%)    6941267/42000000 (16.5%)   PaperDummy       3000000/3000000 (100.0%)
ScissorsDummy           2251/14000 (16.1%)    7917346/42000000 (18.9%)   RockDummy        3000000/3000000 (100.0%)
```

```text
Playing tournament with:
        1000 round long games
        8000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won               Rounds Won                    Nemesis                Rounds Lost to Nemesis
============================================================================================================================
MetaBot                 87932/112000 (78.5%)    69006298/112000000 (61.6%)   DeBruijnDummy          2679948/8000000 (33.5%)
ReflectiveBot2          85049/112000 (75.9%)    68537724/112000000 (61.2%)   MetaBot                5308980/8000000 (66.4%)
MarkovBot               83097/112000 (74.2%)    62718735/112000000 (56.0%)   DeBruijnDummy          2684049/8000000 (33.6%)
HistoryBot              77327/112000 (69.0%)    61045439/112000000 (54.5%)   ReflectiveBot2         7994658/8000000 (99.9%)
ReflectiveBot           69787/112000 (62.3%)    40146304/112000000 (35.8%)   MetaBot                5379089/8000000 (67.2%)
BiasBot                 69375/112000 (61.9%)    60768423/112000000 (54.3%)   ReflectiveBot          3150079/8000000 (39.4%)
DeBruijnDummy           69365/112000 (61.9%)    38547147/112000000 (34.4%)   BiasBot                2698686/8000000 (33.7%)
RandomDummy             55101/112000 (49.2%)    37333332/112000000 (33.3%)   HistoryBot             2670259/8000000 (33.4%)
DecayingFrequencyBot    49420/112000 (44.1%)    57496047/112000000 (51.3%)   BiasBot                5325287/8000000 (66.6%)
FrequencyBot            42620/112000 (38.1%)    50049997/112000000 (44.7%)   DecayingFrequencyBot   5693239/8000000 (71.2%)
PaperDummy              34306/112000 (30.6%)    23872875/112000000 (21.3%)   ScissorsDummy          8000000/8000000 (100.0%)
PatternDummy            32244/112000 (28.8%)    25154275/112000000 (22.5%)   HistoryBot             7929711/8000000 (99.1%)
FlatBot                 28139/112000 (25.1%)    27778442/112000000 (24.8%)   PaperDummy             5346587/8000000 (66.8%)
ScissorsDummy           18303/112000 (16.3%)    21169192/112000000 (18.9%)   RockDummy              8000000/8000000 (100.0%)
RockDummy               18260/112000 (16.3%)    18520769/112000000 (16.5%)   PaperDummy             8000000/8000000 (100.0%)
```

```text
Playing tournament with:
        100 round long games
        120000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won                 Rounds Won                    Nemesis          Rounds Lost to Nemesis
==========================================================================================================================
MetaBot                1352064/1680000 (80.5%)    99953573/168000000 (59.5%)   BiasBot           4200887/12000000 (35.0%)
BiasBot                1316844/1680000 (78.4%)    92351648/168000000 (55.0%)   ReflectiveBot     4200105/12000000 (35.0%)
ReflectiveBot2         1231718/1680000 (73.3%)   101523276/168000000 (60.4%)   MetaBot           7918156/12000000 (66.0%)
MarkovBot              1154798/1680000 (68.7%)    86450191/168000000 (51.5%)   RandomDummy       4001454/12000000 (33.3%)
HistoryBot             1034149/1680000 (61.6%)    85170926/168000000 (50.7%)   ReflectiveBot2   11920144/12000000 (99.3%)
DeBruijnDummy           892979/1680000 (53.2%)    58676626/168000000 (34.9%)   MetaBot           4840156/12000000 (40.3%)
ReflectiveBot           891110/1680000 (53.0%)    59159415/168000000 (35.2%)   MetaBot           7961572/12000000 (66.3%)
RandomDummy             799461/1680000 (47.6%)    55999288/168000000 (33.3%)   FrequencyBot      4004015/12000000 (33.4%)
DecayingFrequencyBot    776279/1680000 (46.2%)    87767072/168000000 (52.2%)   BiasBot           7880307/12000000 (65.7%)
FrequencyBot            648107/1680000 (38.6%)    76938397/168000000 (45.8%)   BiasBot           8412879/12000000 (70.1%)
PaperDummy              512849/1680000 (30.5%)    36777879/168000000 (21.9%)   ScissorsDummy    12000000/12000000 (100.0%)
FlatBot                 483365/1680000 (28.8%)    43048125/168000000 (25.6%)   PaperDummy        8169987/12000000 (68.1%)
PatternDummy            478542/1680000 (28.5%)    40120143/168000000 (23.9%)   HistoryBot       10944668/12000000 (91.2%)
RockDummy               392544/1680000 (23.4%)    28395954/168000000 (16.9%)   PaperDummy       12000000/12000000 (100.0%)
ScissorsDummy           272691/1680000 (16.2%)    32034478/168000000 (19.1%)   RockDummy        12000000/12000000 (100.0%)
```
