# About

Messing around in `rust` to make a damage calculator for D&D 5e or Pathfinder 2e.

## Contents

1. [Background](#background)
1. [Detailed description of commands](#detailed-description-of-commands)
1. [Examples](#examples)
1. [What to do with the output](#what-to-do-with-the-output)

---

## Background

Performs a simulation over a set of armour class values. Each simulation consists of a set of attacks, representing a round of combat in either the D&D or Pathfinder rules.

---

## Detailed description of commands

A full overview of the parameters availabe in the tool can be seen by running with the `-h` or `--help` flags.

```bash
Usage: dpr_simulator [OPTIONS] --output <OUTPUT FILE>

Options:
  -a, --ac-targets <AC TARGETS>...
          Space-delimited AC values to test against (default 12, 14, 16, 18, 20) [default: 12 14 16 18 20]
  -t, --to-hit <TO HIT>...
          To-Hit modifier, one or one per attack to be made
  -w, --weapon-details <WEAPON DETAILS>...
          Details of each attack to be made in the form 1d8+5
  -o, --output <OUTPUT FILE>
          Path to save results (Apache parquet format)
  -n, --number-turns <NUMBER TURNS>
          Number of turns to simulate (default 1,000,000) [default: 1000000]
      --n-threads <N THREADS>
          Number of threads for running in multi-threaded mode (optional)
      --use-pf2e-criticals
          Use Pathfinder 2e rules for critical hits and damage calculation (default: False)
      --seed <SEED VALUE>
          Set the seed to dice rolling to a predetermined value (optional)
  -h, --help
          Print help
```

**To hit**

A series of integer values representing the characters to-hit bonus for each fo the attacks made. For flexibility, this tool uses a value per attack made as the attack bonus is not necessarily static. In the D&D 5e rules, a character could be using different modifiers when fighting two-handed. This could happen either in a case where they have two weapons with different bonuses to hit (e.g. a +2 weapon and a +1 weapon), or if that were mixing a strength and finese attack, or if they were using feats such as the D&D 5E (2014) [Great Weapon Master](http://dnd5e.wikidot.com/feat:great-weapon-master) or [Sharpshooter](http://dnd5e.wikidot.com/feat:sharpshooter), but toggling them on and off during a round.

Alternatively, this approach allows the tool to capture the [Multiple Attack Penalty](https://2e.aonprd.com/Rules.aspx?ID=220) (MAP) if modelling Pathfinder 2e rules. There are a huge number of factors which can affect a characters MAP, such as weapon traits, class features, and circumstance bonuses. It's not practical to capture all of these without massive bloat of the command line parameters and a messy structure within the tool code.

The simplest solution which covers both of these rule sets, and all their edge cases, is to simply give each attack its own to-hit bonus, at the discretion of the user. For example the following combinations are all supported, all starting with a `+8` to hit.

```bash
# D&D 5e, making a standard weapon attack
dpr_simulation --to-hit "1d20+8" ...

# D&D 5e, GWM on the second hit, not on the first
dpr_simulation --to-hit "1d20+8 1d20+3" ...

# Pathfinder 2e, three attacks applying the standard MAP progression
dpr_simulation --to-hit "1d20+8 1d20+3 1d20-2" ...

# Pathfinder 2e, three attacks applying MAP with an Agile weapon
dpr_simulation --to-hit "1d20+8 1d20+4 1d20" ...

# Pathfinder 2e, three attacks applying MAP progression for a Ranger (Flurry) with an Agile weapon
dpr_simulation --to-hit "1d20+8 1d20+5 1d20+2" ...
```

**AC targets**

One of the biggest problems when people describe the damage of attacks is that the chance to hit isn't factored in. This is easy to do, assuming a base chance to hit as a percentage and multiplying damage rolls by this number but in practice even across a single adventuring day, you would expect to encounter enemies with differing armour class values and so the flat percentage isn't necessarily informative.

This tool therefore rolls attacks using the to-hit modifier above against a range of armour class values, so that you can see how your approach would stack up against different targets. In particular, this should highlight how feats like [Great Weapon Master](http://dnd5e.wikidot.com/feat:great-weapon-master) and [Sharpshooter](http://dnd5e.wikidot.com/feat:sharpshooter) might be a significant damage increase against low-AC enemies but become signficantly worse on comparatively high-AC (and what the definition of low- and high-AC might be for your current to-hit value).

**Weapon details**

As shown in the examples in the previous section, this is where the damage per attack is written. The notation used to describe damage is the standard die plus modifier notation common in D&D and Pathfinder.

Similar to the to-hit parmeter, one element per attack needs to be provided. To expand upon the to-hit example above using GWM, this can be captured in the weapon details by altering the static damage modifier on each attack.

For example, for a weapon that deals two `d6` worth of damage this would be written as "`2d6`". Data are extracted from the user-provided strings in terms of either die elements, or static elements.

```bash
# Single main hand attack in D&D 5e:
dpr_simulation --weapon-details "1d8+5" ...

# Two main hand attacks...
dpr_simulation --weapon-details "1d8+5" "1d8+5" ...

# Two main hand attacks and an offhand attack (without modifier added)
dpr_simulation --weapon-details "1d8+5" "1d8+5" "1d6"

# Carrying on with the GWM example previously, where the second hit applies the GWM bonus but the first does not
dpr_simulation --weapon-details "2d6+5" "2d6+15" ...
```

Mixed die notations are also allowed, to account for situations where a weapon or class feature may add a die on top of weapon damage independent of the weapon's damage die. When using these, make sure to separate the different die in some form (comma, space, etc) otherwise the regex engine will likely misread the input. An as example, a D&D Ranger using a longbow, with a dexterity modifier of three, using Hunter's Mark would could write their damage as;

```bash
dpr_simulation --weapon-details "1d8,1d6+3" ...
```

---

## Adding modifier tags to dice

This tool supports some modified behaviours, based on common traits applied to weapons in either D&D or Pathfinder. All currently supported tags are listed below:

|Tag|Behaviour|Game trait|
|:---|:---|:---|
|`[kh#]`|Reroll the dice `#` times and keep the highest value|D&D Advantage|
|`[kl#]`|Reroll the dice `#` times and keep the lowest value|D&D Disadvantage|
|`[onmiss]`|Damage is only applied when the attack misses|D&D [Graze mastery](http://dnd2024.wikidot.com/equipment:weapon#toc3)|
|`[oncrit]`|Damage is applied as-in on a critical hit only|D&D Barbarian [Brutal Strike](http://dnd2024.wikidot.com/barbarian:main#Brutal-Strike)|
|`[oncrit_doubles]`|Damage is applied on a critical hit only, and doubles as per critical rules|D&D Paladin saving their [Divine Smite](http://dnd2024.wikidot.com/spell:divine-smite) for when they score a critical|
|`[deadly#]`|Pathfinder 2e `Deadly` trait with die size `#`|Pathfinder [Deadly](https://2e.aonprd.com/Traits.aspx?ID=570)|
|`[fatal#]`|Pathfinder 2e `Fatal` trait with die size `#`|Pathfinder [Fatal](https://2e.aonprd.com/Traits.aspx?ID=597)|

### Advantage-style rerolls

Dice can be rolled multiple times to keep the highest or lowest value. This mimics D&D Advantage/Disadvantage or Pathfinder Hero Points. It is also flexible as to the number of dice, to account for features like the [Elven Accuracy feat](http://dnd5e.wikidot.com/feat:elven-accuracy) from the 2014 D&D rules.

```bash
# D&D 5e, making a standard weapon attack with Advantage...
dpr_simulation --to-hit "1d20[kh2]+8" ...

# ...Disadvantage...
dpr_simulation --to-hit "1d20[kl2]+8" ...

# ...Elven Accuracy
dpr_simulation --to-hit "1d20[kh3]+8" ...
```

## Output

The name of the file to which results are written. Results are compressed in the [Apache Parquet](https://parquet.apache.org/) format. This can easily be parsed using libraries like [pandas](https://pandas.pydata.org/) or [polars](https://pola.rs/) in `python`, or [read_parquet.R](https://rdrr.io/cran/arrow/man/read_parquet.html) in `R`.

**Number of turns**

The number of turns to simulate over. The default value is 1,000,000 per AC value, which is more than sufficient to extract a good simulation. Realisitically, it's more turns than you'll ever have over a campaign.

**Pathfinder criticals rule**

Changes the logic to use the Pathfinder 2e rules for interpretting natural 1s and 20s, and the rules for [degrees of success](https://2e.aonprd.com/Rules.aspx?ID=319). Briefly, these are:

1. If a natural 20, or the AC is bet by 10 or more, increase the degree of success by one.
1. If a natural 1, or the AC is missed by 10 or more, decrease the degree of success by one.
1. On a critical hit, double **_all_** damage (D&D 5e rule is to double just the dice rolled).

In practice this *mostly* just means that it is easier to score critical hits against enemies with lower AC values but there are some situations where the difference between the to-hit and AC are so great that a natural 1 can still hit.

---

## Examples

Since I made this tool for myself, there are a few questions I was interested in answering with it:

1. [Pathfinder 2E - Baseline of hit rates across different martial types](./docs/example_2e_baseline.md)
1. [Pathfinder 2E - Comparing Gunslinger builds against a rogue and fighter](./docs/example_2e_gunslinger.md)
1. [Pathfinder 2E - Comparing Fighters and Barbarians](./docs/example_2e_barb_fighter.md)
1. [Pathfinder 2E - Comparing wildshape Druids to caster option](./docs/example_2e_druid.md)
1. [Pathfinder 2E - Comparing the remastered Mutagenist Alchemist to martials](./docs/example_2e_mutagenist.md)
1. [D&D 5E - Dual-wield evaluation for a Barbarian](./docs/example_5e_barbarian.md)

---

## What to do with the output

After running a quick simulation, the results can be viewed using a library like `pandas` or `polars`. For example, running a quick simulation for a level 5 Fighter (+3 proficiency, +4 STR) using a one-handed longsword.

```bash
dpr_simulator -t "1d20+7 1d20+7" -w "1d8+4 1d8+4" -o output.parquet
```

```python
import polars as pl

(
    pl
    .read_parquet('output.parquet')
    .group_by('Target_AC')
    .agg(
        pl.col('Total_damage').min().alias('Min'),
        pl.col('Total_damage').median().alias('Median'),
        pl.col('Total_damage').max().alias('Max'),
    )
    .sort('Target_AC')
)
```

|Target_AC|Min|Median|Max|
|:---:|:---:|:---:|:---:|
|12.0|0.0|15.0|39.0|
|14.0|0.0|12.0|39.0|
|16.0|0.0|10.0|39.0|
|18.0|0.0|9.0|38.0|
|20.0|0.0|7.0|40.0|

>The minimum and maximum are not necessarily useful - the minimum will almost certainly reflect a round of all misses, and the maximum will most like be a round of critical hits rolling max damage on each die (very rare, but over a million rounds it'll happen sooner or later).

---
