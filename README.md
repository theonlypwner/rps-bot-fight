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
| SAM       | O(n)          | O(n^2)     | O(n)  |
| SAM+LCT   | O(log n)      | O(n log n) | O(n)  |

A LCT improves the time complexity, but it has a higher constant factor.
The hybrid algorithm initially uses only the SAM but later builds and uses the LCT when the tree becomes deep enough.

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
        240000 round long games
        10 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won         Rounds Won                  Nemesis          Rounds Lost to Nemesis
==============================================================================================================
ReflectiveBot2         119/140 (85.0%)   23182034/33600000 (69.0%)   MetaBot          1527044/2400000 (63.6%)
HistoryBot             115/140 (82.1%)   20403924/33600000 (60.7%)   ReflectiveBot2   2399993/2400000 (100.0%)
MetaBot                112/140 (80.0%)   22718326/33600000 (67.6%)   BiasBot           802453/2400000 (33.4%)
MarkovBot               98/140 (70.0%)   23517619/33600000 (70.0%)   ReflectiveBot2   1391356/2400000 (58.0%)
BiasBot                 92/140 (65.7%)   17604208/33600000 (52.4%)   MarkovBot        1827465/2400000 (76.1%)
ReflectiveBot           88/140 (62.9%)   12242279/33600000 (36.4%)   ReflectiveBot2   1919992/2400000 (80.0%)
DecayingFrequencyBot    65/140 (46.4%)   16493248/33600000 (49.1%)   MarkovBot        2305757/2400000 (96.1%)
RandomDummy             65/140 (46.4%)   11198208/33600000 (33.3%)   MarkovBot         800825/2400000 (33.4%)
FrequencyBot            56/140 (40.0%)   14490788/33600000 (43.1%)   MarkovBot        2386095/2400000 (99.4%)
DeBruijnDummy           50/140 (35.7%)    8132739/33600000 (24.2%)   ReflectiveBot2   2393262/2400000 (99.7%)
PatternDummy            46/140 (32.9%)    7714139/33600000 (23.0%)   HistoryBot       2399917/2400000 (100.0%)
PaperDummy              46/140 (32.9%)    7117492/33600000 (21.2%)   ScissorsDummy    2400000/2400000 (100.0%)
FlatBot                 36/140 (25.7%)    7790233/33600000 (23.2%)   PaperDummy       1599748/2400000 (66.7%)
ScissorsDummy           24/140 (17.1%)    6516416/33600000 (19.4%)   RockDummy        2400000/2400000 (100.0%)
RockDummy               23/140 (16.4%)    5508546/33600000 (16.4%)   PaperDummy       2400000/2400000 (100.0%)
```

```text
Playing tournament with:
        40000 round long games
        100 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won           Rounds Won                  Nemesis          Rounds Lost to Nemesis
================================================================================================================
MetaBot                1160/1400 (82.9%)   37671866/56000000 (67.3%)   HistoryBot       1339014/4000000 (33.5%)
ReflectiveBot2         1152/1400 (82.3%)   36901307/56000000 (65.9%)   MetaBot          2557568/4000000 (63.9%)
HistoryBot             1068/1400 (76.3%)   33763314/56000000 (60.3%)   ReflectiveBot2   3999935/4000000 (100.0%)
MarkovBot              1050/1400 (75.0%)   37297696/56000000 (66.6%)   ReflectiveBot2   1516553/4000000 (37.9%)
ReflectiveBot          1004/1400 (71.7%)   20233774/56000000 (36.1%)   MetaBot          2730536/4000000 (68.3%)
BiasBot                 906/1400 (64.7%)   30029776/56000000 (53.6%)   ReflectiveBot    1893862/4000000 (47.3%)
RandomDummy             709/1400 (50.6%)   18666728/56000000 (33.3%)   MarkovBot        1334665/4000000 (33.4%)
DecayingFrequencyBot    620/1400 (44.3%)   27796440/56000000 (49.6%)   MarkovBot        3081253/4000000 (77.0%)
FrequencyBot            580/1400 (41.4%)   24304506/56000000 (43.4%)   MarkovBot        3867406/4000000 (96.7%)
DeBruijnDummy           509/1400 (36.4%)   13657992/56000000 (24.4%)   ReflectiveBot2   3932733/4000000 (98.3%)
PaperDummy              413/1400 (29.5%)   12067119/56000000 (21.5%)   ScissorsDummy    4000000/4000000 (100.0%)
PatternDummy            406/1400 (29.0%)   12806024/56000000 (22.9%)   HistoryBot       3999128/4000000 (100.0%)
FlatBot                 350/1400 (25.0%)   13094771/56000000 (23.4%)   FrequencyBot     2667453/4000000 (66.7%)
RockDummy               191/1400 (13.6%)    9242833/56000000 (16.5%)   PaperDummy       4000000/4000000 (100.0%)
ScissorsDummy           188/1400 (13.4%)   10503413/56000000 (18.8%)   RockDummy        4000000/4000000 (100.0%)
```

```text
Playing tournament with:
        7000 round long games
        1000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won             Rounds Won                  Nemesis          Rounds Lost to Nemesis
