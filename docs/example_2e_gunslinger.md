# Pathfinder 2E: Gunslinger

A run through of different weapon set ups for a [Gunslinger](https://2e.aonprd.com/Classes.aspx?ID=20).

* [Pistolero](https://2e.aonprd.com/Ways.aspx?ID=2) with 1 weapon, using [Pistol Twirl](https://2e.aonprd.com/Feats.aspx?ID=3162) to soften defenses and [Munitions Machinist](https://2e.aonprd.com/Feats.aspx?ID=3172) for some extra damage.
  * Picking this over an [Alchemical Shot](https://2e.aonprd.com/Feats.aspx?ID=3165) build because I don't like the misfire risk, and once specialist ammunition moves to `2d4` it's better damage anyway.
* [Pistolero](https://2e.aonprd.com/Ways.aspx?ID=2) dual-wielding using [Paired Shots](https://2e.aonprd.com/Feats.aspx?ID=3168)
* [Drifter](https://2e.aonprd.com/Ways.aspx?ID=1) using a pistol/sword combo, leading with pistol and using [Sword and Pistol](https://2e.aonprd.com/Feats.aspx?ID=3159)
* Comparison - [Rogue](https://2e.aonprd.com/Classes.aspx?ID=37)
  * [Thief racket](https://2e.aonprd.com/Rackets.aspx)
  * A different way to get the character concept with what is probably more consistent damage.
* Comparison - [Fighter](https://2e.aonprd.com/Classes.aspx?ID=35)
  * Sword and Board build, no particular feats.
  * An approximation of the DPR a martial could pull at the same level.

## Contents

1. [Setting the stage](#setting-the-stage)
1. [Pistol Twirl build](#pistol-twirl-build)
1. [Paired Shots build](#paired-shots-build)
1. [Drifter build](#drifter-build)
1. [Summary](#summary)

Running for a 9th level character, with level-appropriate runes.

Based on the [previous check](./example_2e_mutagenist.md) the median AC at this level is 28.

---

## Pistol Twirl build

[Pistol Twirl](https://2e.aonprd.com/Feats.aspx?ID=3162) allows a ranged [Feint](https://2e.aonprd.com/Actions.aspx?ID=48) - this will manifest as a +2 to hit, rather than decrease the AC values for target, just for ease of comparison at the end. This also means that the reload can be performed as a [Demoaralize](https://2e.aonprd.com/Actions.aspx?ID=2395&Redirected=1) stacking with the off-guard from feinting (another +1 to hit).

[Munitions Machinist](https://2e.aonprd.com/Feats.aspx?ID=3172) allows the creation of nine pieces of ammunition per day, which usually site around `2d4` worth of damage.

The rotation here assumes a single shot per round, with some combination of feint, reload, or movement using the remaining actions.

* Hit bonus = `+20`
  * +4 DEX
  * +9 Level
  * +6 Proficiency (Master)
  * [+1 Weapon Potency](https://2e.aonprd.com/Equipment.aspx?ID=2830)
  * Offguard (+2)
  * Target frightened (+1)
* Damage = `2f6~10+4`
  * [Dueling pistol](https://2e.aonprd.com/Weapons.aspx?ID=201) (`2d6`, with `Fatal d10`)
  * [Striking rune](https://2e.aonprd.com/Equipment.aspx?ID=2829)
  * +1 Single expertise (Circumstance)
  * +3 Weapon specialization (Untyped bonus)
  * +`2d4` specialist ammo, on occassion (lands as persistent, not on hit)

```bash
# Regular ammo
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20+3" --weapon-details "2f6~10+4"

# Specialist ammo
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20+3" --weapon-details "2f6~10,2d4+4"

# Regular ammo, Off-guard, no Frightened
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20+2" --weapon-details "2f6~10+4"

# Regular ammo, no AC debuffs
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20" --weapon-details "2f6~10+4"
```

---

## Paired Shots build

[Paired Shots](https://2e.aonprd.com/Feats.aspx?ID=3168) - Two attacks without factoring MAP. However, there will be rounds with no shot fired. Can't simulate that, but just be aware that the rotation for this build is basically:

1. Attack (2 actions), reload x1
1. Reload x1, Attack (2 actions)
1. Reload, reload, -

Which is extremely optimistic. Will report the raw numbers, and then a 3-round average and assume that the character never needs to move...

* Hit bonus = `+20`
  * +4 DEX
  * +9 Level
  * +6 Proficiency (Master)
  * [+1 Weapon Potency](https://2e.aonprd.com/Equipment.aspx?ID=2830)
  * Offguard (+2)
* Damage = `2f6~10+4`
  * [Dueling pistol](https://2e.aonprd.com/Weapons.aspx?ID=201) (`2d6`, with `Fatal d10`)
  * [Striking rune](https://2e.aonprd.com/Equipment.aspx?ID=2829)
  * +1 Single expertise (Circumstance)
  * +3 Weapon specialization (Untyped bonus)

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20+2" "1d20+20+2" --weapon-details "2f6~10+4"
```

---

## Drifter build

In practice you can use [Sword and Pistol](https://2e.aonprd.com/Feats.aspx?ID=3159) in either order, but shot/stab is the higher DPR option.

Assuming a character with +2 in STR, and using a [Shortsword](https://2e.aonprd.com/Weapons.aspx?ID=43) for the [Agile](https://2e.aonprd.com/Traits.aspx?ID=170) and [Finesse](https://2e.aonprd.com/Traits.aspx?ID=179) traits.

* Hit bonus (gun) = `+20`
  * +4 DEX
  * +9 Level
  * +6 Proficiency (Master)
  * [+1 Weapon Potency](https://2e.aonprd.com/Equipment.aspx?ID=2830)
* Damage (gun) = `2f6~10+4`
  * [Dueling pistol](https://2e.aonprd.com/Weapons.aspx?ID=201) (`2d6`, with `Fatal d10`)
  * [Striking rune](https://2e.aonprd.com/Equipment.aspx?ID=2829)
  * +1 Single expertise (Circumstance)
  * +3 Weapon specialization (Untyped bonus)

* Hit bonus (sword) = `+18`
  * +4 DEX
  * +9 Level
  * +4 Proficiency (Expert)
  * [+1 Weapon Potency](https://2e.aonprd.com/Equipment.aspx?ID=2830)
* Damage (sword) = `1d6+2`
  * [Dueling pistol](https://2e.aonprd.com/Weapons.aspx?ID=398) (`2d6`)
  * [Striking rune](https://2e.aonprd.com/Equipment.aspx?ID=2829)
  * +2 STR

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20" "1d20+18-4" --weapon-details "2f6~10+4" "2d6+2"
```

---

## Rogue

[Thief racket](https://2e.aonprd.com/Rackets.aspx), assuming a constant source of off-guard from positioning. However, as a skirmisher assuming only a single attack per round, with a round like:

1. Position for off-guard
1. Attack
1. Move to safety

Using a [Shortsword](https://2e.aonprd.com/Weapons.aspx?ID=43) for the [Agile](https://2e.aonprd.com/Traits.aspx?ID=170) and [Finesse](https://2e.aonprd.com/Traits.aspx?ID=179) traits.

* Hit bonus = `+18`
  * +4 DEX
  * +9 Level
  * +4 Proficiency (Expert)
  * [+1 Weapon Potency](https://2e.aonprd.com/Equipment.aspx?ID=2830)
  * Offguard (+2)
* Damage = `2d6,2d6+6`
  * [Shortsword](https://2e.aonprd.com/Weapons.aspx?ID=43) (`2d6`)
  * [Striking rune](https://2e.aonprd.com/Equipment.aspx?ID=2829)
  * +4 DEX
  * +2 Weapon specialization (Untyped bonus)
  * `2d6` Sneak attack

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+18+2" --weapon-details "2d6,2d6+6"
```

---

## Fighter

Baseline for what a frontline martial can do, since they are the max for single-target damage in Pathfinder. Assuming either a single or double attack per round, allowing 1 or 2 actions for moving and raising shield.

Using a [Longsword](https://2e.aonprd.com/Weapons.aspx?ID=386), and picking this as the weapon group when gaining [Fighter Weapon Mastery](https://2e.aonprd.com/Classes.aspx?ID=35).

* Hit bonus = `+20`
  * +4 STR
  * +9 Level
  * +6 Proficiency (Master)
  * [+1 Weapon Potency](https://2e.aonprd.com/Equipment.aspx?ID=2830)
* Damage = `2d8+7`
  * [Longsword](https://2e.aonprd.com/Weapons.aspx?ID=386) (`2d8`)
  * [Striking rune](https://2e.aonprd.com/Equipment.aspx?ID=2829)
  * +4 STR
  * +3 Weapon specialization (Untyped bonus)

```bash
# Single attack
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20" --weapon-details "2d8+7"

# Double attack
dpr_simulator --use-pf2e-criticals --ac-targets 28 --to-hit "1d20+20" "1d20+20-5" --weapon-details "2d8+7"
```

---

## Summary

|Build|ConsiderationsHits per round|Crits per round|Damage per round|
|:---|:---|:---:|:---:|:---:|
|[Pistol twirl](#pistol-twirl-build)||0.75|0.25|14.37|
|[Pistol twirl](#pistol-twirl-build)|Special ammo|0.75|0.25|19.33|
|[Pistol twirl](#pistol-twirl-build)|Off-guard, not frightened|0.70|0.20|12.61|
|[Pistol twirl](#pistol-twirl-build)|No debuffs on target AC|0.60|0.10|9.04|
|[Paired shots](#paired-shots-build)||1.4|0.40|25.21|
|[Paired shots](#paired-shots-build)|3 round average|0.93|0.27|16.81|
|[Drifter build](#sword-and-pistol-build)||0.95|0.15|12.67|
|[Rogue](#rogue)||0.60|0.10|14.00|
|[Fighter](#fighter)|1 attack|0.60|0.10|11.21|
|[Fighter](#fighter)|2 attacks|1.00|0.15|18.40|

---
