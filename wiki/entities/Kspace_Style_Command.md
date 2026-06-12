# Kspace Style Command / K空间样式命令

## 实体概述 / Entity Overview

Defines the method for computing long-range Coulombic interactions via reciprocal space.

## 语法格式 / Syntax Format

```lammps
kspace_style style args
```

## 支持的样式 / Supported Styles

| Style | Description | Package |
|-------|-------------|---------|
| `ewald` | Standard Ewald summation | KSPACE |
| `pppm` | Particle-Particle Particle-Mesh | KSPACE |
| `pppm/tip4p` | PPPM for TIP4P water | KSPACE |
| `msm` | Multilevel summation | KSPACE |
| `dispersion` | Dispersion corrections | KSPACE |
| `wolf` | Wolf method | - |

## 使用示例 / Usage Example

```lammps
# Standard Ewald
kspace_style ewald 1.0e-6

# PPPM
kspace_style pppm 1.0e-4

# PPPM with specified accuracy
kspace_style pppm 1.0e-5

# MSM
kspace_style msm 1.0e-4

# PPPM for TIP4P water
kspace_style pppm/tip4p 1.0e-4
```

## 参数说明 / Parameter Description

### Accuracy Parameter
- Specifies target accuracy (relative error)
- Typical values: 1.0e-4 to 1.0e-6
- Smaller value = higher accuracy, slower

### Method Comparison / 方法比较

| Method | Pros | Cons | Use Case |
|--------|------|------|----------|
| Ewald | Accurate | Slow (O(N^(3/2))) | Small systems |
| PPPM | Fast (O(N log N)) | Complex setup | Large systems |
| MSM | Fast, parallel | Memory intensive | Very large systems |
| Wolf | Simple, no FFT | Less accurate | Quick tests |

## Ewald 参数 / Ewald Parameters

```lammps
kspace_style ewald 1.0e-6
kspace_modify ewald/rho 1.0e-4
kspace_modify ewald/gewald 0.25
```

## PPPM 参数 / PPPM Parameters

```lammps
kspace_style pppm 1.0e-4

# Modify grid
kspace_modify mesh 8 8 8

# Modify order
kspace_modify order 6

# Mix embedding
kspace_modify mix one
```

## MSM 参数 / MSM Parameters

```lammps
kspace_style msm 1.0e-4

# Modify levels
kspace_modify levels 3
```

## 精度控制 / Accuracy Control

```lammps
# Relative accuracy
kspace_style pppm 1.0e-5

# Absolute accuracy
kspace_style pppm 1.0e-5 absolute

# Slab correction
kspace_style pppm 1.0e-4 slab 3.0
```

## 相关命令 / Related Commands

- `kspace_modify`: Modify kspace parameters
- `pair_style lj/cut/coul/long`: Compatible pair style
- `dielectric`: Set dielectric constant

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/kspace_style.md`
:::
