# LAMMPS New Features 2024-2025 - Changelog Summary

Source: https://www.lammps.org/bug2025.html, https://www.lammps.org/bug2024.html, compiled 2026-06-12

## New Pair Styles

### pair_style dispersion/d3 (Added 4Feb2025)
DFT-D3 dispersion energy correction. Computes dispersion correction based on Grimme's D3 method. Used as overlay with ML potentials via hybrid/overlay.
- Damping functions: original, zerom, bj, bjm
- XC functionals: pbe, pbe0, b3-lyp, revpbe, rpbe, tpss, hse06, and many more
- Part of EXTRA-PAIR package

### pair_style rheo (Added 29Aug2024)
Pressure and viscous forces for RHEO SPH package.

### pair_style rheo/solid (Added 29Aug2024)
Contact forces between solid RHEO bodies.

### Additional new/modified pair styles (from pair_style.html listing):
- `pair_style pace/apip`, `pace/fast/apip`, `pace/precise/apip` - Active learning PACE variants
- `pair_style lepton/coul`, `lepton/sphere` - Lepton expression pair styles
- `pair_style uf3` - Ultra-Fast Force Field
- `pair_style mie/cut` - Mie potential
- `pair_style coul/exclude` - Coulomb with exclusion
- `pair_style coul/ctip` - CTIP charge transfer
- `pair_style coul/slater/cut`, `coul/slater/long` - Slater-type Coulomb
- `pair_style pedone` - Pedone potential
- `pair_style mbx` - MBX many-body potential
- `pair_style rebomos` - ReboMOS potential
- `pair_style nb3b/screened` - Screened three-body
- `pair_style lj/pirani` - Pirani potential
- `pair_style gauss/cut` - Gaussian cutoff
- `pair_style lennard/mdf` - Lennard-MDF potential
- `pair_style hybrid/molecular`, `hybrid/scaled` - New hybrid variants
- `pair_style lambda/input/apip`, `lambda/input/csp/apip`, `lambda/zone/apip` - Lambda APIP variants
- `pair_style eam/apip`, `eam/fs/apip` - EAM APIP variants

## New Fix Styles

### RHEO package fixes (Added 29Aug2024):
- `fix rheo` - Core RHEO SPH integration
- `fix rheo/pressure` - Pressure equation of state
- `fix rheo/viscosity` - Viscosity model
- `fix rheo/thermal` - Thermal evolution
- `fix rheo/oxidation` - Surface oxidation bonding

### Other new fix styles:
- `fix add/heat` - Add heat to particles
- `fix addtorque/atom`, `fix addtorque/group` - Apply torques
- `fix align/self` - Self-alignment
- `fix atom_weight/apip` - APIP atom weights
- `fix deform/pressure` - Pressure-driven deformation
- `fix efield/tip4p` - Electric field with TIP4P water
- `fix efield/lepton` - Lepton expression electric field
- `fix electrode/conp`, `electrode/conq`, `electrode/thermo` - Electrode models
- `fix electron/stopping`, `electron/stopping/fit` - Electron stopping
- `fix gjf` - G-JF integrator
- `fix hmc` - Hybrid Monte Carlo
- `fix heat/flow` - Heat flow
- `fix lambda/apip`, `fix lambda_thermostat/apip` - APIP lambda dynamics
- `fix neighbor/swap` - Neighbor swapping
- `fix nonaffine/displacement` - Non-affine displacement
- `fix pimd/langevin`, `pimd/nvt` (and bosonic variants) - Path integral MD
- `fix press/langevin` - Langevin barostat
- `fix brownian/sphere`, `brownian/asphere` - Brownian dynamics variants
- `fix set` - Set atom properties via fix
- `fix settorque/atom` - Set atom torques
- `fix wall/flow` - Flow wall
- `fix wall/lepton` - Lepton expression wall
- `fix wall/harmonic/outside` - Outside harmonic wall
- `fix qeq/ctip` - CTIP charge equilibration
- `fix qeq/point`, `qeq/shielded`, `qeq/slater` - QEQ variants
- `fix qtpie/reaxff` - QTPIE for ReaxFF
- `fix tgnvt/drude`, `tgnpt/drude` - Targeted G-NVT/NPT for Drude

## New Bond Styles
- `bond_style rheo/shell` - RHEO shell bonds for oxidation modeling (Added 29Aug2024)

## New Compute Styles
- `compute rheo/property/atom` - RHEO atom properties (Added 29Aug2024)

## New Atom Styles
- `atom_style rheo` - RHEO basic
- `atom_style rheo/thermal` - RHEO with thermal

## Howto Pages
- `Howto_rheo` - RHEO SPH fluid dynamics (Added 29Aug2024)

## Build System
- `Build_diskspace` - Disk space requirements

## Sources
- https://www.lammps.org/bug2025.html
- https://www.lammps.org/bug2024.html
- https://docs.lammps.org/pair_style.html (complete pair style listing)
