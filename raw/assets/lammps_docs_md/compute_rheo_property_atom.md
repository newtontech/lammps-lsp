# compute rheo/property/atom command

Source: https://docs.lammps.org/compute_rheo_property_atom.html

## Syntax

```
compute ID group-ID rheo/property/atom input1 input2 ...
```

- ID, group-ID are documented in compute command
- rheo/property/atom = style name of this compute command
- input = one or more atom attributes

```
possible attributes = phase, surface, surface/r,
                      surface/divr, surface/n/a, coordination,
                      shift/v/a, energy, temperature, heatflow,
                      conductivity, cv, viscosity, pressure, rho,
                      grad/v/ab, stress/v/ab, stress/t/ab, nbond/shell
```

## Examples

```
compute 1 all rheo/property/atom phase surface/r surface/n/* pressure
compute 2 all rheo/property/atom shift/v/x grad/v/xx stress/v/*
```

## Description

Added in version 29Aug2024.

Define a computation that stores atom attributes specific to the RHEO package for each atom in the group. This is useful so that the values can be used by other output commands that take computes as inputs.

Key attributes:
- **phase**: atom phase state (0 = fluid, 1 = solid)
- **surface**: surface designation (0 = bulk, 1 = surface, 2 = splash)
- **surface/r**: distance from the surface
- **surface/n/a**: component of surface normal vector
- **coordination**: coordination number
- **shift/v/a**: component of atom shifting velocity
- **energy**: atom energy
- **temperature**: atom temperature
- **viscosity**: atom viscosity (from fix rheo/viscosity)
- **pressure**: atom pressure
- **rho**: atom density
- **grad/v/ab**: component of velocity gradient tensor
- **stress/v/ab**: component of viscous stress tensor
- **stress/t/ab**: component of total stress tensor
- **nbond/shell**: number of oxide bonds

For vector attributes, specify the x, y, or z component or use wildcard *.
For tensor attributes, specify both components or use wildcard *.

This compute style is part of the RHEO package. It is only enabled if LAMMPS was built with that package.

## Related commands

dump custom, compute reduce, fix ave/atom, fix ave/chunk, fix rheo/viscosity, fix rheo/pressure, fix rheo/thermal, fix rheo/oxidation, fix rheo
