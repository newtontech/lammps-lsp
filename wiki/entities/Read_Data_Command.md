# Read Data Command / 读取数据命令

## 实体概述 / Entity Overview

Reads atomic structure and topology from a LAMMPS data file.

## 语法格式 / Syntax Format

```lammps
read_data file_name keyword args ...
```

## 常用关键字 / Common Keywords

| Keyword | Description |
|---------|-------------|
| `offset` | Offset atom IDs |
| `skip` | Skip lines in data file |
| `extra` | Allow extra atom types |
| `add` | Add atoms/attributes |
| `group` | Assign atoms to group |

## 使用示例 / Usage Example

```lammps
# Basic read
read_data system.data

# With atom type offset
read_data system.data offset 2

# With extra atom types (for hybrid styles)
read_data system.data extra/atom/types 5

# Assign group on read
read_data system.data group solute type 1 2
```

## 数据文件格式 / Data File Format

### Header Section
```
# LAMMPS data file

atoms           1000
bonds           500
angles           250
dihedrals        50
impropers         0

atom types        5
bond types        3
angle types        2
```

### Sections (order-dependent)
1. `xlo xhi` - Box dimensions
2. `Masses` - Atomic masses
3. `Atoms` - Atomic coordinates (format depends on atom_style)
4. `Velocities` - Initial velocities (optional)
5. `Bonds` - Bond topology
6. `Angles` - Angle topology
7. `Dihedrals` - Dihedral topology
8. `Impropores` - Improper topology

## Atoms Section Format

### atomic
```
# atom-ID atom-type x y z
1 1 0.0 0.0 0.0
```

### charge
```
# atom-ID atom-type q x y z
1 1 -0.8 0.0 0.0 0.0
```

### molecular
```
# atom-ID molecule-ID atom-type x y z
1 1 1 0.0 0.0 0.0
```

### full
```
# atom-ID molecule-ID atom-type q x y z
1 1 1 -0.8 0.0 0.0 0.0
```

## 限制条件 / Restrictions

- File must be in correct format
- Atom counts must match header
- Section order is strict
- Box must be orthogonal unless `triclinic` specified

## 相关命令 / Related Commands

- `write_data`: Write data file
- `create_atoms`: Create atoms programmatically
- `atom_style`: Define atom attributes

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/read_data.md`
:::
