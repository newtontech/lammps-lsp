# Compute Command / 计算命令

## 实体概述 / Entity Overview

Calculates instantaneous properties (temperature, pressure, stress, etc.) during a simulation.

## 语法格式 / Syntax Format

```lammps
compute ID group-name style style-args
```

- `ID`: User-defined name for the compute
- `group-name`: Atom group to compute over
- `style`: Compute style name
- `style-args`: Style-specific arguments

## 主要分类 / Main Categories

### 1. Thermodynamic Properties / 热力学性质

- `compute temp`: Temperature
- `compute press`: Pressure (requires pressure compute)
- `compute pe`: Potential energy
- `compute ke`: Kinetic energy

### 2. Structural Properties / 结构性质

- `compute rdf`: Radial distribution function
- `compute com`: Center of mass
- `compute gyration`: Radius of gyration
- `compute msd`: Mean squared displacement

### 3. Stress/Strain / 应力应变

- `compute stress/atom`: Per-atom stress
- `compute pressure/atom`: Per-atom pressure
- `compute reduce`: Sum/reduce values

### 4. Chunk-based / 块计算

- `compute chunk/atom`: Assign atoms to chunks
- `compute temp/chunk`: Temperature per chunk
- `compute property/chunk`: Properties per chunk

## 使用示例 / Usage Example

```lammps
# Temperature
compute 1 all temp

# Pressure (requires temperature compute)
compute 2 all pressure temp 1

# RDF
compute 3 all rdf 100
fix 1 all ave/time 100 1 100 c_3[1] c_3[2] file rdf.dat

# Per-atom stress
compute 4 all stress/atom NULL

# Chunk-based temperature
compute 5 all chunk/atom molecule
compute 6 all temp/chunk 5
```

## Compute Output / 计算输出

Computes can be accessed via:
- `thermo_style`: Thermodynamic output
- `fix ave/time`: Time averaging
- `fix ave/correlate`: Correlation functions
- `dump custom`: Custom dump output

## 相关命令 / Related Commands

- `fix ave/time`: Time averaging
- `thermo_style`: Thermodynamic output
- `dump`: Output compute values
- `uncompute`: Remove a compute

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/compute_*.md`
:::
