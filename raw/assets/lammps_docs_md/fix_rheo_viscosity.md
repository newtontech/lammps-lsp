# fix rheo/viscosity command

Source: https://docs.lammps.org/fix_rheo_viscosity.html

## Syntax

```
fix ID group-ID rheo/viscosity type1 pstyle1 args1 ... typeN pstyleN argsN
```

- ID, group-ID are documented in fix command
- rheo/viscosity = style name of this fix command
- one or more types and viscosity styles must be appended
- types = lists of types (see below)
- vstyle = constant or power

```
constant args = eta
  eta = viscosity

power args = eta, gd0, K, n
  eta = viscosity
  gd0 = critical strain rate
  K = consistency index
  n = power-law exponent
```

## Examples

```
fix 1 all rheo/viscosity * constant 1.0
fix 1 all rheo/viscosity 1 constant 1.0 2 power 0.1 5e-4 0.001 0.5
```

## Description

Added in version 29Aug2024.

This fix defines a viscosity for RHEO particles. One can define different viscosities for different atom types, but a viscosity must be specified for every atom type.

- **constant**: applies a constant viscosity eta to each particle of the assigned type.
- **power**: Herschel-Bulkley constitutive equation for stress tau, with regularization using critical strain rate gd0.

This fix is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Related commands

fix rheo, pair rheo, compute rheo/property/atom
