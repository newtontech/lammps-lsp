# fix rheo command

Source: https://docs.lammps.org/fix_rheo.html

## Syntax

```
fix ID group-ID rheo cut kstyle zmin keyword values...
```

- ID, group-ID are documented in fix command
- rheo = style name of this fix command
- cut = cutoff for the kernel (distance)
- kstyle = quintic or RK0 or RK1 or RK2
- zmin = minimal number of neighbors for reproducing kernels
- zero or more keyword/value pairs may be appended to args
- keyword = thermal or interface/reconstruct or surface/detection or shift or rho/sum or density or speed/sound

```
thermal turns on thermal evolution
  values = none
interface/reconstruct reconstructs interfaces with solid particles
  values = none
surface/detection detects free-surfaces with an absence of particles
  values = sdstyle limit limit/splash
    sdstyle = coordination or divergence
    limit = threshold for surface particles
    limit/splash = threshold for splash particles (unitless)
shift turns on velocity shifting
  values = none
  optional args = exclude/type or scale/cross/type
    exclude/type values = types
      types = list of types
    scale/cross/type values = shiftscale cmin rmin
      shiftscale = fraction of shifting in normal direction to preserve (unitless)
      cmin = minimum color function value required for scaling (unitless)
      rmin = minimum local same-type weighted distance required for any shifting (unitless)
rho/sum density evolution performed by a kernel summation
  values = none
  optional args = self/mass
    self/mass values = none, a particle uses its own mass in summation
density specify equilibrium densities for each atom type
  values = rho01, ... rho0N (density)
speed/sound specify speeds of sound for each atom type
  values = cs0, ... csN (velocity)
```

## Examples

```
fix 1 all rheo 3.0 quintic 0 thermal density 0.1 0.1 speed/sound 10.0 1.0
fix 1 all rheo 3.0 RK1 10 shift surface/detection coordination 40
fix 1 all rheo 3.0 RK1 10 shift exclude/type 2*4 scale/cross/type 0.05 0.02 0.5
fix 1 all rheo 3.0 RK1 10 rhosum self/mass
```

## Description

Added in version 29Aug2024.

Perform time integration for RHEO particles, updating positions, velocities, and densities. Part of the RHEO package for smoothed particle hydrodynamics (SPH).

The type of kernel is specified using kstyle and the cutoff is cut. Four kernels are currently available. The quintic kernel is a standard quintic spline function commonly used in SPH. The other options, RK0, RK1, and RK2, are zeroth, first, and second order reproducing kernels.

To model temperature evolution, specify the thermal keyword, define a separate instance of fix rheo/thermal, and use atom style rheo/thermal.

The interface/reconstruct keyword reconstructs density and velocity of solid particles for every fluid-solid interaction to ensure no-slip and pressure-balanced boundaries.

The shift keyword enables modified Fickian particle shifting to generate a more uniform spatial distribution.

The surface/detection keyword classifies particles as bulk fluid, free surface, or splash/droplet.

The density keyword specifies equilibrium density for each atom type.
The speed/sound keyword specifies speed of sound for each atom type.

## Restrictions

This fix must be used with atom style rheo or rheo/thermal. This fix must be used in conjunction with fix rheo/pressure and fix rheo/viscosity. If the thermal setting is used, there must also be an instance of fix rheo/thermal. The fix group must be set to all. Only one instance of fix rheo may be defined and it must be defined prior to all other RHEO fixes.

This fix is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Related commands

fix rheo/viscosity, fix rheo/pressure, fix rheo/thermal, pair rheo, compute rheo/property/atom

## Default

rho0 and cs are set to 1.0 for all atom types.

## References

(Palermo) Palermo, Wolf, Clemmer, O'Connor, Phys. Fluids, 36, 113337 (2024).
(Yang) Yang, Rakhsha, Hu, Negrut, J. Comp. Physics, 458, 111079 (2022).
(Hu) Hu, and Adams, J. Comp. Physics, 213, 844-861 (2006).
