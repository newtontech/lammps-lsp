# pair_style rheo/solid command

Source: https://docs.lammps.org/pair_rheo_solid.html

## Syntax

```
pair_style rheo/solid
```

## Examples

```
pair_style rheo/solid
pair_coeff * * 1.0 1.5 1.0
```

## Description

Added in version 29Aug2024.

Style rheo/solid is effectively a copy of pair style bpm/spring except it only applies forces between solid RHEO particles, determined by checking the status of each pair of neighboring particles before calculating forces.

The style computes pairwise forces: F = k (r - r_c) where k is a stiffness and r_c is the cutoff length. An additional damping force is also applied.

Coefficients:
- k (force/distance units)
- r_c (distance units)
- gamma (force/velocity units)

This pair style is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Related commands

fix rheo, fix rheo/thermal, pair bpm/spring
