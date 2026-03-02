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

With larger sizes, it still runs quite quickly.

```text
Playing tournament with:
        10000 round long games
        10 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won         Rounds Won                Nemesis          Rounds Lost to Nemesis
==========================================================================================================
ReflectiveBot2         123/140 (87.9%)    934253/1400000 (66.7%)   MetaBot           63964/100000 (64.0%)
MetaBot                114/140 (81.4%)    931686/1400000 (66.5%)   HistoryBot        33647/100000 (33.6%)
MarkovBot              108/140 (77.1%)    886430/1400000 (63.3%)   ReflectiveBot2    34141/100000 (34.1%)
HistoryBot             104/140 (74.3%)    834634/1400000 (59.6%)   ReflectiveBot2    99993/100000 (100.0%)
ReflectiveBot           90/140 (64.3%)    499212/1400000 (35.7%)   ReflectiveBot2    79992/100000 (80.0%)
BiasBot                 85/140 (60.7%)    736652/1400000 (52.6%)   ReflectiveBot     44800/100000 (44.8%)
RandomDummy             71/140 (50.7%)    466435/1400000 (33.3%)   RockDummy         33493/100000 (33.5%)
DecayingFrequencyBot    62/140 (44.3%)    712020/1400000 (50.9%)   BiasBot           66659/100000 (66.7%)
FrequencyBot            58/140 (41.4%)    610388/1400000 (43.6%)   MarkovBot         87131/100000 (87.1%)
DeBruijnDummy           52/140 (37.1%)    353279/1400000 (25.2%)   MetaBot           93206/100000 (93.2%)
PaperDummy              43/140 (30.7%)    293860/1400000 (21.0%)   ScissorsDummy    100000/100000 (100.0%)
PatternDummy            41/140 (29.3%)    328094/1400000 (23.4%)   HistoryBot        99925/100000 (99.9%)
FlatBot                 34/140 (24.3%)    336353/1400000 (24.0%)   PaperDummy        66870/100000 (66.9%)
RockDummy               23/140 (16.4%)    230297/1400000 (16.4%)   PaperDummy       100000/100000 (100.0%)
ScissorsDummy           18/140 (12.9%)    267215/1400000 (19.1%)   RockDummy        100000/100000 (100.0%)
```

```text
Playing tournament with:
        8000 round long games
        100 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won           Rounds Won                  Nemesis          Rounds Lost to Nemesis
==============================================================================================================
MetaBot                1192/1400 (85.1%)    7434565/11200000 (66.4%)   HistoryBot       269325/800000 (33.7%)
ReflectiveBot2         1134/1400 (81.0%)    7339812/11200000 (65.5%)   MetaBot          510148/800000 (63.8%)
MarkovBot              1083/1400 (77.4%)    7035015/11200000 (62.8%)   ReflectiveBot2   269198/800000 (33.6%)
HistoryBot             1047/1400 (74.8%)    6660901/11200000 (59.5%)   ReflectiveBot2   799938/800000 (100.0%)
BiasBot                 937/1400 (66.9%)    6043597/11200000 (54.0%)   ReflectiveBot    355360/800000 (44.4%)
ReflectiveBot           879/1400 (62.8%)    4024525/11200000 (35.9%)   MetaBot          544756/800000 (68.1%)
RandomDummy             681/1400 (48.6%)    3734857/11200000 (33.3%)   BiasBot          267362/800000 (33.4%)
DecayingFrequencyBot    608/1400 (43.4%)    5706867/11200000 (51.0%)   BiasBot          533266/800000 (66.7%)
FrequencyBot            561/1400 (40.1%)    4918517/11200000 (43.9%)   MarkovBot        662388/800000 (82.8%)
DeBruijnDummy           511/1400 (36.5%)    2854041/11200000 (25.5%)   ReflectiveBot2   732821/800000 (91.6%)
PaperDummy              426/1400 (30.4%)    2367059/11200000 (21.1%)   ScissorsDummy    800000/800000 (100.0%)
PatternDummy            421/1400 (30.1%)    2519213/11200000 (22.5%)   HistoryBot       799153/800000 (99.9%)
FlatBot                 356/1400 (25.4%)    2688509/11200000 (24.0%)   PaperDummy       533890/800000 (66.7%)
RockDummy               240/1400 (17.1%)    1849765/11200000 (16.5%)   PaperDummy       800000/800000 (100.0%)
ScissorsDummy           234/1400 (16.7%)    2118126/11200000 (18.9%)   RockDummy        800000/800000 (100.0%)
```

```text
Playing tournament with:
        3000 round long games
        1000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won             Rounds Won                  Nemesis          Rounds Lost to Nemesis
==================================================================================================================
ReflectiveBot2         11727/14000 (83.8%)   27149148/42000000 (64.6%)   MetaBot          1955656/3000000 (65.2%)
MetaBot                11634/14000 (83.1%)   27379535/42000000 (65.2%)   HistoryBot       1012265/3000000 (33.7%)
HistoryBot             10972/14000 (78.4%)   24432795/42000000 (58.2%)   ReflectiveBot2   2999331/3000000 (100.0%)
MarkovBot              10915/14000 (78.0%)   25410251/42000000 (60.5%)   ReflectiveBot2   1003074/3000000 (33.4%)
ReflectiveBot           8740/14000 (62.4%)   15009169/42000000 (35.7%)   ReflectiveBot2   2054316/3000000 (68.5%)
BiasBot                 8700/14000 (62.1%)   22803451/42000000 (54.3%)   ReflectiveBot    1253832/3000000 (41.8%)
RandomDummy             6958/14000 (49.7%)   14000709/42000000 (33.3%)   MarkovBot        1000560/3000000 (33.4%)
DecayingFrequencyBot    6208/14000 (44.3%)   21530120/42000000 (51.3%)   BiasBot          1998677/3000000 (66.6%)
FrequencyBot            5630/14000 (40.2%)   18677779/42000000 (44.5%)   MarkovBot        2221865/3000000 (74.1%)
DeBruijnDummy           5064/14000 (36.2%)   11637885/42000000 (27.7%)   ReflectiveBot2   2326749/3000000 (77.6%)
PaperDummy              4299/14000 (30.7%)    8902557/42000000 (21.2%)   ScissorsDummy    3000000/3000000 (100.0%)
PatternDummy            4064/14000 (29.0%)    9400262/42000000 (22.4%)   HistoryBot       2991263/3000000 (99.7%)
FlatBot                 3550/14000 (25.4%)   10212918/42000000 (24.3%)   PaperDummy       2001577/3000000 (66.7%)
ScissorsDummy           2249/14000 (16.1%)    7911161/42000000 (18.8%)   RockDummy        3000000/3000000 (100.0%)
RockDummy               2245/14000 (16.0%)    6904587/42000000 (16.4%)   PaperDummy       3000000/3000000 (100.0%)
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
