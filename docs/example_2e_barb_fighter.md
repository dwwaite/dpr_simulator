# Pathfinder 2E: Fighter and Barbarian

A few examples of the tool in action for some Pathfinder martial comparisons. All are built at level 5 and assume two attack actions with appropriate MAP. Testing the spread of AC values around 21 based on the [baseline work](./example_2e_baseline.md).

---

## Fighter - 1 handed

* STR 4
* To hit = +4 (STR) +5 (LEVEL) +6 (MASTER)

1. Hit + Snagging strike
1. Hit with offguard, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets {19..23} --to-hit "1d20+15" "1d20+15-5+2" --weapon-details "2d8+4" "2d8+4"
```

<details>
<summary>Results</summary>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|19|1.55|0.55|22.8|
|20|1.45|0.45|20.2|
|21|1.35|0.35|17.6|
|22|1.25|0.25|15.0|
|23|1.15|0.20|13.3|

</details>

## Fighter - Dual wielding

Double slice with a non-agile weapon.

```bash
dpr_simulator --use-pf2e-criticals --ac-targets {19..23} --to-hit "1d20+15" "1d20+15-2" --weapon-details "2d8+4" "2d8+4"
```

<details>
<summary>Results</summary>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|19|1.60|0.60|24.1|
|20|1.50|0.50|21.5|
|21|1.40|0.40|18.9|
|22|1.30|0.30|16.3|
|23|1.20|0.20|13.7|

</details>

Double slice with an agile off hand weapon, so assuming the second weapon is 1 die size smaller.

```bash
dpr_simulator --use-pf2e-criticals --ac-targets {19..23} --to-hit "1d20+15" "1d20+15" --weapon-details "2d8+4" "2d6+4"
```

<details>
<summary>Results</summary>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|19|1.70|0.70|24.8|
|20|1.60|0.60|22.4|
|21|1.50|0.50|20.0|
|22|1.40|0.40|17.6|
|23|1.30|0.30|15.2|

</details>

## Fighter - 2 handed

1. Hit
1. Hit, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets {19..23} --to-hit "1d20+15" "1d20+15-5" --weapon-details "2d12+4" "2d12+4"
```

<details>
<summary>Results</summary>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|19|1.45|0.45|25.8|
|20|1.35|0.35|22.4|
|21|1.25|0.30|20.1|
|22|1.15|0.25|18.0|
|23|1.05|0.20|15.7|

</details>

---

## Barbarian - Dragon instinct, 2-handed

* STR 4
* To hit = +4 (STR) +5 (LEVEL) +4 (EXPERT)

>Rage bonus +4 damage

1. Hit
1. Hit, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets {19..23} --to-hit "1d20+15" "1d20+15-5" --weapon-details "2d12+8" "2d12+8"
```

<details>
<summary>Results</summary>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|19|1.45|0.45|33.4|
|20|1.35|0.35|29.2|
|21|1.25|0.30|26.4|
|22|1.15|0.25|23.6|
|23|1.05|0.20|20.7|

</details>

---

## Barbarian - Other instincts, 2-handed

>Rage bonus +2 damage

1. Hit
1. Hit, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets {19..23} --to-hit "1d20+15" "1d20+15-5" --weapon-details "2d12+6" "2d12+6"
```

<details>
<summary>Results</summary>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|19|1.45|0.45|29.6|
|20|1.35|0.35|25.8|
|21|1.25|0.30|23.3|
|22|1.15|0.25|20.7|
|23|1.05|0.20|18.2|

</details>

---

## Summary

### Hits per round

|Build|Target AC<br />19|<br />20|<br />21|<br />22|<br />23|
|:---|:---:|:---:|:---:|:---:|:---:|:---:|
|Fighter (1H)|1.55|1.45|1.35|1.25|1.15|
|Fighter (DW, non-agile OH)|1.60|1.50|1.40|1.30|1.20|
|Fighter (DW, agile OH)|1.70|1.60|1.50|1.40|1.30|
|Fighter (2H)|1.45|1.35|1.25|1.15|1.05|
|Barbarian (Dragon)|1.45|1.35|1.25|1.15|1.05|
|Barbarian (Other)|1.45|1.35|1.25|1.15|1.05|

### Crits per round

|Build|Target AC<br />19|<br />20|<br />21|<br />22|<br />23|
|:---|:---:|:---:|:---:|:---:|:---:|:---:|
|Fighter (1H)|0.55|0.45|0.35|0.25|0.20|
|Fighter (DW, non-agile OH)|0.60|0.50|0.40|0.30|0.20|
|Fighter (DW, agile OH)|0.70|0.60|0.50|0.40|0.30|
|Fighter (2H)|0.45|0.35|0.30|0.25|0.20|
|Barbarian (Dragon)|0.45|0.35|0.30|0.25|0.20|
|Barbarian (Other)|0.45|0.35|0.30|0.25|0.20|

### Damage per round

|Build|Target AC<br />19|<br />20|<br />21|<br />22|<br />23|
|:---|:---:|:---:|:---:|:---:|:---:|:---:|
|Fighter (1H)|22.8|20.2|17.6|15.0|13.3|
|Fighter (DW, non-agile OH)|24.1|21.5|18.9|16.3|13.7|
|Fighter (DW, agile OH)|24.8|22.4|20.0|17.6|15.2|
|Fighter (2H)|25.8|22.4|20.1|18.0|15.7|
|Barbarian (Dragon)|33.4|29.2|26.4|23.6|20.7|
|Barbarian (Other)|29.6|25.8|23.3|20.7|18.2|

---
