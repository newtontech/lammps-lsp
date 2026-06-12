# Pair Style LJ/Cut/Coul/Cut / 配对样式 LJ/Cut/Coul/Cut

## 实体概述 / Entity Overview

Lennard-Jones potential with Coulombic interaction computed with direct cutoff.

## 语法格式 / Syntax Format

```lammps
pair_style lj/cut/coul/cut cutoff (cutoff2)
pair_coeff atom_type1 atom_type2 epsilon sigma (cutoff1) (cutoff2)
```

- `cutoff`: Global cutoff for LJ and Coulombic (if only 1 arg)
- `cutoff2`: Global cutoff for Coulombic (optional)
- `epsilon`: Energy coefficient (energy units)
- `sigma`: Distance coefficient (distance units)

## 数学公式 / Mathematical Formula

**Lennard-Jones 12/6 Potential:**

$$E = 4 \epsilon \left[ \left(\frac{\sigma}{r}\right)^{12} - \left(\frac{\sigma}{r}\right)^6 \right] \qquad r < r_c$$

**Coulombic Interaction:**

$$E = \frac{C q_i q_j}{\epsilon r} \qquad r < r_c$$

Where:
- $r_c$: Cutoff distance
- $C$: Energy-conversion constant
- $q_i, q_j$: Charges on atoms
- $\epsilon$: Dielectric constant

## 参数说明 / Parameter Description

| Parameter | Description | Units |
|-----------|-------------|-------|
| epsilon | LJ energy well depth | energy |
| sigma | LJ zero-crossing distance | distance |
| cutoff | Truncation distance | distance |

## 使用示例 / Usage Example

```lammps
pair_style lj/cut/coul/cut 10.0
pair_coeff * * 100.0 3.0
pair_coeff 1 1 100.0 3.5 9.0
```

## 变体 / Variants

- `lj/cut/coul/debye`: Debye screening
- `lj/cut/coul/dsf`: Damped shifted force
- `lj/cut/coul/long`: Long-range with Ewald/PPPM
- `lj/cut/coul/msm`: Multilevel summation
- `lj/cut/coul/wolf`: Wolf summation method

## 限制条件 / Restrictions

- Requires KSPACE package for lj/cut/coul/long
- Requires EXTRA-PAIR package for Debye, DSF, Wolf variants
- GPU/INTEL/KK/OMP/OPT suffix variants require respective packages

## 相关命令 / Related Commands

- `pair_coeff`: Set pair coefficients
- `pair_modify`: Modify pair settings
- `dielectric`: Set dielectric constant
- `kspace_style`: Long-range solver (for /long variant)

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/pair_lj_cut_coul.md`
:::
