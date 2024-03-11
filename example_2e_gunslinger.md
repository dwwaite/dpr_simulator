# Pathfinder 2E: Gunslinger

A run through of different weapon set ups for an gunslinger:

* Pistolero with 1 weapon and [Pistol Twirl](https://2e.aonprd.com/Feats.aspx?ID=3162)
* Pistolero with 1 weapon using [Alchemical Shot](https://2e.aonprd.com/Feats.aspx?ID=3165)
* Pistolero dual-wielding using [Paired Shots](https://2e.aonprd.com/Feats.aspx?ID=3168)
* Drifter using a pistol/sword combo, leading with pistol and using [Sword and Pistol](https://2e.aonprd.com/Feats.aspx?ID=3159)

Running for a 5th level character;

1. 18 DEX
1. Master proficiency in firearms (+4)
   1. Expert in martial weapons
1. +1 firearm damage([Singular Expertise](https://2e.aonprd.com/Classes.aspx?ID=20))
1. 18 STR (Drifter)
   1. +4 damage on the melee
1. [Dueling Pistol](https://2e.aonprd.com/Weapons.aspx?ID=201) in all cases
1. [Striking](https://2e.aonprd.com/Equipment.aspx?ID=280) and [Weapon Potency](https://2e.aonprd.com/Equipment.aspx?ID=281) on all weapons

This gives a total hit bonus of 16 for firearms ($+4[DEX] +6[MASTER] +5[LEVEL] +1[RUNE]$), and 14 ($+4[DEX] +6[MASTER] +5[LEVEL] +1[RUNE]$) for melee weapons

---

## Pistol Twirl build

[Pistol Twirl](https://2e.aonprd.com/Feats.aspx?ID=3162) allows a ranged [Feint](https://2e.aonprd.com/Actions.aspx?ID=48) - this will manifest as a +2 to hit, rather than decrease the AC values for target.

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 --to-hit "1d20+16+2" --weapon-details "2f6~10+1"
```

---

## Alchemical Shot build

Adds an extra `1d6` at the chance to misfire. In practice this could do better as it could be tailored to a weakness, but will ignore that here.

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 --to-hit "1d20+16" --weapon-details "2f6~10+1,1d6"
```

---

## Paired Shots build

Two attacks without factoring MAP.

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 --to-hit "1d20+16" "1d20+16" --weapon-details "2f6~10+1" "2f6~10+1"
```

---

## Sword and Pistol build

In practice you can use [Sword and Pistol](https://2e.aonprd.com/Feats.aspx?ID=3159) in either order, but shot/stab is the higher DPR option.

Using a [Shortsword](https://2e.aonprd.com/Weapons.aspx?ID=43) for the [Agile](https://2e.aonprd.com/Traits.aspx?ID=170) and [Finesse](https://2e.aonprd.com/Traits.aspx?ID=179) traits.

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 --to-hit "1d20+16" "1d20+14-3" --weapon-details "2f6~10+1" "2d6+4"
```

---

## Summary

|Target AC|Pistol Twirl build|Alchemical Shot build|Paired Shots build|Sword and Pistol build|
|:---:|:---:|:---:|:---:|:---:|
|14|22.26|25.32|40.22|33.29|
|16|20.11|22.86|35.88|28.98|
|18|17.95|20.36|31.62|24.59|
|20|15.81|16.70|25.70|19.43|
|22|12.85|13.06|19.78|15.96|
|24|9.90|9.39|13.91|11.90|

By comparison the [1H Fighter comparison](./example_pf2e_melee.md#1-handed) has an average DPR of 22.11, 16.90, 14.31 at AC 20, 22, 24. That's marginally better, but requires melee range. It's pretty well balanced.

It's also worth noting that in pratice the Paired Shot build does suffer from action economy limits. Assuming a start with both guns drawn and loaded;

1. Attack, reload x1
1. Reload x1, attack
1. Reload, reload, -
1. Attack, reload x1...

So it's high damage open, assuming no movement is required, but then past the second round it will suffer.

---
