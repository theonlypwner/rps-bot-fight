# Rock-Paper-Scissors Bot Fight

I ported [sparemind/RockPaperScissorsBots](https://github.com/sparemind/RockPaperScissorsBots) to Rust.

I don't remember the motivation for doing so.

## Improvements

### Command Line Options

The number of games and rounds are now specified at runtime, instead of being hardcoded.

### ReflectiveBot Fix

There seems to be a bug in `ReflectiveBot`, causing the predictor score to be updated incorrectly. `ReflectiveBot2` fixes it.

### History Bots Speedup

`HistoryBot`, `MetaBot`, and `ReflectiveBot` are optimized by using a suffix automaton (SAM) instead of a naive search every time a move is made.

This code also fixes a fencepost error in the original code, which ignores the first character when searching for suffixes that appear earlier.

### Parallelism Speedup

Using all CPU cores with multithreading speeds up the program.

## Example Tournament Output

```text
Playing tournament with:
        1000 round long games
        10 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won         Rounds Won              Nemesis          Rounds Lost to Nemesis
========================================================================================================
MetaBot                110/140 (78.6%)    86039/140000 (61.5%)   MarkovBot         3360/10000 (33.6%)
ReflectiveBot2         103/140 (73.6%)    86965/140000 (62.1%)   MetaBot           6640/10000 (66.4%)
HistoryBot             100/140 (71.4%)    76354/140000 (54.5%)   ReflectiveBot2    9994/10000 (99.9%)
MarkovBot               98/140 (70.0%)    78294/140000 (55.9%)   RandomDummy       3423/10000 (34.2%)
ReflectiveBot           91/140 (65.0%)    50535/140000 (36.1%)   ReflectiveBot2    7994/10000 (79.9%)
DeBruijnDummy           87/140 (62.1%)    48336/140000 (34.5%)   PatternDummy      3420/10000 (34.2%)
BiasBot                 84/140 (60.0%)    76878/140000 (54.9%)   ReflectiveBot     3956/10000 (39.6%)
RandomDummy             72/140 (51.4%)    46945/140000 (33.5%)   ReflectiveBot     3367/10000 (33.7%)
DecayingFrequencyBot    62/140 (44.3%)    72791/140000 (52.0%)   BiasBot           6656/10000 (66.6%)
FrequencyBot            52/140 (37.1%)    62075/140000 (44.3%)   BiasBot           7990/10000 (79.9%)
PaperDummy              44/140 (31.4%)    30275/140000 (21.6%)   ScissorsDummy    10000/10000 (100.0%)
FlatBot                 42/140 (30.0%)    35217/140000 (25.2%)   FrequencyBot      6710/10000 (67.1%)
PatternDummy            40/140 (28.6%)    31649/140000 (22.6%)   HistoryBot        9924/10000 (99.2%)
ScissorsDummy           22/140 (15.7%)    26565/140000 (19.0%)   RockDummy        10000/10000 (100.0%)
RockDummy               20/140 (14.3%)    23110/140000 (16.5%)   PaperDummy       10000/10000 (100.0%)
```

With larger sizes, it still runs quite quickly:

```text
Playing tournament with:
        2000 round long games
        1000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won             Rounds Won                  Nemesis          Rounds Lost to Nemesis
==================================================================================================================
MetaBot                12037/14000 (86.0%)   18028576/28000000 (64.4%)   HistoryBot        685665/2000000 (34.3%)
ReflectiveBot2         11360/14000 (81.1%)   17832428/28000000 (63.7%)   MetaBot          1322158/2000000 (66.1%)
MarkovBot              10907/14000 (77.9%)   16571873/28000000 (59.2%)   ReflectiveBot2    668048/2000000 (33.4%)
HistoryBot             10648/14000 (76.1%)   16051166/28000000 (57.3%)   ReflectiveBot2   1999346/2000000 (100.0%)
ReflectiveBot           9962/14000 (71.2%)   10011317/28000000 (35.8%)   MetaBot          1359236/2000000 (68.0%)
BiasBot                 9013/14000 (64.4%)   15204683/28000000 (54.3%)   ReflectiveBot     821832/2000000 (41.1%)
RandomDummy             6915/14000 (49.4%)    9333274/28000000 (33.3%)   HistoryBot        667489/2000000 (33.4%)
DecayingFrequencyBot    6220/14000 (44.4%)   14369306/28000000 (51.3%)   BiasBot          1332650/2000000 (66.6%)
FrequencyBot            5429/14000 (38.8%)   12434766/28000000 (44.4%)   HistoryBot       1440573/2000000 (72.0%)
DeBruijnDummy           5085/14000 (36.3%)    8244387/28000000 (29.4%)   ReflectiveBot2   1327345/2000000 (66.4%)
PatternDummy            3966/14000 (28.3%)    6230523/28000000 (22.3%)   HistoryBot       1991299/2000000 (99.6%)
PaperDummy              3945/14000 (28.2%)    5956705/28000000 (21.3%)   ScissorsDummy    2000000/2000000 (100.0%)
FlatBot                 3558/14000 (25.4%)    6865264/28000000 (24.5%)   PaperDummy       1335815/2000000 (66.8%)
RockDummy               1959/14000 (14.0%)    4634808/28000000 (16.6%)   PaperDummy       2000000/2000000 (100.0%)
ScissorsDummy           1958/14000 (14.0%)    5279945/28000000 (18.9%)   RockDummy        2000000/2000000 (100.0%)
```

```text
Playing tournament with:
        1000 round long games
        10000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won               Rounds Won                    Nemesis                Rounds Lost to Nemesis
==============================================================================================================================
MetaBot                109862/140000 (78.5%)    86264030/140000000 (61.6%)   DeBruijnDummy           3350022/10000000 (33.5%)
ReflectiveBot2         106489/140000 (76.1%)    85688303/140000000 (61.2%)   MetaBot                 6645724/10000000 (66.5%)
MarkovBot              103770/140000 (74.1%)    78394612/140000000 (56.0%)   DeBruijnDummy           3353345/10000000 (33.5%)
HistoryBot              96564/140000 (69.0%)    76299125/140000000 (54.5%)   ReflectiveBot2          9993380/10000000 (99.9%)
ReflectiveBot           87308/140000 (62.4%)    50157369/140000000 (35.8%)   MetaBot                 6723015/10000000 (67.2%)
BiasBot                 86799/140000 (62.0%)    75926381/140000000 (54.2%)   ReflectiveBot           3939890/10000000 (39.4%)
DeBruijnDummy           86595/140000 (61.9%)    48182547/140000000 (34.4%)   BiasBot                 3373295/10000000 (33.7%)
RandomDummy             68626/140000 (49.0%)    46656429/140000000 (33.3%)   RockDummy               3336288/10000000 (33.4%)
DecayingFrequencyBot    62098/140000 (44.4%)    71957549/140000000 (51.4%)   BiasBot                 6656693/10000000 (66.6%)
FrequencyBot            53183/140000 (38.0%)    62526558/140000000 (44.7%)   DecayingFrequencyBot    7116872/10000000 (71.2%)
PaperDummy              42882/140000 (30.6%)    29846908/140000000 (21.3%)   ScissorsDummy          10000000/10000000 (100.0%)
PatternDummy            40304/140000 (28.8%)    31448838/140000000 (22.5%)   HistoryBot              9912210/10000000 (99.1%)
FlatBot                 35242/140000 (25.2%)    34717920/140000000 (24.8%)   PaperDummy              6682539/10000000 (66.8%)
RockDummy               22784/140000 (16.3%)    23129328/140000000 (16.5%)   PaperDummy             10000000/10000000 (100.0%)
ScissorsDummy           22778/140000 (16.3%)    26434480/140000000 (18.9%)   RockDummy              10000000/10000000 (100.0%)
```

```text
Playing tournament with:
        100 round long games
        100000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won                 Rounds Won                    Nemesis          Rounds Lost to Nemesis
==========================================================================================================================
MetaBot                1127318/1400000 (80.5%)    83298723/140000000 (59.5%)   BiasBot           3499574/10000000 (35.0%)
BiasBot                1097273/1400000 (78.4%)    76960832/140000000 (55.0%)   ReflectiveBot     3500061/10000000 (35.0%)
ReflectiveBot2         1026745/1400000 (73.3%)    84592442/140000000 (60.4%)   MetaBot           6593356/10000000 (65.9%)
MarkovBot               961912/1400000 (68.7%)    72040833/140000000 (51.5%)   ReflectiveBot     3334814/10000000 (33.3%)
HistoryBot              861673/1400000 (61.5%)    70973817/140000000 (50.7%)   ReflectiveBot2    9933539/10000000 (99.3%)
DeBruijnDummy           743784/1400000 (53.1%)    48894919/140000000 (34.9%)   MetaBot           4033274/10000000 (40.3%)
ReflectiveBot           742017/1400000 (53.0%)    49297994/140000000 (35.2%)   MetaBot           6636009/10000000 (66.4%)
RandomDummy             666218/1400000 (47.6%)    46664889/140000000 (33.3%)   ReflectiveBot2    3335656/10000000 (33.4%)
DecayingFrequencyBot    646695/1400000 (46.2%)    73130858/140000000 (52.2%)   BiasBot           6566960/10000000 (65.7%)
FrequencyBot            540081/1400000 (38.6%)    64118416/140000000 (45.8%)   BiasBot           7017722/10000000 (70.2%)
PaperDummy              426679/1400000 (30.5%)    30637135/140000000 (21.9%)   ScissorsDummy    10000000/10000000 (100.0%)
FlatBot                 403224/1400000 (28.8%)    35881909/140000000 (25.6%)   PaperDummy        6807074/10000000 (68.1%)
PatternDummy            398960/1400000 (28.5%)    33431805/140000000 (23.9%)   HistoryBot        9120806/10000000 (91.2%)
RockDummy               327301/1400000 (23.4%)    23672993/140000000 (16.9%)   PaperDummy       10000000/10000000 (100.0%)
ScissorsDummy           227122/1400000 (16.2%)    26691920/140000000 (19.1%)   RockDummy        10000000/10000000 (100.0%)
```
