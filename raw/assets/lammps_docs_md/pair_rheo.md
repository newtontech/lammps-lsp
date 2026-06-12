# pair_style rheo command

Source: https://docs.lammps.org/pair_rheo.html

## Syntax

```
pair_style rheo cutoff keyword values
```

- cutoff = global cutoff for kernel (distance units)
- zero or more keyword/value pairs may be appended to args
- keyword = rho/damp or artificial/visc or harmonic/means

```
rho/damp args = density damping prefactor xi
artificial/visc args = artificial viscosity prefactor zeta
harmonic/means args = none
```

## Examples

```
pair_style rheo 3.0 rho/damp 1.0 artificial/visc 2.0
pair_coeff * *
```

## Description

Added in version 29Aug2024.

Pair style rheo computes pressure and viscous forces between particles in the rheo package. If thermal evolution is turned on in fix rheo, then the pair style also calculates heat exchanged between particles.

The artificial/viscosity keyword specifies an optional artificial viscosity contribution to forces, helping stabilize simulations.

The rho/damp keyword specifies an optional pairwise damping term between the density of particles.

The harmonic/means keyword changes how viscosities or conductivities are averaged when particles have different values, using harmonic means instead of arithmetic.

No coefficients are defined for each pair of atoms types via the pair_coeff command.

This style does not support the pair_modify shift, table, and tail options.

## Restrictions

This fix is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Related commands

fix rheo, fix rheo/pressure, fix rheo/thermal, fix rheo/viscosity, compute rheo/property/atom

## Default

Density damping and artificial viscous forces are not calculated. Arithmetic means are used for mixing particle properties.
