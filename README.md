# Rock-Paper-Scissors Bot Fight

I ported [sparemind/RockPaperScissorsBots](https://github.com/sparemind/RockPaperScissorsBots) to Rust.

I don't remember the motivation for doing so.

## Improvements

### History Bots Speedup

`HistoryBot`, `MetaBot`, and `ReflectiveBot` are optimized by using a suffix automaton (SAM) instead of a naive search every time a move is made.

This code also fixes a fencepost error in the original code, which ignores the first character when searching for suffixes that appear earlier.

### Parallelism

Using all CPU cores with multithreading speeds up the program.

### Command Line Options

The number of games and rounds are now specified at runtime, instead of being hardcoded.

## Example Tournament Output

```text
Playing tournament with:
        1000 round long games
        10 game long matches
        14 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won         Rounds Won              Nemesis         Rounds Lost to Nemesis
=======================================================================================================
MarkovBot              101/130 (77.7%)    75073/130000 (57.7%)   MetaBot         33.9%
MetaBot                100/130 (76.9%)    79658/130000 (61.3%)   BiasBot         32.7%
HistoryBot              99/130 (76.2%)    76302/130000 (58.7%)   ReflectiveBot   67.2%
ReflectiveBot           87/130 (66.9%)    50169/130000 (38.6%)   MetaBot         67.3%
DeBruijnDummy           83/130 (63.8%)    44718/130000 (34.4%)   FlatBot         32.8%
BiasBot                 81/130 (62.3%)    72044/130000 (55.4%)   ReflectiveBot   39.6%
RandomDummy             64/130 (49.2%)    43141/130000 (33.2%)   FlatBot         34.1%
DecayingFrequencyBot    63/130 (48.5%)    68517/130000 (52.7%)   BiasBot         66.6%
FrequencyBot            56/130 (43.1%)    60936/130000 (46.9%)   BiasBot         63.9%
PaperDummy              42/130 (32.3%)    30204/130000 (23.2%)   ScissorsDummy   100.0%
PatternDummy            39/130 (30.0%)    30100/130000 (23.2%)   HistoryBot      99.3%
FlatBot                 35/130 (26.9%)    32184/130000 (24.8%)   FrequencyBot    66.5%
ScissorsDummy           21/130 (16.2%)    26488/130000 (20.4%)   RockDummy       100.0%
RockDummy               19/130 (14.6%)    22266/130000 (17.1%)   PaperDummy      100.0%
```
