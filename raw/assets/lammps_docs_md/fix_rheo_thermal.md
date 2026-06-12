# fix rheo/thermal command

Source: https://docs.lammps.org/fix_rheo_thermal.html

## Syntax

```
fix ID group-ID rheo/thermal attribute values ...
```

- ID, group-ID are documented in fix command
- rheo/thermal = style name of this fix command
- one or more attributes may be appended
- attribute = conductivity or specific/heat or latent/heat or Tfreeze or react

```
conductivity args = types style args
  types = lists of types (see below)
  style = constant
    constant arg = conductivity (power/(length*temperature))
specific/heat args = types style args
  types = lists of types (see below)
  style = constant
    constant arg = specific heat (energy/(mass*temperature))
latent/heat args = types style args
  types = lists of types (see below)
  style = constant
    constant arg = latent heat (energy/mass)
Tfreeze args = types style args
  types = lists of types (see below)
  style = constant
    constant arg = freezing temperature (temperature)
react args = cut type
  cut = maximum bond distance
  type = bond type
```

## Examples

```
fix 1 all rheo/thermal conductivity * constant 1.0 specific/heat * constant 1.0 Tfreeze * constant 1.0
fix 1 all rheo/pressure conductivity 1*2 constant 1.0 conductivity 3*4 constant 2.0 specific/heat * constant 1.0
```

## Description

Added in version 29Aug2024.

This fix performs time integration of temperature for atom style rheo/thermal. In addition, it defines multiple thermal properties of particles and handles melting/solidification, if applicable.

For each atom type, one can define expressions for the conductivity, specific/heat, latent/heat, and critical temperature (Tfreeze). The conductivity and specific heat must be defined for all atom types.

The react keyword controls whether bonds are created/deleted when particles transition between a fluid and solid state. This option only applies to atom types that have a defined value of Tfreeze.

This fix is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Related commands

fix rheo, pair rheo, compute rheo/property/atom, fix add/heat
