# Pair Coeff Command / 配对系数命令

## 实体概述 / Entity Overview

Sets pair style coefficients for specific atom type pairs.

## 语法格式 / Syntax Format

```lammps
pair_coeff atom_type1 atom_type2 coeff_args
pair_coeff * * coeff_args
```

## 参数说明 / Parameter Description

| Parameter | Description |
|-----------|-------------|
| atom_type1 | First atom type (integer or * for wildcard) |
| atom_type2 | Second atom type (integer or * for wildcard) |
| coeff_args | Coefficient arguments (style-dependent) |

## 使用示例 / Usage Example

```lammps
# Lennard-Jones coefficients
pair_coeff * * 100.0 3.0
pair_coeff 1 1 100.0 3.5 9.0
pair_coeff 1 2 80.0 2.5

# Hybrid style
pair_coeff hybrid 1 1 lj/cut 1.0 1.0
pair_coeff hybrid 1 2 morse 100.0 2.0 3.0
```

## Wildcard Usage / 通配符使用

- `* *`: Apply to all atom type pairs
- `1 *`: Apply to type 1 with all other types
- `* 2`: Apply to type 2 with all other types

## Mixing Behavior / 混合行为

For I != J pairs:
- Default: **geometric mixing**
- Can be changed via `pair_modify mix` command
- Some styles support arithmetic, sixthpower, etc.

## 系数顺序 / Coefficient Order

Coefficients are style-dependent. Common patterns:

| Style | Coefficients |
|-------|--------------|
| lj/cut | epsilon sigma (cutoff) |
| lj/cut/coul/cut | epsilon sigma (cutoff1) (cutoff2) |
| morse | D0 alpha r0 (cutoff) |
| buckingham | A rho C (cutoff) |

## 相关命令 / Related Commands

- `pair_style`: Define pair interaction style
- `pair_modify`: Modify pair settings
- `read_data`: Read coefficients from data file

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/pair_coeff.md`
:::
