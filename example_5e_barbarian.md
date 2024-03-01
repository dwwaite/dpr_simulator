# D&D 5E: Barbarian

A run through of different weapon set ups for a barbarian - 1h, 2h with GWM, and dual-wielding.

Running for a 5th level character, variant human lineage so;

1. Starting with a feat and 16 STR
1. Level 4 ASI or feat
1. Extra attack
1. Rage (+2) on all attacks
1. Proficiency (+3)
1. Reckless attack on all rolls

---

## 1H Barbarian

Starting feat: [Fighting Initiate](http://dnd5e.wikidot.com/feat:fighting-initiate) (Dueling)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Longsword (1d8)|
|Hit bonus|7 (4 STR + 3 PROF)|
|Damage modifier|+8 (+4 STR, +2 rage, +2 dueling)|

```bash
./target/release/dpr_simulator --to-hit 1d20A+7 --weapon-details "1d8+8" "1d8+8"
```

---

## 2H Barbarian

Starting feat: [Great Weapon Master](http://dnd5e.wikidot.com/feat:great-weapon-master)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Greastsword (2d6)|
|Hit bonus|2 (+4 STR, +3 PROF, -5 GWM)|
|Damage modifier|+16 (+4 STR, +2 rage, +10 GWM)|

```bash
./target/release/dpr_simulator --to-hit 1d20A+2 --weapon-details "2d6+16" "2d6+16"
```

Starting feat: [Great Weapon Master](http://dnd5e.wikidot.com/feat:great-weapon-master)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|[Polearm master](http://dnd5e.wikidot.com/feat:polearm-master)|
|Weapon|Greastsword (2d6)|
|Hit bonus|1 (+3 STR, +3 PROF, -5 GWM)|
|Damage modifier|+15 (+3 STR, +2 rage, +10 GWM)|

```bash
# First round
./target/release/dpr_simulator --to-hit 1d20A+1 --weapon-details "1d10+15" "1d10+15"

# Subsequent rounds
./target/release/dpr_simulator --to-hit 1d20A+1 --weapon-details "1d10+15" "1d10+15" "1d4+15"
```

---

## Dual-wielding Barbarian

Starting feat: [Dual Wielder](http://dnd5e.wikidot.com/feat:dual-wielder)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Longsword (1d8), Longsword (1d8)|
|Hit bonus|7 (+4 STR, +3 PROF)|
|Damage modifier|MH +6 (+4 STR, +2 rage), OH +2 (+2 rage)|

```bash
# First round
./target/release/dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6"

# Subsequent rounds
./target/release/dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6" "1d8+2"
```

Starting feat: [Fighting Initiate](http://dnd5e.wikidot.com/feat:fighting-initiate) (Two-Weapon Fighting)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Longsword (1d8), Shortsword (1d6)|
|Hit bonus|7 (+4 STR, +3 PROF)|
|Damage modifier|MH +6 (+4 STR, +2 rage), OH +6 (+4 STR, +2 rage)|


```bash
# First round
./target/release/dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6"

# Subsequent rounds
./target/release/dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6" "1d6+6"
```

Starting feat: [Dual Wielder](http://dnd5e.wikidot.com/feat:dual-wielder)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|[Fighting Initiate](http://dnd5e.wikidot.com/feat:fighting-initiate) (Two-Weapon Fighting)|
|Weapon|Longsword (1d8), Longsword (1d8)|
|Hit bonus|6 (+3 STR, +3 PROF)|
|Damage modifier|MH +5 (+3 STR, +2 rage), OH +5 (+3 STR, +2 rage)|

```bash
# First round
./target/release/dpr_simulator --to-hit "1d20A+6" --weapon-details "1d8+5" "1d8+5"

# Subsequent rounds
./target/release/dpr_simulator --to-hit "1d20A+6" --weapon-details "1d8+5" "1d8+5" "1d8+5"
```

---

## Summary

For set ups which use Bonus Action to attack, shown as `First Round / Subsequent Rounds` as the first round requires bonus action to Rage.

|Target AC|1H|2H (GWM)|2H (GWM, PAM)|DW (DW)|DW (TWF)|DW (DW, TWF)|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|12|24.88|38.04|31.82 / 45.19|21.04 / 27.71|21.03 / 30.50|18.69 / 28.04|
|14|23.63|33.44|27.30 / 38.76|19.98 / 26.34|19.99 / 28.98|17.54 / 26.34|
|16|21.88|27.95|21.97 / 31.16|18.52 / 24.44|18.51 / 26.84|16.05 / 24.05|
|18|19.62|21.49|15.84 / 22.41|16.63 / 21.94|16.63 / 24.09|14.13 / 21.19|
|20|16.87|14.16|8.86 / 12.44|14.32 / 18.92|14.32 / 20.74|11.85 / 17.77|

---
