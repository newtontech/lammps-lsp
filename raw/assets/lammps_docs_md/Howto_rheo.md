# 10.5.11. Reproducing hydrodynamics and elastic objects (RHEO)

Source: https://docs.lammps.org/Howto_rheo.html

The RHEO package is a hybrid implementation of smoothed particle hydrodynamics (SPH) for fluid flow, which can couple to the BPM package to model solid elements. RHEO combines these methods to enable mesh-free modeling of multi-phase material systems. Its SPH solver supports many advanced options including reproducing kernels, particle shifting, free surface identification, and solid surface reconstruction.

At the core of the package is fix rheo which integrates particle trajectories and controls many optional features (e.g. the use of reproducing kernels). In conjunction to fix rheo, one must specify an instance of fix rheo/pressure and fix rheo/viscosity to define a pressure equation of state and viscosity model, respectively. Optionally, one can model a heat equation with fix rheo/thermal.

Typically, RHEO requires atom style rheo. In addition to typical atom properties like positions and forces, particles store a local density, viscosity, pressure, and status. If thermal evolution is modeled, one must use atom style rheo/thermal which also includes a local energy, temperature, and conductivity.

The status variable uses bit-masking to track various properties of a particle such as its current state of matter (fluid or solid) and its location relative to a surface.

Fluid interactions are calculated using pair rheo. Unlike typical pair styles, pair rheo ignores the special bond settings. Instead, it determines whether to calculate forces based on the status of particles.

## Elastic Objects in RHEO

Two mechanisms for elastic objects:

### Bulk Solid Bodies
1. Change status of particles to solid (set command)
2. Create BPM bonds between particles
3. Use pair rheo/solid for repulsive contact forces

### Thin Shells (e.g. oxide skin)
- Implemented using fix rheo/oxidation and bond style rheo/shell
- Creates candidate bonds between surface fluid particles
- Newly created bonds start a timer; activated bonds apply additional forces

## Key RHEO Commands

| Command | Purpose |
|---------|---------|
| fix rheo | Core SPH integration |
| fix rheo/pressure | Equation of state |
| fix rheo/viscosity | Viscosity model |
| fix rheo/thermal | Heat equation |
| fix rheo/oxidation | Surface bond creation |
| pair rheo | Pressure & viscous forces |
| pair rheo/solid | Solid body contact forces |
| bond rheo/shell | Shell/oxidation bonds |
| compute rheo/property/atom | Access RHEO atom attributes |

## References

(Palermo) Palermo, Wolf, Clemmer, O'Connor, Phys. Fluids, 36, 113337 (2024).
(Clemmer) Clemmer, Pierce, O'Connor, Nevins, Jones, Lechman, Tencer, Appl. Math. Model., 130, 310-326 (2024).
