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