==================================================================================================================
MetaBot                11625/14000 (83.0%)   64922959/98000000 (66.2%)   HistoryBot       2362244/7000000 (33.7%)
ReflectiveBot2         11150/14000 (79.6%)   63983735/98000000 (65.3%)   MetaBot          4469107/7000000 (63.8%)
HistoryBot             10982/14000 (78.4%)   58156811/98000000 (59.3%)   ReflectiveBot2   6999360/7000000 (100.0%)
MarkovBot              10752/14000 (76.8%)   61254231/98000000 (62.5%)   ReflectiveBot2   2353879/7000000 (33.6%)
BiasBot                 9332/14000 (66.7%)   53052224/98000000 (54.1%)   ReflectiveBot    3092516/7000000 (44.2%)
ReflectiveBot           9243/14000 (66.0%)   35141343/98000000 (35.9%)   MetaBot          4760435/7000000 (68.0%)
RandomDummy             6954/14000 (49.7%)   32669377/98000000 (33.3%)   ScissorsDummy    2335272/7000000 (33.4%)
DecayingFrequencyBot    6173/14000 (44.1%)   50071172/98000000 (51.1%)   BiasBot          4665677/7000000 (66.7%)
FrequencyBot            5574/14000 (39.8%)   43377425/98000000 (44.3%)   MarkovBot        5674238/7000000 (81.1%)
DeBruijnDummy           4774/14000 (34.1%)   25162867/98000000 (25.7%)   ReflectiveBot2   6327481/7000000 (90.4%)
PaperDummy              4271/14000 (30.5%)   20784820/98000000 (21.2%)   ScissorsDummy    7000000/7000000 (100.0%)
PatternDummy            4051/14000 (28.9%)   21874499/98000000 (22.3%)   HistoryBot       6991246/7000000 (99.9%)
FlatBot                 3570/14000 (25.5%)   23464973/98000000 (23.9%)   PaperDummy       4668965/7000000 (66.7%)
ScissorsDummy           2282/14000 (16.3%)   18526514/98000000 (18.9%)   RockDummy        7000000/7000000 (100.0%)
RockDummy               2261/14000 (16.1%)   16083639/98000000 (16.4%)   PaperDummy       7000000/7000000 (100.0%)
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

```text
Playing tournament with:
        10 round long games
        400000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won                 Rounds Won                  Nemesis                Rounds Lost to Nemesis
============================================================================================================================
BiasBot                3908901/5600000 (69.8%)   29264684/56000000 (52.3%)   DeBruijnDummy          1732932/4000000 (43.3%)
MetaBot                3785895/5600000 (67.6%)   27227071/56000000 (48.6%)   DecayingFrequencyBot   1466970/4000000 (36.7%)
ReflectiveBot2         3616090/5600000 (64.6%)   30273366/56000000 (54.1%)   MetaBot                2131899/4000000 (53.3%)
DeBruijnDummy          3080571/5600000 (55.0%)   20249046/56000000 (36.2%)   PaperDummy             2000000/4000000 (50.0%)
DecayingFrequencyBot   3012792/5600000 (53.8%)   28677672/56000000 (51.2%)   BiasBot                2266910/4000000 (56.7%)
HistoryBot             2592515/5600000 (46.3%)   24101347/56000000 (43.0%)   ReflectiveBot2         3733315/4000000 (93.3%)
FrequencyBot           2422413/5600000 (43.3%)   26176807/56000000 (46.7%)   ReflectiveBot2         2533578/4000000 (63.3%)
MarkovBot              2376007/5600000 (42.4%)   18671702/56000000 (33.3%)   MetaBot                1334498/4000000 (33.4%)
RandomDummy            2375296/5600000 (42.4%)   18668272/56000000 (33.3%)   PaperDummy             1335912/4000000 (33.4%)
PatternDummy           2321492/5600000 (41.5%)   18479917/56000000 (33.0%)   HistoryBot             1432335/4000000 (35.8%)
ReflectiveBot          2293667/5600000 (41.0%)   19447668/56000000 (34.7%)   MetaBot                2533536/4000000 (63.3%)
FlatBot                1908770/5600000 (34.1%)   14996476/56000000 (26.8%)   PaperDummy             3019513/4000000 (75.5%)
PaperDummy             1843729/5600000 (32.9%)   15286789/56000000 (27.3%)   ScissorsDummy          4000000/4000000 (100.0%)
RockDummy              1457956/5600000 (26.0%)   11601964/56000000 (20.7%)   PaperDummy             4000000/4000000 (100.0%)
ScissorsDummy          1043518/5600000 (18.6%)   11913626/56000000 (21.3%)   RockDummy              4000000/4000000 (100.0%)
```

