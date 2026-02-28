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
MetaBot                104/130 (80.0%)    79886/130000 (61.5%)   DeBruijnDummy    3345/10000 (33.5%)
MarkovBot              104/130 (80.0%)    75186/130000 (57.8%)   ReflectiveBot    3361/10000 (33.6%)
HistoryBot              92/130 (70.8%)    76264/130000 (58.7%)   ReflectiveBot    6745/10000 (67.5%)
ReflectiveBot           86/130 (66.2%)    51282/130000 (39.4%)   MetaBot          6709/10000 (67.1%)
BiasBot                 79/130 (60.8%)    71859/130000 (55.3%)   ReflectiveBot    3926/10000 (39.3%)
DeBruijnDummy           76/130 (58.5%)    44719/130000 (34.4%)   MarkovBot        3392/10000 (33.9%)
RandomDummy             62/130 (47.7%)    42949/130000 (33.0%)   FrequencyBot     3428/10000 (34.3%)
DecayingFrequencyBot    58/130 (44.6%)    67670/130000 (52.1%)   BiasBot          6659/10000 (66.6%)
FrequencyBot            55/130 (42.3%)    61326/130000 (47.2%)   BiasBot          7186/10000 (71.9%)
PatternDummy            46/130 (35.4%)    30549/130000 (23.5%)   HistoryBot       9925/10000 (99.2%)
PaperDummy              44/130 (33.8%)    29814/130000 (22.9%)   ScissorsDummy   10000/10000 (100.0%)
FlatBot                 39/130 (30.0%)    32210/130000 (24.8%)   PaperDummy       6672/10000 (66.7%)
ScissorsDummy           24/130 (18.5%)    25885/130000 (19.9%)   RockDummy       10000/10000 (100.0%)
RockDummy               19/130 (14.6%)    22442/130000 (17.3%)   PaperDummy      10000/10000 (100.0%)
```
