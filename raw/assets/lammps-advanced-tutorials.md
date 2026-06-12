# LAMMPS Advanced Tutorials and Topics - Comprehensive Reference

Source: https://docs.lammps.org/Howto.html, https://www.lammps.org/tutorials.html, compiled 2026-06-12

## Advanced Howto Topics (docs.lammps.org)

The LAMMPS documentation includes extensive Howto guides for advanced use cases.

### Simulation Methodology

| Howto | Description |
|-------|-------------|
| Howto_2d | Running 2D simulations |
| Howto_barostat | Pressure control (barostats) |
| Howto_thermostat | Temperature control (thermostats) |
| Howto_temperature | Temperature calculation methods |
| Howto_diffusion | Computing diffusion coefficients |
| Howto_viscosity | Computing viscosity |
| Howto_kappa | Computing thermal conductivity |
| Howto_elastic | Computing elastic constants |
| Howto_nemd | Non-equilibrium MD (NEMD) |
| Howto_replica | Multi-replica simulations |
| Howto_restart | Checkpoint and restart |

### Advanced Force Fields

| Howto | Description |
|-------|-------------|
| Howto_amoeba | AMOEBA polarizable force field |
| Howto_bioFF | Biomolecular force fields |
| Howto_coreshell | Core-shell models |
| Howto_drude / Howto_drude2 | Drude polarizable models |
| Howto_polarizable | Polarizable force fields |
| Howto_dispersion | Dispersion corrections |
| Howto_spc | SPC water models |
| Howto_tip3p | TIP3P water model |
| Howto_tip4p | TIP4P water model |
| Howto_tip5p | TIP5P water model |

### Specialized Simulation Types

| Howto | Description |
|-------|-------------|
| Howto_body | Rigid body particles |
| Howto_bpm | Bonded particle models |
| Howto_broken_bonds | Tracking broken bonds |
| Howto_chunk | Chunk-based analysis |
| Howto_granular | Granular simulations |
| Howto_peri | Peridynamics |
| Howto_spins | Magnetic spin models |
| Howto_spherical | Spherical particle simulations |
| Howto_walls | Wall boundaries |
| Howto_manifold | Manifold (surface) constraints |
| Howto_rheo | RHEO SPH fluid dynamics |

### Performance and Acceleration

| Page | Description |
|------|-------------|
| Speed_gpu | GPU acceleration |
| Speed_intel | Intel optimizations (Phi, AVX) |
| Speed_kokkos | Kokkos portability (GPU/Phi) |
| Speed_omp | OpenMP threading |
| Speed_opt | OPT package optimizations |
| Speed_packages | Package-specific speedups |
| Speed_tips | General performance tips |
| Speed_bench | Benchmark comparisons |
| Speed_compare | Comparing acceleration packages |

### Advanced Input/Output

| Howto | Description |
|-------|-------------|
| Howto_output | Output options overview |
| Howto_structured_data | Structured data output |
| Howto_type_labels | Type label management |
| Howto_viz | Visualization methods |
| Howto_grid | Grid-based computations |

### Coupling and Integration

| Howto | Description |
|-------|-------------|
| Howto_couple | Coupling LAMMPS to other codes |
| Howto_library | Library interface to LAMMPS |
| Howto_mdi | MolSSI Driver Interface |
| Howto_multiple | Multiple LAMMPS instances |
| Howto_pylammps | Using PyLammps |
| Howto_github | Contributing on GitHub |
| Howto_cmake | CMake build system |
| Howto_wsl | Using LAMMPS on WSL |
| Howto_triclinic | Triclinic (non-orthogonal) boxes |

## External Tutorials

### Official LAMMPS Tutorials (lammps.org/tutorials.html)
- Step-by-step tutorials for beginners and intermediate users
- Covers several different kinds of systems

### arXiv Tutorial Paper (2025)
- "A Set of Tutorials for the LAMMPS Simulation Package" (arXiv:2503.14020v2)
- Published March 2025, introduces a suite of tutorials

### MatSci.org Community
- Forum discussions about LAMMPS documentation gaps
- Topic of the Month threads on LAMMPS documentation

## Advanced Topics Not in Standard Docs

### Parallel Tempering (Replica Exchange)
```
temper N M temp fix-ID seed1 seed2 index
temper_npt N M temp fix-ID seed1 seed2 index press-ID
temper_grem N M lambda fix-ID seed1 seed2 index
```

### Metadynamics
Available via COLVARS and PLUMED interfaces:
```
fix colvars all colvars config_file
fix plumed all plumed plumedfile <plumed.dat
```

### Path Integrals (PIMD)
```
fix pimd/langevin fix pimd/nvt
```

### Hyperdynamics
```
fix hyper/global fix hyper/local
```

### NEB (Nudged Elastic Band)
```
neb etol ftol N1 N2 Nevery file
neb_spin etol ftol N1 N2 Nevery file
```

### Parallel Replica Dynamics
```
prd t_event t_corr n_dephase t_dephase command
```

### Temperature Accelerated Dynamics
```
tad t_event t_corr n_dephase t_dephase t_push distance command
```

## Sources

- https://docs.lammps.org/Howto.html
- https://www.lammps.org/tutorials.html
- https://docs.lammps.org/Speed.html
- https://arxiv.org/html/2503.14020v2
- https://matsci.org/t/topic-of-the-month-lammps-documentation/40919