```text
Playing tournament with:
        2 round long games
        400000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won                 Rounds Won                  Nemesis          Rounds Lost to Nemesis
====================================================================================================================
DeBruijnDummy          2267327/5600000 (40.5%)    4667787/11200000 (41.7%)   PaperDummy       400000/800000 (50.0%)
ReflectiveBot          2266385/5600000 (40.5%)    4797483/11200000 (42.8%)   ReflectiveBot2   267182/800000 (33.4%)
ReflectiveBot2         2265995/5600000 (40.5%)    4801673/11200000 (42.9%)   PaperDummy       267368/800000 (33.4%)
PaperDummy             2000516/5600000 (35.7%)    3867552/11200000 (34.5%)   ScissorsDummy    800000/800000 (100.0%)
FlatBot                1866954/5600000 (33.3%)    3599663/11200000 (32.1%)   PaperDummy       532984/800000 (66.6%)
MarkovBot              1865528/5600000 (33.3%)    3731544/11200000 (33.3%)   RandomDummy      267346/800000 (33.4%)
RandomDummy            1865242/5600000 (33.3%)    3732037/11200000 (33.3%)   HistoryBot       267372/800000 (33.4%)
PatternDummy           1864723/5600000 (33.3%)    3731020/11200000 (33.3%)   RockDummy        267906/800000 (33.5%)
RockDummy              1468510/5600000 (26.2%)    3068927/11200000 (27.4%)   PaperDummy       800000/800000 (100.0%)
MetaBot                1468365/5600000 (26.2%)    3734698/11200000 (33.3%)   MarkovBot        267033/800000 (33.4%)
DecayingFrequencyBot   1467533/5600000 (26.2%)    4134000/11200000 (36.9%)   ReflectiveBot2   533445/800000 (66.7%)
HistoryBot             1467309/5600000 (26.2%)    4133011/11200000 (36.9%)   ReflectiveBot    533529/800000 (66.7%)
FrequencyBot           1466318/5600000 (26.2%)    4131245/11200000 (36.9%)   DeBruijnDummy    533669/800000 (66.7%)
BiasBot                1465770/5600000 (26.2%)    4131188/11200000 (36.9%)   DeBruijnDummy    533906/800000 (66.7%)
ScissorsDummy          1333488/5600000 (23.8%)    3465090/11200000 (30.9%)   RockDummy        800000/800000 (100.0%)
```

```text
Playing tournament with:
        1 round long games
        400000 game long matches
        15 competitors

Tournament Progress: 0% 10% 20% 30% 40% 50% 60% 70% 80% 90% 100%

Name                   Games Won                 Rounds Won                Nemesis                Rounds Lost to Nemesis
========================================================================================================================
ScissorsDummy          2265525/5600000 (40.5%)   2265525/5600000 (40.5%)   RockDummy              400000/400000 (100.0%)
ReflectiveBot2         1868044/5600000 (33.4%)   1868044/5600000 (33.4%)   PaperDummy             133661/400000 (33.4%)
PatternDummy           1867760/5600000 (33.4%)   1867760/5600000 (33.4%)   FlatBot                133712/400000 (33.4%)
RockDummy              1867567/5600000 (33.3%)   1867567/5600000 (33.3%)   PaperDummy             400000/400000 (100.0%)
FlatBot                1866963/5600000 (33.3%)   1866963/5600000 (33.3%)   ReflectiveBot2         133759/400000 (33.4%)
DeBruijnDummy          1866680/5600000 (33.3%)   1866680/5600000 (33.3%)   ScissorsDummy          400000/400000 (100.0%)
MetaBot                1866603/5600000 (33.3%)   1866603/5600000 (33.3%)   DeBruijnDummy          133967/400000 (33.5%)
ReflectiveBot          1866360/5600000 (33.3%)   1866360/5600000 (33.3%)   DecayingFrequencyBot   133786/400000 (33.4%)
FrequencyBot           1866355/5600000 (33.3%)   1866355/5600000 (33.3%)   ReflectiveBot2         133775/400000 (33.4%)
PaperDummy             1866224/5600000 (33.3%)   1866224/5600000 (33.3%)   ScissorsDummy          400000/400000 (100.0%)
RandomDummy            1866117/5600000 (33.3%)   1866117/5600000 (33.3%)   PaperDummy             133699/400000 (33.4%)
DecayingFrequencyBot   1865518/5600000 (33.3%)   1865518/5600000 (33.3%)   MetaBot                133796/400000 (33.4%)
BiasBot                1865395/5600000 (33.3%)   1865395/5600000 (33.3%)   PatternDummy           133893/400000 (33.5%)
MarkovBot              1864965/5600000 (33.3%)   1864965/5600000 (33.3%)   DecayingFrequencyBot   133832/400000 (33.5%)
HistoryBot             1864707/5600000 (33.3%)   1864707/5600000 (33.3%)   RandomDummy            133821/400000 (33.5%)
```
