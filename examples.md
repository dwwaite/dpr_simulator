# Examples

A few examples of the tool in action for some Pathfinder martial comparisons

---

## Fighter 5

* STR 4
* To hit = +4 (STR) +5 (LEVEL) +6 (MASTER)

### 1 handed

1. Hit + Snagging strike
1. Hit w/ offguard, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 \
    --to-hit "1d20+15 1d20+15-5+2" \
    --weapon-details "2d8+4 2d8+4" \
    --output fighter_1h.parquet
```

<details>
<summary>Results</results>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|14|1.80|0.95|35.78|
|16|1.70|0.75|31.83|
|18|1.55|0.55|27.31|
|20|1.35|0.35|22.11|
|22|1.15|0.15|16.90|
|24|1.00|0.10|14.31|

</details>

### Dual wielding

1. Double slice with a non-agile weapon

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 \
    --to-hit "1d20+15 1d20+15-2" \
    --weapon-details "2d8+4 2d8+4" \
    --output fighter_dw.parquet
```

<details>
<summary>Results</results>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|14|1.80|1.00|36.40|
|16|1.75|0.80|33.14|
|18|1.60|0.60|28.63|
|20|1.40|0.40|23.38|
|22|1.20|0.20|18.18|
|24|1.05|0.10|14.95|

</details>

1. Double slice with an agile off hand weapon
   1. Assume 1 die size smaller

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 \
    --to-hit "1d20+15 1d20+15" \
    --weapon-details "2d8+4 2d6+4" \
    --output fighter_dw_agile.parquet
```

<details>
<summary>Results</results>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|14|1.80|1.10|34.80|
|16|1.80|0.90|32.39|
|18|1.70|0.70|28.80|
|20|1.50|0.50|24.02|
|22|1.30|0.30|19.22|
|24|1.10|0.10|14.40|

</details>

### 2 handed

1. Hit
1. Hit, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 \
    --to-hit "1d20+15 1d20+15-5" \
    --weapon-details "2d12+4 2d12+4" \
    --output fighter_2h.parquet
```

<details>
<summary>Results</results>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|14|1.70|0.85|43.34|
|16|1.60|0.65|38.25|
|18|1.45|0.45|32.29|
|20|1.25|0.25|25.50|
|22|1.10|0.20|22.09|
|24|0.90|0.10|17.00|

</details>

---

## Barbarian

* STR 4
* To hit = +4 (STR) +5 (LEVEL) +4 (EXPERT)

### Dragon instinct, 2-handed

>Rage bonus +4 damage

1. Hit
1. Hit, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 \
    --to-hit "1d20+15 1d20+15-5" \
    --weapon-details "2d12+8 2d12+8" \
    --output barbarian_2h_dragon.parquet
```

<details>
<summary>Results</results>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|14|1.70|0.85|53.58|
|16|1.60|0.65|47.19|
|18|1.45|0.45|39.88|
|20|1.25|0.25|31.49|
|22|1.10|0.20|27.32|
|24|0.90|0.10|20.99|

</details>

### Other instinct, 2-handed

>Rage bonus +2 damage

1. Hit
1. Hit, MAP-5

```bash
dpr_simulator --use-pf2e-criticals --ac-targets 14 16 18 20 22 24 \
    --to-hit "1d20+15 1d20+15-5" \
    --weapon-details "2d12+6 2d12+6" \
    --output barbarian_2h.parquet
```

<details>
<summary>Results</results>

|Target AC|Hits per round (mean)|Critical hits per round (mean)|Damage per round (mean)|
|:---:|:---:|:---:|:---:|
|14|1.70|0.85|48.47|
|16|1.60|0.65|42.76|
|18|1.45|0.45|36.08|
|20|1.25|0.25|28.53|
|22|1.10|0.20|24.73|
|24|0.90|0.10|19.02|

</details>

---

## Comparison

|Class|Build|DPR<br />AC14|<br />AC16|<br />AC18|<br />AC20|<br />AC22|<br />AC24|Crits per round<br />AC14|<br />AC16|<br />AC18|<br />AC20|<br />AC22|<br />AC24|
|:---|:---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|Fighter|1H|35.78|31.83|27.31|22.11|16.90|14.31|0.95|0.75|0.55|0.35|0.15|0.10|
|Fighter|DW, non-agile OH|36.40|33.14|28.63|23.38|18.18|14.95|1.00|0.80|0.60|0.40|0.20|0.10|
|Fighter|DW, agile OH|34.80|32.39|28.80|24.02|19.22|14.40|1.10|0.90|0.70|0.50|0.30|0.10|
|Fighter|2H|43.34|38.25|32.29|25.50|22.09|17.00|0.85|0.65|0.45|0.25|0.20|0.10|
|Barbarian|Dragon, 2H|53.58|47.19|39.88|31.49|27.32|20.99|0.85|0.65|0.45|0.25|0.20|0.10|
|Barbarian|Other, 2H|48.47|42.76|36.08|28.53|24.73|19.02|0.85|0.65|0.45|0.25|0.20|0.10|

---
