# RHEO Package Commands

**Category**: LAMMPS Package - Smoothed Particle Hydrodynamics
**Added in**: version 29Aug2024
**Source**: docs.lammps.org

## Overview / 概述

The RHEO package implements a hybrid smoothed particle hydrodynamics (SPH) method for fluid flow in LAMMPS. It supports coupling with the BPM package for solid elements, enabling mesh-free modeling of multi-phase material systems.

RHEO包实现了混合光滑粒子流体动力学(SPH)方法，支持与BPM包耦合建模固体元素，实现多相材料系统的无网格建模。

## Core Commands / 核心命令

### fix rheo
Core time integration for RHEO particles. Updates positions, velocities, and densities.

```
fix ID group-ID rheo cut kstyle zmin keyword values...
```

- `kstyle`: quintic, RK0, RK1, RK2
- Keywords: thermal, interface/reconstruct, surface/detection, shift, rho/sum, density, speed/sound
- Must be used with atom style `rheo` or `rheo/thermal`
- Must be defined before all other RHEO fixes

### fix rheo/pressure
Defines pressure equation of state.

```
fix ID group-ID rheo/pressure types pstyle args
```

Pressure styles: linear, tait/water, tait/general, cubic, ideal/gas, background

### fix rheo/viscosity
Defines viscosity model.

```
fix ID group-ID rheo/viscosity types vstyle args
```

Viscosity styles: constant, power (Herschel-Bulkley)

### fix rheo/thermal
Temperature integration and thermal properties.

```
fix ID group-ID rheo/thermal attribute values ...
```

Attributes: conductivity, specific/heat, latent/heat, Tfreeze, react

### fix rheo/oxidation
Dynamic surface bond creation for oxidation.

```
fix ID group-ID rheo/oxidation cut btype rsurf
```

## Pair Styles / 对势样式

### pair_style rheo
Pressure and viscous forces between SPH particles.

```
pair_style rheo cutoff keyword values
```

Keywords: rho/damp, artificial/visc, harmonic/means

### pair_style rheo/solid
Contact forces between solid RHEO bodies (based on bpm/spring).

```
pair_style rheo/solid
pair_coeff * * k r_c gamma
```

## Bond Style / 键样式

### bond_style rheo/shell
Elastic shell bonds for oxidation modeling.

```
bond_style rheo/shell t/form time_value
```

## Compute / 计算

### compute rheo/property/atom
Access RHEO-specific atom attributes: phase, surface, surface/r, coordination, viscosity, pressure, rho, energy, temperature, grad/v/*, stress/v/*, stress/t/*, nbond/shell

## Atom Styles / 原子样式

- `atom_style rheo`: density, viscosity, pressure, status
- `atom_style rheo/thermal`: adds energy, temperature, conductivity

## Typical Workflow / 典型工作流

```
atom_style      rheo
pair_style      rheo 3.0
pair_coeff      * *

fix             1 all rheo 3.0 quintic 0 density 0.1 speed/sound 10.0
fix             2 all rheo/pressure * linear
fix             3 all rheo/viscosity * constant 1.0

timestep        0.001
run              10000
```

## Build / 构建

```bash
cmake -DPKG_RHEO=on ../cmake
```

## References / 参考文献

Palermo, Wolf, Clemmer, O'Connor, Phys. Fluids, 36, 113337 (2024).
Clemmer, Pierce, O'Connor, Nevins, Jones, Lechman, Tencer, Appl. Math. Model., 130, 310-326 (2024).
