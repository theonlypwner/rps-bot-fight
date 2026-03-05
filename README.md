# Rock-Paper-Scissors Bot Fight

I ported [sparemind/RockPaperScissorsBots](https://github.com/sparemind/RockPaperScissorsBots) to Rust.

I don't remember the motivation for doing so.

## Improvements

### Command Line Options

The number of games and rounds are now specified at runtime, instead of being hardcoded.

### ReflectiveBot Fix

There seems to be a bug in `ReflectiveBot`, causing the predictor score to be updated incorrectly. `ReflectiveBot2` fixes it.

### History Bots Speedup

`HistoryBot`, `MetaBot`, and `ReflectiveBot` are optimized by using a suffix automaton (SAM) and link cut tree (LCT) instead of a naive search every time a move is made.

This code also fixes a fencepost error in the original code, which ignores the first character when searching for suffixes that appear earlier.

| Algorithm | Time Per Move | Total Time | Space |
|-----------|---------------|------------|-------|
| Naive     | O(n^2)        | O(n^3)     | O(n)  |
| Improved  | O(n)          | O(n^2)     | O(n)  |
| Optimized | O(log n)      | O(n log n) | O(n)  |

The O(log n) optimal solution outperforms the O(n) solution only for a large enough number of rounds per game.

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
        2000 round long games
        1000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won             Rounds Won                  Nemesis          Rounds Lost to Nemesis
==================================================================================================================
MetaBot                11981/14000 (85.6%)   18025425/28000000 (64.4%)   HistoryBot        685532/2000000 (34.3%)
ReflectiveBot2         11296/14000 (80.7%)   17792330/28000000 (63.5%)   MetaBot          1317054/2000000 (65.9%)
MarkovBot              10944/14000 (78.2%)   16573541/28000000 (59.2%)   BiasBot           667450/2000000 (33.4%)
HistoryBot             10651/14000 (76.1%)   16050974/28000000 (57.3%)   ReflectiveBot2   1999348/2000000 (100.0%)
ReflectiveBot           9928/14000 (70.9%)   10009139/28000000 (35.7%)   MetaBot          1358572/2000000 (67.9%)
BiasBot                 9012/14000 (64.4%)   15208296/28000000 (54.3%)   ReflectiveBot     821449/2000000 (41.1%)
RandomDummy             7005/14000 (50.0%)    9335276/28000000 (33.3%)   BiasBot           666949/2000000 (33.3%)
DecayingFrequencyBot    6200/14000 (44.3%)   14359691/28000000 (51.3%)   BiasBot          1332673/2000000 (66.6%)
FrequencyBot            5423/14000 (38.7%)   12438592/28000000 (44.4%)   HistoryBot       1441747/2000000 (72.1%)
DeBruijnDummy           5149/14000 (36.8%)    8245607/28000000 (29.4%)   ReflectiveBot2   1327158/2000000 (66.4%)
PatternDummy            3988/14000 (28.5%)    6237633/28000000 (22.3%)   HistoryBot       1991154/2000000 (99.6%)
PaperDummy              3925/14000 (28.0%)    5953287/28000000 (21.3%)   ScissorsDummy    2000000/2000000 (100.0%)
FlatBot                 3534/14000 (25.2%)    6857124/28000000 (24.5%)   PaperDummy       1334569/2000000 (66.7%)
RockDummy               1962/14000 (14.0%)    4632403/28000000 (16.5%)   PaperDummy       2000000/2000000 (100.0%)
ScissorsDummy           1928/14000 (13.8%)    5275395/28000000 (18.8%)   RockDummy        2000000/2000000 (100.0%)
```
