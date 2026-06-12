# Atom Style Command / 原子样式命令

## 实体概述 / Entity Overview

Defines what attributes are associated with atoms in the simulation.

## 语法格式 / Syntax Format

```lammps
atom_style style
```

## 支持的原子样式 / Supported Atom Styles

| Style | Description |
|-------|-------------|
| `angle` | Bonds + angles (no dihedrals/impropers) |
| `atomic` | Minimal: atom ID, type, position |
| `body` | Particles with body particles |
| `bond` | Bonds only |
| `charge` | + atomic charge |
| `colloid` | Large colloidal particles |
| `dipole` | + dipole moment |
| `dpd` | Dissipative Particle Dynamics |
| `electron` | Electronic force fields |
| `full` | Bonds + angles + dihedrals + impropers + charge |
| `granular` | Granular particles |
| `hybrid` | Custom combination of styles |
| `line` | Line segments |
| `mdpd` | Many-body DPD |
| `molecular` | Bonds + angles + dihedrals + impropers |
| `sphere` | Finite-size spheres |
| `template` | Molecule templates |
| `tri` | Triangular particles |
| `wavepacket` | Quantum wavepackets |

## 使用示例 / Usage Example

```lammps
# Simple atomic system
atom_style atomic

# Charged molecules (e.g., water)
atom_style full

# Metal atoms with charge
atom_style charge

# Granular particles
atom_style granular
```

## 属性详解 / Attribute Details

### atomic (minimal)
- atom ID
- atom type
- x, y, z coordinates

### charge
- All atomic attributes
- charge (q)

### molecular
- All atomic attributes
- molecule ID
- bond, angle, dihedral, improper topology

### full
- All molecular attributes
- atomic charge

## 选择建议 / Selection Guidelines

| Use Case | Recommended Style |
|----------|-------------------|
| Noble gases | `atomic` |
| Water with bonds | `full` |
| Ions in solution | `charge` |
| Polymers | `molecular` or `full` |
| Granular media | `granular` |
| Coarse-grained | `dpd` or `mdpd` |

## 限制条件 / Restrictions

- Style cannot be changed after atoms are created
- Some pair styles require specific atom styles
- Data file must match atom style format

## 相关命令 / Related Commands

- `read_data`: Read atomic coordinates
- `create_atoms`: Create new atoms
- `set`: Set atom properties

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/atom_style.md`
:::
