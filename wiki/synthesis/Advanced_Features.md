# Advanced LAMMPS Features and Workflows

**Category**: Synthesis - Advanced Workflows
**Source**: docs.lammps.org, raw/assets/lammps-advanced-tutorials.md

## Overview / 概述

This page covers advanced LAMMPS features beyond basic MD simulations, including enhanced sampling, free energy methods, multiscale modeling, and specialized simulation packages.

本页涵盖基本MD模拟之外的高级LAMMPS功能，包括增强采样、自由能方法、多尺度建模和专用模拟包。

## Enhanced Sampling Methods / 增强采样方法

### Metadynamics
Via COLVARS or PLUMED interface:
```
fix 1 all colvars config.in
# or
fix 1 all plumed plumedfile plumed.dat
```

### Replica Exchange (Parallel Tempering)
```
temper N M temp fix-ID seed1 seed2 index
temper_npt N M temp fix-ID seed1 seed2 index press-ID
temper_grem N M lambda fix-ID seed1 seed2 index
```

### Hyperdynamics
```
fix 1 all hyper/global ...
fix 1 all hyper/local ...
```

### Well-Tempered Metadynamics
Via PLUMED plugin (fix plumed).

## Free Energy Methods / 自由能方法

### Alchemical Free Energy
```
fix 1 all alchemy ...
compute fep all fep ...
compute ti all ti ...
```

### Thermodynamic Integration
```
fix ti_spring all ti/spring ...
```

### Widom Insertion
```
fix 1 all widom ...
```

## Multiscale Methods / 多尺度方法

### QM/MM Coupling
```
fix 1 all qmmm ...
# or via MDI:
fix 1 all mdi/qm ...
fix 1 all mdi/qmmm ...
```

### Coupling with External Codes
```
# Library interface (C/C++/Python)
# See Howto_couple, Howto_library
```

### RHEO for Fluid-Structure
```
fix 1 all rheo 3.0 quintic 0 thermal
fix 2 all rheo/pressure * linear
fix 3 all rheo/viscosity * constant 1.0
pair_style rheo 3.0
```

## Machine Learning Workflows / 机器学习工作流

### Active Learning with PACE
```
pair_style pace/apip
pair_coeff * * potential.yaml Cu Ni
```

### ML-IAP with PyTorch
```python
from lammps import lammps
lmp = lammps()
lmp.command("pair_style mliap model mliappy model.pt descriptor sna ...")
```

### D3 Dispersion with ML Potentials (New 4Feb2025)
```
pair_style hybrid/overlay pace dispersion/d3 original pbe 30.0 20.0
```

## Path Integrals and Quantum Effects / 路径积分和量子效应

### PIMD
```
fix 1 all pimd/langevin ...
fix 1 all pimd/nvt ...
# Bosonic variants available
```

### NEB (Nudged Elastic Band)
```
neb etol ftol N1 N2 Nevery file
neb_spin etol ftol N1 N2 Nevery file
```

### TAD (Temperature Accelerated Dynamics)
```
tad t_event t_corr n_dephase t_dephase t_push distance command
```

### PRD (Parallel Replica Dynamics)
```
prd t_event t_corr n_dephase t_dephase command
```

## Specialized Simulation Packages / 专用模拟包

### RHEO (Fluid Dynamics, Added 29Aug2024)
- SPH-based fluid simulation
- Supports fluid-solid coupling
- Thermal evolution and oxidation modeling

### Granular
```
pair_style granular
fix 1 all pour ...
```

### Peridynamics
```
pair_style peri/pmb
pair_style peri/lps
pair_style peri/ves
```

### Magnetic Spins
```
pair_style spin/exchange
fix 1 all langevin/spin ...
```

## Performance Optimization / 性能优化

### GPU Acceleration
```
package gpu 1
pair_style lj/cut/gpu
```

### KOKKOS (Portable Performance)
```
package kokkos newton on neigh full
pair_style lj/cut/kk
```

### Intel Optimizations
```
package intel 0
pair_style lj/cut/intel
```

### OpenMP Threading
```
package omp 4
pair_style lj/cut/omp
```

## Plugin System / 插件系统

Load custom styles without recompiling:
```
plugin load my_plugin.so
pair_style my/custom/style
```

## Reference Table: Advanced Methods / 高级方法参考表

| Method | Primary Command | Package |
|--------|----------------|---------|
| Metadynamics | fix colvars/plumed | COLVARS/PLUMED |
| Parallel Tempering | temper | REPLICA |
| Hyperdynamics | fix hyper_* | HYPER |
| QM/MM | fix mdi/qm | MDI |
| PIMD | fix pimd/* | QTB |
| NEB | neb | REPLICA |
| TAD | tad | REPLICA |
| PRD | prd | REPLICA |
| ML Potentials | pair_style pace/snap/mliap | ML-* |
| RHEO SPH | fix rheo | RHEO |
| Plugins | plugin load | CORE |

## Sources / 来源

- docs.lammps.org/Howto.html
- raw/assets/lammps-advanced-tutorials.md
- raw/assets/lammps-ml-potentials.md
- raw/assets/lammps-rheo-package.md
- raw/assets/lammps-new-features-2024-2025.md
