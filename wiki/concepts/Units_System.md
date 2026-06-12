# Units System / 单位系统

## 概念概述 / Concept Overview

LAMMPS uses self-consistent unit systems where all quantities are expressed in compatible units. The `units` command determines which unit system is used.

## 单位系统详解 / Unit Systems Details

### 1. lj (Lennard-Jones) / LJ 约化单位

| Quantity | Unit |
|----------|------|
| Mass | m (particle mass) |
| Distance | σ (LJ distance) |
| Time | τ (LJ time) |
| Energy | ε (LJ energy well depth) |
| Temperature | ε/kB |
| Pressure | ε/σ³ |
| Charge | e (elementary charge) |

**Common values:**
- timestep: 0.005 τ
- temperature: 1.0 ε/kB
- density: 0.8 σ⁻³

### 2. real (Real Units) / 实际单位

| Quantity | Unit |
|----------|------|
| Mass | grams/mol |
| Distance | Angstroms |
| Time | femtoseconds |
| Energy | kcal/mol |
| Temperature | Kelvin |
| Pressure | atmospheres |
| Charge | e (elementary charge) |

**Common values:**
- timestep: 1.0 fs
- temperature: 300 K
- pressure: 1.0 atm

### 3. metal (Metal Units) / 金属单位

| Quantity | Unit |
|----------|------|
| Mass | grams/mol |
| Distance | Angstroms |
| Time | picoseconds |
| Energy | eV |
| Temperature | Kelvin |
| Pressure | bars |
| Charge | e |

**Common values:**
- timestep: 0.001 ps (1 fs)
- temperature: 300 K
- pressure: 0.0 bar (vacuum)

### 4. cgs (CGS Units)

| Quantity | Unit |
|----------|------|
| Mass | grams |
| Distance | cm |
| Time | seconds |
| Energy | ergs |
| Temperature | Kelvin |
| Pressure | dyne/cm² |

### 5. si (SI Units)

| Quantity | Unit |
|----------|------|
| Mass | kilograms |
| Distance | meters |
| Time | seconds |
| Energy | Joules |
| Temperature | Kelvin |
| Pressure | Pascals |

### 6. electron (Electronic Units)

| Quantity | Unit |
|----------|------|
| Mass | atomic mass units (amu) |
| Distance | Bohr |
| Time | femtoseconds |
| Energy | Hartree |
| Temperature | Kelvin |
| Pressure | - |

## 单位转换 / Unit Conversion

### Temperature
```lammps
# real to metal
# 1 kcal/mol = 0.04336 eV
# 1 K = 8.617e-05 eV/kB

# Temperature conversion
# T(eV/kB) = T(K) × kB(eV/K)
# kB = 8.617e-05 eV/K
```

### Energy
```lammps
# kcal/mol to eV
# 1 kcal/mol = 0.04336 eV

# kcal/mol to J
# 1 kcal/mol = 4184 J/mol
```

### Pressure
```lammps
# atm to bar
# 1 atm = 1.01325 bar

# bar to Pa
# 1 bar = 100000 Pa
```

## 选择指南 / Selection Guidelines

### Biomolecules (Proteins, DNA)
```lammps
units real
# kcal/mol, fs, Angstroms
# Compatible with AMBER, CHARMM force fields
```

### Materials (Metals, Semiconductors)
```lammps
units metal
# eV, ps, Angstroms
# Compatible with DFT, materials force fields
```

### Generic MD Studies
```lammps
units lj
# Reduced units
# Simplifies analysis, removes scale dependencies
```

### Engineering Applications
```lammps
units si
# SI base units
# Direct comparison with experimental conditions
```

### Coarse-Grained
```lammps
units real
# Or custom units
# Depends on CG model definition
```

## 陷阱 / Pitfalls

### 1. Forgetting units command
```lammps
# WRONG - no units specified
pair_style lj/cut 10.0  # What units?

# CORRECT
units real
pair_style lj/cut 10.0  # 10.0 Angstroms
```

### 2. Mixing unit systems
```lammps
# WRONG - mixing units
units real
pair_style lj/cut 10.0
fix 1 all nvt temp 300 300 100  # What units for temp?

# CORRECT
units real
pair_style lj/cut 10.0  # Angstroms
fix 1 all nvt temp 300.0 300.0 100.0  # Kelvin
```

### 3. Wrong timestep for units
```lammps
# WRONG - timestep too large for real units
units real
timestep 10.0  # 10 fs, may be unstable

# CORRECT
units real
timestep 1.0  # 1 fs, typical for biomolecules
```

## 相关概念 / Related Concepts

- **Physical constants**: kB, e, ε0
- **Force fields**: Unit-dependent parameters
- **Conversion factors**: Between unit systems

## 相关命令 / Related Commands

- `units`: Set unit system
- `timestep`: Set timestep (unit-dependent)
- `thermo_style`: Output quantities (unit-dependent)

## 参考资料 / References

:::info
**Source**: LAMMPS units documentation
:::
