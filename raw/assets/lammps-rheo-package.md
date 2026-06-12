# LAMMPS RHEO Package - Complete Reference

Source: https://docs.lammps.org/ (multiple RHEO pages), compiled 2026-06-12

## Overview

The RHEO package (Added in version 29Aug2024) is a hybrid implementation of smoothed particle hydrodynamics (SPH) for fluid flow in LAMMPS. It can couple to the BPM package to model solid elements, enabling mesh-free modeling of multi-phase material systems.

Published in: Palermo, Wolf, Clemmer, O'Connor, Phys. Fluids, 36, 113337 (2024).

## Complete Command List

### Fix Commands
| Command | Purpose |
|---------|---------|
| `fix rheo` | Core SPH integration, position/velocity/density update |
| `fix rheo/pressure` | Pressure equation of state |
| `fix rheo/viscosity` | Viscosity model (constant or Herschel-Bulkley) |
| `fix rheo/thermal` | Temperature integration and thermal properties |
| `fix rheo/oxidation` | Dynamic surface bond creation for oxidation |

### Pair Styles
| Command | Purpose |
|---------|---------|
| `pair_style rheo` | Pressure and viscous forces between SPH particles |
| `pair_style rheo/solid` | Contact forces between solid RHEO bodies |

### Bond Styles
| Command | Purpose |
|---------|---------|
| `bond_style rheo/shell` | Elastic shell bonds for oxidation modeling |

### Compute Styles
| Command | Purpose |
|---------|---------|
| `compute rheo/property/atom` | Access RHEO-specific atom properties |

### Atom Styles
| Style | Purpose |
|-------|---------|
| `atom_style rheo` | Basic RHEO (density, viscosity, pressure, status) |
| `atom_style rheo/thermal` | RHEO with thermal evolution (energy, temperature, conductivity) |

## Typical RHEO Input Script Structure

```
# RHEO fluid simulation example
units           lj
dimension       3
boundary        p p p
atom_style      rheo

# Create system
lattice         fcc 0.8
region          box block 0 20 0 20 0 20
create_box      1 box
create_atoms    1 box

# Define pair style
pair_style      rheo 3.0 rho/damp 1.0 artificial/visc 2.0
pair_coeff      * *

# Define RHEO fixes
fix             1 all rheo 3.0 quintic 0 density 0.1 speed/sound 10.0
fix             2 all rheo/pressure * linear
fix             3 all rheo/viscosity * constant 1.0

# Run
timestep        0.001
run              10000
```

## RHEO with Thermal Evolution

```
atom_style      rheo/thermal
# ... (same setup as above, plus)
fix             1 all rheo 3.0 quintic 0 thermal density 0.1 speed/sound 10.0
fix             4 all rheo/thermal conductivity * constant 1.0 specific/heat * constant 1.0
```

## RHEO with Elastic Solids

```
# Create solid body with BPM bonds
atom_style      hybrid bond rheo
set             atom 1*100 atom 1 status 1  # Set particles to solid

pair_style      hybrid rheo 3.0 rheo/solid
pair_coeff      * * rheo 3.0
pair_coeff      * * rheo/solid 1.0 1.5 1.0

bond_style      bpm/spring
fix             1 all rheo 3.0 RK1 10 shift interface/reconstruct
```

## RHEO with Oxidation

```
bond_style      hybrid bpm/spring rheo/shell t/form 10.0
fix             5 all rheo/oxidation 1.5 2 0.0
```

## Kernel Types

| Kernel | Description |
|--------|-------------|
| `quintic` | Standard quintic spline (SPH standard) |
| `RK0` | Zeroth-order reproducing kernel |
| `RK1` | First-order reproducing kernel |
| `RK2` | Second-order reproducing kernel |

## Pressure Equations of State

| Style | Formula |
|-------|---------|
| linear | P = c^2 * (rho - rho_0) |
| cubic | P = c^2 * ((rho - rho_0) + A3*(rho - rho_0)^3) |
| tait/water | P = (c^2 * rho_0 / 7) * [(rho/rho_0)^7 - 1] |
| tait/general | P = (c^2 * rho_0 / gamma) * [(rho/rho_0)^gamma - 1] |
| ideal/gas | P = (gamma-1) * rho * e |

## Viscosity Models

| Style | Description |
|-------|-------------|
| constant | Constant viscosity eta |
| power | Herschel-Bulkley with critical strain rate regularization |

## Build Requirements

LAMMPS must be built with the RHEO package enabled:
```bash
cmake -DPKG_RHEO=on ../cmake
# or
make yes-rheo
```

## Sources

- https://docs.lammps.org/fix_rheo.html
- https://docs.lammps.org/pair_rheo.html
- https://docs.lammps.org/Howto_rheo.html
- https://docs.lammps.org/fix_rheo_pressure.html
- https://docs.lammps.org/fix_rheo_viscosity.html
- https://docs.lammps.org/fix_rheo_thermal.html
- https://docs.lammps.org/fix_rheo_oxidation.html
- https://docs.lammps.org/bond_rheo_shell.html
- https://docs.lammps.org/pair_rheo_solid.html
- https://docs.lammps.org/compute_rheo_property_atom.html
