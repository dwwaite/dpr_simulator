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
dpr_simulator --to-hit 1d20A+7 --weapon-details "1d8+8" "1d8+8"
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
dpr_simulator --to-hit 1d20A+2 --weapon-details "2d6+16" "2d6+16"
```

Starting feat: [Great Weapon Master](http://dnd5e.wikidot.com/feat:great-weapon-master)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|[Polearm master](http://dnd5e.wikidot.com/feat:polearm-master)|
|Weapon|Greastsword (2d6)|
|Hit bonus|1 (+3 STR, +3 PROF, -5 GWM)|
|Damage modifier|+15 (+3 STR, +2 rage, +10 GWM)|

```bash
# First round - Bonus Action to Rage
dpr_simulator --to-hit 1d20A+1 --weapon-details "1d10+15" "1d10+15"

# Subsequent rounds - Bonus Action to attack
dpr_simulator --to-hit 1d20A+1 --weapon-details "1d10+15" "1d10+15" "1d4+15"
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
# First round - Bonus Action to Rage
dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6"

# Subsequent rounds - Bonus Action to attack
dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6" "1d8+2"
```

Starting feat: [Fighting Initiate](http://dnd5e.wikidot.com/feat:fighting-initiate) (Two-Weapon Fighting)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Longsword (1d8), Shortsword (1d6)|
|Hit bonus|7 (+4 STR, +3 PROF)|
|Damage modifier|MH +6 (+4 STR, +2 rage), OH +6 (+4 STR, +2 rage)|


```bash
# First round - Bonus Action to Rage
dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6"

# Subsequent rounds - Bonus Action to attack
dpr_simulator --to-hit "1d20A+7" --weapon-details "1d8+6" "1d8+6" "1d6+6"
```

Starting feat: [Dual Wielder](http://dnd5e.wikidot.com/feat:dual-wielder)

|Feature|Value|
|:---|:---:|
|Level 4 ASI|[Fighting Initiate](http://dnd5e.wikidot.com/feat:fighting-initiate) (Two-Weapon Fighting)|
|Weapon|Longsword (1d8), Longsword (1d8)|
|Hit bonus|6 (+3 STR, +3 PROF)|
|Damage modifier|MH +5 (+3 STR, +2 rage), OH +5 (+3 STR, +2 rage)|

```bash
# First round - Bonus Action to Rage
dpr_simulator --to-hit "1d20A+6" --weapon-details "1d8+5" "1d8+5"

# Subsequent rounds - Bonus Action to attack
dpr_simulator --to-hit "1d20A+6" --weapon-details "1d8+5" "1d8+5" "1d8+5"
```

---

## Summary

For set ups which use Bonus Action to attack, shown as `First Round / Subsequent Rounds` as the first round requires bonus action to Rage.

|Build|Target AC<br />12|<br />14|<br />16|<br />18|<br />20|
|:---:|:---:|:---:|:---:|:---:|:---:|
|1H|24.88|23.63|21.88|19.62|16.87|
|2H (GWM)|38.04|33.44|27.95|21.49|14.1|
|2H (GWM + PAM), round 1|31.82|27.30|21.97|15.84|8.86|
|2H (GWM + PAM), round 2+|45.19|38.76|31.16|22.41|12.44|
|Dual wielding (DW), round 1|21.04|19.98|18.52|16.63|14.32|
|Dual wielding (DW), round 2+|27.71|26.34|24.44|21.94|18.92|
|Dual wielding (TWF), round 1|21.03|19.99|18.51|16.63|14.32|
|Dual wielding (TWF), round 2+|30.50|28.98|26.84|24.09|20.74|
|Dual wielding (DW + TWF), round 1|18.69|17.54|16.05|14.13|11.85|
|Dual wielding (DW + TWF), round 2+|28.04|26.34|24.05|21.19|17.77|

---
