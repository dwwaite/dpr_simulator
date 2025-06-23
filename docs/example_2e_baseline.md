# Proficiency baseline

Since a lot of the Pathfinder simulations I want to run test over a range of levels, this is a baseline test to find what an appropriate AC at each level would be, and to map out the hit modifiers for common class profiles so that I don't need to write them out for each test run later.

---

## Expected monster AC by level

|Level|Number of entries|Mean AC|Median AC|
|:---:|:---:|:---:|:---:|
|-1|18|14.8|15.0|
|0|11|15.1|15.0|
|1|53|16.1|16.0|
|2|41|17.4|17.0|
|3|44|17.9|18.0|
|4|34|20.7|21.0|
|5|28|21.0|21.5|
|6|24|23.2|24.0|
|7|21|24.5|25.0|
|8|19|26.6|27.0|
|9|19|27.1|28.0|
|10|16|29.8|30.0|
|11|10|30.8|31.0|
|12|10|32.3|33.0|

Rather than run against a series of AC values, will run against the average of each, for each level.

---

# Expected to-hit values for different archetypes

Based on these values, what is the average chance to hit for different class types, assuming they are prioritising their to-hit stat and following item bonus progression.

Working this out based on classes of proficiency progression, not game classes, as there are really only three cases:

1. Martial/Expert - Fighter and Gunslinger
   1. Start at Expert proficiency with weapons, then upgrade to Master at Level 5.
   1. Example: [Fighter](https://2e.aonprd.com/Classes.aspx?ID=35)
1. Martial/Trained - Barbarian and Champion
   1. Start at Trained proficiency with weapons, upgrade to Expert at Level 5.
   1. Example: [Barbarian](https://2e.aonprd.com/Classes.aspx?ID=57)
1. Other - Non-martial classes
   1. Start at Trained proficiency with weapons, upgrade to Expert at Level 11.
   1. Example: [Bard](https://2e.aonprd.com/Classes.aspx?ID=32)

|Level|MOD|Item bonus|Martial/Expert<br />Proficiency|<br />Total|Martial/Trained<br />Proficiency|Total|Non-martial<br />Proficiency|Total|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|1|4|0|4|9|2|7|2|7|
|2|4|1|4|11|2|9|2|9|
|3|4|1|4|12|2|10|2|10|
|4|4|1|4|13|2|11|2|11|
|5|4 (4.5)|1|6|16|4|14|2|12|
|6|4 (4.5)|1|6|17|4|15|2|13|
|7|4 (4.5)|1|6|18|4|16|2|14|
|8|4 (4.5)|1|6|19|4|17|2|15|
|9|4 (4.5)|2|6|21|4|19|2|17|
|10|5|2|6|23|4|21|2|19|
|11|5|2|6|24|4|22|4|22|
|12|5|2|6|25|4|23|4|23|

---

# Average hit and crit rates

Putting these together (ignoring damage, which is dependent on class, subclass, and items), we can simulate the hit rates for the three classes of attack profile against the expected AC at each character level.

```bash
ac_array=(16 17 18 21 22 24 25 27 28 30 31 33)

# Martial/Expert
hit_array=(9 11 12 13 16 17 18 19 21 23 24 25)

for i in {0..11};
do
    dpr_simulator --use-pf2e-criticals --ac-targets ${ac_array[$i]} --to-hit "1d20+${hit_array[$i]}" --weapon-details 0 
done

# Martial/Trained
hit_array=(7 9 10 11 14 15 16 17 19 21 22 23)

for i in {0..11};
do
    dpr_simulator --use-pf2e-criticals --ac-targets ${ac_array[$i]} --to-hit "1d20+${hit_array[$i]}" --weapon-details 0 
done


# Non-martial
hit_array=(7 9 10 11 12 13 14 15 17 19 22 23)

for i in {0..11};
do
    dpr_simulator --use-pf2e-criticals --ac-targets ${ac_array[$i]} --to-hit "1d20+${hit_array[$i]}" --weapon-details 0 
done
```

---

## Summary

|Level|Martial/Expert<br />Hit chance|<br />Crit chance|Martial/Trained<br />Hit chance|<br />Crit chance|Non-martial<br />Hit chance|<br />Crit chance|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
|1|0.70|0.20|0.60|0.10|0.60|0.10|
|2|0.75|0.25|0.65|0.15|0.65|0.15|
|3|0.75|0.25|0.65|0.15|0.65|0.15|
|4|0.65|0.15|0.55|0.05|0.55|0.05|
|5|0.75|0.25|0.65|0.15|0.55|0.05|
|6|0.70|0.20|0.60|0.10|0.50|0.05|
|7|0.70|0.20|0.60|0.10|0.50|0.05|
|8|0.65|0.15|0.55|0.05|0.45|0.05|
|9|0.70|0.20|0.60|0.10|0.50|0.05|
|10|0.70|0.20|0.60|0.10|0.50|0.05|
|11|0.70|0.20|0.60|0.10|0.60|0.10|
|12|0.65|0.15|0.55|0.05|0.55|0.05|

Interestingly, there are slight drops at level 4, 6, 8, and 12.

It's also notable the difference in hit rate between martial and non-martial classes - the non-martial can basically only score a critical hit on the natural 20. By contrast expert classes would mostly score a critical on a 19 or 20, and Fighters (and Gunslingers) mostly score criticals on a 17 or higher.

---
