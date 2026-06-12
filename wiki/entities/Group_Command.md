# Group Command / 组命令

## 实体概述 / Entity Overview

Defines groups of atoms for selective operations.

## 语法格式 / Syntax Format

```lammps
group ID style args
```

- `ID`: Group name
- `style`: Selection method
- `args`: Style-specific arguments

## 选择样式 / Selection Styles

### Type Selection
```lammps
# By atom type
group solute type 1
group solvent type 2 3 4
group ions type 5 6

# Type ranges
group heavy type 3:10
```

### Molecule Selection
```lammps
# By molecule ID
group water molecule 1:1000
group solute molecule 1001
```

### Region Selection
```lammps
region slab block -10 10 -10 10 0 5
group surface region slab
```

### Variable Selection
```lammps
group high_energy variable v_energy > 100.0
```

### Property Selection
```lammps
# By charge
group cations property/atom q > 0
group anions property/atom q < 0

# By mass
group heavy property/atom mass > 12.0
```

### Group Operations
```lammps
# Union
group combined union solute solvent

# Subtract
group mobile subtract all fixed

# Intersect
group overlap intersect group1 group2
```

## 使用示例 / Usage Example

```lammps
# Define groups
group solute type 1
group solvent type 2 3 4
group mobile subtract all fixed
group water molecule 1:1000

# Use with fixes
fix 1 solute nvt temp 300.0 300.0 100.0
fix 2 solvent langevin 300.0 300.0 100.0 48279

# Use with computes
compute 1 solute temp
compute 2 solvent temp

# Use with dumps
dump 1 solute custom 1000 solute.lammpstrj id type x y z

# Delete atoms
group delete_region region 1
delete_atoms group delete_region
```

## 组操作 / Group Operations

### Dynamic Groups
```lammps
# Group updates with atom properties
group dynamic dynamic all property/atom q > 0
```

### Empty Groups
```lammps
# Clear group
group solute type 9999  # No atoms match
```

## 相关命令 / Related Commands

- `delete_atoms`: Delete atoms in group
- `fix`: Apply fixes to groups
- `compute`: Compute properties for groups
- `dump`: Dump group atoms
- `undump`: Stop dumping group

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/group.md`
:::
