# Statistical Ensembles / 统计系综

## 概念概述 / Concept Overview

In statistical mechanics, an ensemble is a collection of microstates representing a thermodynamic system. LAMMPS simulates various ensembles through appropriate fix combinations.

## 主要系综 / Major Ensembles

### 1. NVE Ensemble / 微正则系综
- **Variables**: Constant Number (N), Volume (V), Energy (E)
- **Fix**: `fix nve`
- **Use Case**: Isolated systems, energy conservation studies

```lammps
fix 1 all nve
run 10000
```

### 2. NVT Ensemble / 正则系综
- **Variables**: Constant N, V, Temperature (T)
- **Fixes**: `fix nvt`, `fix langevin`, `fix temp/berendsen`
- **Use Case**: Most MD simulations, canonical ensemble

```lammps
# Nose-Hoover
fix 1 all nvt temp 300.0 300.0 100.0

# Langevin
fix 1 all nve
fix 2 all langevin 300.0 300.0 100.0 48279
```

### 3. NPT Ensemble / 等温等压系综
- **Variables**: Constant N, Pressure (P), T
- **Fixes**: `fix npt`, `fix nph`
- **Use Case**: Pressure-controlled simulations

```lammps
# Isotropic pressure control
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0

# Anisotropic pressure
fix 2 all npt temp 300.0 300.0 100.0 &
  aniso 0.0 0.0 1000.0
```

### 4. NPH Ensemble / 等压等焓系综
- **Variables**: Constant N, P, Enthalpy (H)
- **Fix**: `fix nph`
- **Use Case**: Adiabatic compression/expansion

```lammps
fix 1 all nph iso 0.0 0.0 1000.0
```

### 5. muVT Ensemble / 巨正则系综
- **Variables**: Constant Chemical potential (mu), V, T
- **Fix**: `fix gcmc`
- **Use Case**: Adsorption, grand canonical Monte Carlo

```lammps
fix 1 all gcmc 100 100 0 0 0 29814 1.0
```

## 恒温器类型 / Thermostat Types

### 1. Nose-Hoover
```lammps
fix 1 all nvt temp 300.0 300.0 100.0
# Tstart, Tstop, Tdamp
```

### 2. Langevin
```lammps
fix 1 all nve
fix 2 all langevin 300.0 300.0 100.0 48279
# Tstart, Tstop, Tdamp, seed
```

### 3. Berendsen
```lammps
fix 1 all temp/berendsen 300.0 300.0 100.0
```

## 恒压器类型 / Barostat Types

### 1. Nose-Hoover Barostat
```lammps
fix 1 all npt temp 300.0 300.0 100.0 &
  iso 0.0 0.0 1000.0
# Pstart, Pstop, Pdamp
```

### 2. Berendsen Barostat
```lammps
fix 1 all press/berendsen iso 0.0 0.0 1000.0
```

## 选择指南 / Selection Guidelines

| Property | NVE | NVT | NPT |
|----------|-----|-----|-----|
| Energy | Conserved | Fluctuating | Fluctuating |
| Volume | Constant | Constant | Fluctuating |
| Pressure | Fluctuating | Fluctuating | Controlled |
| Temperature | Fluctuating | Controlled | Controlled |

## 典型工作流 / Typical Workflow

```lammps
# 1. Minimization
minimize 1.0e-4 1.0e-6 100 1000

# 2. NVE equilibration
fix 1 all nve
run 10000

# 3. NVT equilibration
unfix 1
fix 2 all nvt temp 300.0 300.0 100.0
run 10000

# 4. NPT production
unfix 2
fix 3 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
run 100000
```

## 相关概念 / Related Concepts

- **Thermostat**: Temperature control
- **Barostat**: Pressure control
- **Time integration**: Equations of motion

## 参考资料 / References

:::info
**Source**: LAMMPS statistical mechanics documentation
:::
