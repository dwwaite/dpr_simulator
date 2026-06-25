# D&D 5.5E: Barbarian

A run through of different weapon set ups for a barbarian - 1h, 2h with GWM, and dual-wielding.

Running for a 5th level character;

1. Starting with 16 STR
1. Level 4 ASI or feat
1. Extra attack
1. Rage (+2) on all attacks
1. Proficiency (+3)
1. Reckless attack on all rolls

---

## 1H Barbarian

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Longsword (1d8)|
|Hit bonus|7 (+4 STR, +3 PROF)|
|Damage modifier|+6 (+4 STR, +2 rage)|

```bash
dpr_simulator --to-hit "1d20[kh2]+7" --weapon-details "1d8+6" "1d8+6"
```

---

## 2H Barbarian

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Greastsword (2d6)|
|Hit bonus|7 (+4 STR, +3 PROF)|
|Damage modifier|+6 (+4 STR, +2 rage)|

```bash
dpr_simulator --to-hit "1d20[kh2]+7" --weapon-details "2d6+6" "2d6+6"
```


|Feature|Value|
|:---|:---:|
|Level 4 ASI|[Great Weapon Master](http://dnd2024.wikidot.com/feat:great-weapon-master)|
|Weapon|Greastsword (2d6)|
|Hit bonus|6 (+3 STR, +3 PROF)|
|Damage modifier|+8 (+3 STR, +2 rage, +3 GWM)|

```bash
dpr_simulator --to-hit "1d20[kh2]+6" --weapon-details "2d6+8" "2d6+8"
```

|Feature|Value|
|:---|:---:|
|Level 4 ASI|[Polearm master](http://dnd2024.wikidot.com/feat:polearm-master)|
|Weapon|Glaive (1d10)|
|Hit bonus|6 (+3 STR, +3 PROF)|
|Damage modifier|+5 (+3 STR, +2 rage)|

```bash
# First round - Bonus Action to Rage
dpr_simulator --to-hit "1d20[kh2]+6" --weapon-details "1d10+5" "1d10+5"

# Subsequent rounds - Bonus Action to attack
dpr_simulator --to-hit "1d20[kh2]+6" --weapon-details "1d10+5" "1d10+5" "1d4+5"
```

---

## Dual-wielding Barbarian

|Feature|Value|
|:---|:---:|
|Level 4 ASI|+2 STR|
|Weapon|Shortsword (1d6), Shortsword (1d6)|
|Hit bonus|6 (+4 STR, +3 PROF)|
|Damage modifier|MH +6 (+4 STR, +2 rage), OH +2 (+2 rage)|

```bash
# First round - Bonus Action to Rage
dpr_simulator --to-hit "1d20[kh2]+7" --weapon-details "1d6+6" "1d6+6"

# Subsequent rounds - Bonus Action to attack
dpr_simulator --to-hit "1d20[kh2]+7" --weapon-details "1d6+6" "1d6+6" "1d6+2"
```

|Feature|Value|
|:---|:---:|
|Level 4 ASI|[Dual Wielder](http://dnd2024.wikidot.com/feat:dual-wielder)|
|Weapon|Shortsword (1d6), Shortsword (1d6)|
|Hit bonus|6 (+3 STR, +3 PROF)|
|Damage modifier|MH +5 (+3 STR, +2 rage), OH +2 (+2 rage)|

```bash
# First round - Bonus Action to Rage
dpr_simulator --to-hit "1d20[kh2]+6" --weapon-details "1d6+5" "1d6+5" "1d6+2"

# Subsequent rounds - Bonus Action to attack
dpr_simulator --to-hit "1d20[kh2]+6" --weapon-details "1d8+5" "1d8+5" "1d6+2" "1d6+2"
```

---

## Summary

For set ups which use Bonus Action to attack, shown as `First Round / Subsequent Rounds` as the first round requires bonus action to Rage.

|Build|Target AC<br />12|<br />14|<br />16|<br />18|<br />20|
|:---|:---:|:---:|:---:|:---:|:---:|
|1H|22.1|21.7|20.9|19.7|17.7|
|2H|21.8|21.5|20.8|19.6|17.9|
|2H (GWM)|25.6|25.0|23.9|22.2|19.7|
|2H (PAM), round 1|22.2|21.7|20.7|19.1|16.8|
|2H (PAM), round 2+|30.0|29.2|27.8|25.7|22.6|
|Dual wielding, round 1|19.8|19.5|18.8|17.6|15.9|
|Dual wielding, round 2+|25.8|25.3|24.4|22.9|20.7|
|Dual wielding (DW), round 1|23.6|23.0|21.9|20.3|17.8|
|Dual wielding (DW), round 2+|31.8|31.0|29.5|27.3|24.0|

---
