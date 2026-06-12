# Units Command / 单位命令

## 实体概述 / Entity Overview

Defines the unit system for the simulation. Must appear before other simulation setup commands.

## 语法格式 / Syntax Format

```lammps
units style
```

## 支持的单位系统 / Supported Unit Systems

| Style | Distance | Time | Mass | Energy | Pressure |
|-------|----------|------|-------|---------|----------|
| `lj` | sigma | tau | m | epsilon | epsilon/sigma^3 |
| `real` | Angstroms | femtoseconds | grams | kcal/mol | atmospheres |
| `metal` | Angstroms | picoseconds | grams | eV | bars |
| `cgs` | cm | s | g | ergs | dynes/cm^2 |
| `electron` | Bohr | fs | amu | Hartree | |
| `micro` | microns | microseconds | grams | fg | picobar |
| `nano` | nm | ns | pg | fJ | nbar |
| `si` | meters | seconds | kg | Joules | Pascals |

## 使用示例 / Usage Example

```lammps
# Lennard-Jones reduced units
units lj

# Real units (common for biomolecules)
units real

# Metal units (common for materials)
units metal

# SI units
units si
```

## 单位选择建议 / Unit Selection Guidelines

| Field | Common Units |
|-------|--------------|
| Biomolecules | `real` (kcal/mol, fs, Angstrom) |
| Materials | `metal` (eV, ps, Angstrom) |
| Generic MD | `lj` (reduced units) |
| Engineering | `si` (SI base units) |

## 单位转换 / Unit Conversion

```lammps
# Converting between real and metal
# 1 eV = 23.060 kcal/mol
# 1 bar = 0.987 atm
```

## 常见陷阱 / Common Pitfalls

- Units command **must come first** in input script
- Mixing units from different systems causes errors
- `thermo` output units depend on this setting
- Pair style coefficients must use consistent units

## 相关命令 / Related Commands

- `thermo_style`: Output unit-dependent quantities
- `variable`: Define conversion variables

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/units.md`
:::
