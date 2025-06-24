# Player Core Untamed Druid

Calculating the damage of a wild shape druid is tricky due to the way that the players to-hit bonus can *sometimes* be used in place of the animal form's shape. This comparison is used to look at the difference between three untamed build styles.

1. Dump STR, just rely on the forms bonus.
1. Pump STR, using whichever bonus is greater at any given time.
1. Pump STR, use [Form Control] to reduce shape shift spell level and force the use of the STR bonus over the form.
   1. This seems like a terrible idea, as the players AC and damage will be behind the curve.
   1. But maybe the higher hit makes up for it?

>At all levels, will use whichever heightened shapeshift form looks like it would do the most damage. Most forms have a lower-damage agile attack but previous testing has shown that this doesn't pan out overall.

>Two attacks per turn - this isn't always going to be realistic since transforming is a 2-action ability but it's constant between builds.

>This comparison uses the [armor class progression](./example_2e_mutagenist.md#player-core-mutagenist) determined in the **Mutagenist** analysis, and will pull in the results of that analysis at the end for comparison.

---

## Forms-only approach

Who needs STR? Just pump that CON and WIS and do other stuff. For level 1, casting a cantrip so assuming max WIS for the starting level.

|Level|Form|MOD|Proficiency bonus|Item bonus|Total attack|Damage|Notes|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---|
|1|[Gouging Claw](https://2e.aonprd.com/Spells.aspx?ID=1546)|4 (WIS)|2|0|7|`2d6+2`|Spell 1|
|2|[Gouging Claw](https://2e.aonprd.com/Spells.aspx?ID=1546)|4 (WIS)|2|0|8|`2d6+2`|Spell 1|
|3|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|9|`2d8+1`|Spell 2|
|4|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|9|`2d8+1`|Spell 2|
|5|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|14|`2d8+5`|Spell 3, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|6|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|14|`2d8+5`|Spell 3, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|7|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|16|`2d8+9`|Spell 4, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|8|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|16|`2d8+9`|Spell 4, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575) and equal to [Dinosaur Form](https://2e.aonprd.com/Spells.aspx?ID=1489)|
|9|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|18|`4d8+7`|Spell 5, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|10|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|-|-|-|18|`4d8+7`|Spell 5, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|11|[Aerial Form](https://2e.aonprd.com/Spells.aspx?ID=1437)|-|-|-|21|`4d8+4`|Spell 6, only shape available at this rank|
|12|[Aerial Form](https://2e.aonprd.com/Spells.aspx?ID=1437)|-|-|-|21|`4d8+4`|Spell 6, only shape available at this rank|


```bash
# First and second level, single cantrip
dpr_simulator --use-pf2e-criticals --ac-targets 16 --to-hit "1d20+7" --weapon-details "2d6+2" -o resources/Druid1_1.parquet
dpr_simulator --use-pf2e-criticals --ac-targets 17 --to-hit "1d20+8" --weapon-details "2d6+2" -o resources/Druid1_2.parquet

# Third onward
ac_array=(18 21 22 24 25 27 28 30 31 33)
hit_array=(9 9 14 14 16 16 18 18 21 21)
dmg_array=("2d8+1" "2d8+1" "2d8+5" "2d8+5" "2d8+9" "2d8+9" "4d8+7" "4d8+7" "4d8+4" "4d8+4")

for i in {0..9};
do
    dpr_simulator --use-pf2e-criticals \
        --ac-targets ${ac_array[$i]} \
        --to-hit "1d20+${hit_array[$i]}" "1d20+${hit_array[$i]}-5" \
        --weapon-details ${dmg_array[$i]} \
        -o resources/Druid1_$(($i+3)).parquet
done
```

---

## STR-based approach

Will occassionally make use of the higher PC to-hit bonus to keep the damage higher. Druids follow the [Non-martial progression](./example_2e_baseline.md), but are offset by the lower primary stat.

I'm unclear on how the [Untamed Form](https://2e.aonprd.com/Spells.aspx?ID=1861) status bonus relates to the wording on feats such as [Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440), specifically

```
When you choose to use your own attack modifier while polymorphed instead of the form's default attack modifier, you gain a +2 status bonus to your attack rolls.
```

```
If your unarmed attack bonus is higher, you can use it instead.
```

I've seen online interpretations where both the `+2 status bonus` is only given when the base attack bonus is greater than the form, or when the PC can include the `+2 status bonus` in determining whether or not their bonus is greater than the form. I am assuming for this simulation the later case, as otherwise it's impossible to get this bonus other than at Level 4. If this is not the case when the status bonus would appear to exist solely for when downcasting with [Form Control](https://2e.aonprd.com/Feats.aspx?ID=4723) or when multiclassing.

|Level|Form|MOD|Proficiency bonus|Item bonus|Status bonus|Total attack|Form attack|Final attack|Damage|Notes|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---|
|1|[Untamed Shift](https://2e.aonprd.com/Spells.aspx?ID=1862)|3|2|0|0|6|-|6|"1d6+3"|Spell 1|
|2|[Untamed Shift](https://2e.aonprd.com/Spells.aspx?ID=1862)|3|2|1|0|8|-|8|"1d6+3"|Spell 1, [Handwraps of Mighty Blows](https://2e.aonprd.com/Equipment.aspx?ID=3086)|
|3|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|3|2|1|2|11|9|11|`2d8+1`|Spell 2|
|4|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|3|2|1|2|12|9|12|`2d8+1`|Spell 2|
|5|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|4|2|1|2|14|14|14|`2d8+5`|Spell 3|
|6|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|4|2|1|2|15|14|15|`2d8+5`|Spell 3|
|7|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|4|2|1|2|16|16|16|`2d8+9`|Spell 4, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|8|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|4|2|1|2|17|16|17|`2d8+9`|Spell 4, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|9|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|4|2|1|2|18|18|18|`4d8+7`|Spell 5, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|10|[Animal Form](https://2e.aonprd.com/Spells.aspx?ID=1440)|4|2|2|2|20|18|20|`4d8+7`|Spell 5, stronger than [Insect Form](https://2e.aonprd.com/Spells.aspx?ID=1575)|
|11|[Aerial Form](https://2e.aonprd.com/Spells.aspx?ID=1437)|4|4|2|2|23|21|23|`4d8+4`|Spell 6, only shape available at this rank|
|12|[Aerial Form](https://2e.aonprd.com/Spells.aspx?ID=1437)|4|4|2|2|24|21|24|`4d8+4`|Spell 6, only shape available at this rank|

Technically `Animal Form` is still strong at levels 11 and 12, but due to the lower AC I'm ignoring it.

```bash
ac_array=(16 17 18 21 22 24 25 27 28 30 31 33)
hit_array=(6 8 11 12 14 15 16 17 18 20 23 24)
dmg_array=("1d6+3" "1d6+3" "2d8+1" "2d8+1" "2d8+5" "2d8+5" "2d8+9" "2d8+9" "4d8+7" "4d8+7" "4d8+4" "4d8+4")

for i in {0..11};
do
    dpr_simulator --use-pf2e-criticals \
        --ac-targets ${ac_array[$i]} \
        --to-hit "1d20+${hit_array[$i]}" "1d20+${hit_array[$i]}-5" \
        --weapon-details ${dmg_array[$i]} \
        -o resources/Druid2_$(($i+1)).parquet
done
```

---

## Summary

Pulling in the Fighter and Barbarian data from the [Mutagenist simulation](./example_2e_mutagenist.md#summary) for comparison.

```python
import glob
import polars as pl
import plotly.express as px

input_data = []
for input_file in glob.glob('resources/*.parquet'):
    input_data.append(
        pl
        .scan_parquet(input_file)
        .with_columns(File=pl.lit(input_file))
        .with_columns(
            Class=pl.col('File').str.extract(r'resources/(\w+)_', 1).cast(pl.Categorical),
            Level=pl.col('File').str.extract(r'_(\d+)', 1).cast(int)
        )
        .select('Iteration', 'Target_AC', 'Number_hits', 'Number_crits', 'Total_damage', 'Class', 'Level')
        .collect()
    )

df = pl.concat(input_data, how='vertical')

(
    df
    .with_columns(hit_mask=pl.col('Number_hits').gt(0))
    .group_by(['Class', 'Level'])
    .agg(
        Damage=pl.col('Total_damage').median(),
        Hit_chance=(pl.col('hit_mask').sum()/pl.col('Number_hits').count())
    )
    .pivot(index='Level', on='Class', values=['Hit_chance', 'Damage'])
    .sort('Level', descending=False)
)

# View the distribution for a single hit, non-critical
plot_df = (
    df
    .filter(
        pl.col('Number_hits').eq(1).over(['Level', 'Class']),
        pl.col('Number_crits').eq(0)
    )
)

fig = px.box(plot_df, x='Level', y='Total_damage', color='Class', points=False)
fig.write_image('images/example_2e_druid.png')
```

|Level|Druid (form-only)<br />Hit chance|<br />Damage|Druid (STR-based)<br />Hit chance|<br />Damage|Fighter<br />Hit chance|<br />Damage|Barbarian<br />Hit chance|<br />Damage|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|1|0.60|7|0.68|6|0.83|11|0.74|13|
|2|0.60|7|0.74|7|0.87|12|0.79|14|
|3|0.74|10|0.83|13|0.87|12|0.79|14|
|4|0.56|6|0.74|10|0.79|15|0.69|16|
|5|0.79|16|0.79|16|0.87|19|0.79|19|
|6|0.69|13|0.74|14|0.84|16|0.74|17|
|7|0.74|18|0.74|18|0.84|20|0.74|19|
|8|0.63|16|0.69|17|0.79|18|0.68|18|
|9|0.68|24|0.68|24|0.83|20|0.74|19|
|10|0.56|20|0.68|24|0.84|21|0.74|20|
|11|0.68|21|0.79|24|0.83|21|0.74|20|
|12|0.56|17|0.74|22|0.79|25|0.69|23|

![](../images/example_2e_druid.png)

---
