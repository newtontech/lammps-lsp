# fix rheo/oxidation command

Source: https://docs.lammps.org/fix_rheo_oxidation.html

## Syntax

```
fix ID group-ID rheo/oxidation cut btype rsurf
```

- ID, group-ID are documented in fix command
- rheo/oxidation = style name of this fix command
- cut = maximum bond length (distance units)
- btype = type of bonds created
- rsurf = distance from surface to create bonds (distance units)

## Examples

```
fix 1 all rheo/oxidation 1.5 2 0.0
fix 1 all rheo/oxidation 1.0 1 2.0
```

## Description

Added in version 29Aug2024.

This fix dynamically creates bonds on the surface of fluids to represent physical processes such as oxidation. It is intended for use with bond style rheo/shell.

Every timestep, particles check neighbors within a distance of cut. Bonds of type btype are created between a fluid particle and either a fluid or solid neighbor. The fluid particles must also be on the fluid surface, or within a distance of rsurf from the surface.

This fix is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Restrictions

This fix must be used with the bond style rheo/shell and fix rheo with surface detection enabled.

## Related commands

fix rheo, bond rheo/shell, compute rheo/property/atom

## References

(Clemmer) Clemmer, Pierce, O'Connor, Nevins, Jones, Lechman, Tencer, Appl. Math. Model., 130, 310-326 (2024).
