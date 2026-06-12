# fix rheo/pressure command

Source: https://docs.lammps.org/fix_rheo_pressure.html

## Syntax

```
fix ID group-ID rheo/pressure type1 pstyle1 args1 ... typeN pstyleN argsN
```

- ID, group-ID are documented in fix command
- rheo/pressure = style name of this fix command
- one or more types and pressure styles must be appended
- types = lists of types (see below)
- pstyle = linear or tait/water or tait/general or cubic or ideal/gas or background

```
linear args = none
tait/water args = none
tait/general args = exponent gamma (unitless)
cubic args = cubic prefactor A3 (pressure/density^2)
ideal/gas args = heat capacity ratio gamma (unitless)
background args = background pressure P[b] (pressure)
```

## Examples

```
fix 1 all rheo/pressure * linear
fix 1 all rheo/pressure 1 linear 2 cubic 10.0
fix 1 all rheo/pressure * linear * background 0.1
```

## Description

Added in version 29Aug2024.

This fix defines a pressure equation of state for RHEO particles. One can define different equations of state for different atom types. An equation must be specified for every atom type.

Available pressure styles:
- **linear**: P = c^2 (rho - rho_0)
- **cubic**: P = c^2 ((rho - rho_0) + A3 (rho - rho_0)^3)
- **tait/water**: P = (c^2 rho_0 / 7) * [(rho/rho_0)^7 - 1]
- **tait/general**: P = (c^2 rho_0 / gamma) * [(rho/rho_0)^gamma - 1]
- **ideal/gas**: P = (gamma - 1) * rho * e (only with atom style rheo/thermal)
- **background**: adds a constant background pressure shift P[b]

This fix is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Related commands

fix rheo, pair rheo, compute rheo/property/atom
