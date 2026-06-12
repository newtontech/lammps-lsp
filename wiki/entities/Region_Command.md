# Region Command / 区域命令

## 实体概述 / Entity Overview

Defines a geometric region of space for various operations (atom selection, boundary conditions, etc.).

## 语法格式 / Syntax Format

```lammps
region ID style args keyword values ...
```

## 区域样式 / Region Styles

### Primitive Shapes / 基本形状

| Style | Description | Arguments |
|-------|-------------|-----------|
| `block` | Rectangular block | xlo xhi ylo yhi zlo zhi |
| `cone` | Conical frustum | dim c1 c2 radius1 radius2 lo hi |
| `cylinder` | Cylinder | dim c1 c2 radius lo hi |
| `plane` | Infinite plane | px py pz nx ny nz |
| `sphere` | Sphere | x y z radius |
| `union` | Boolean union | ... |
| `intersect` | Boolean intersection | ... |
| `subtract` | Boolean subtraction | ... |

### Dimensional / 维度参数

- `x`, `y`, `z`: Axis specification
- `c1 c2 c3`: Center coordinates
- `lo hi`: Bounds along axis

## 使用示例 / Usage Example

```lammps
# Block region
region 1 block -10 10 -10 10 0 20

# Sphere region
region sphere1 sphere 0 0 0 5.0

# Cylinder along z-axis
region cyl cylinder z 0 0 5.0 -5 5

# Union of two blocks
region both block 0 10 0 10 0 10
region combined union 2 s1 s2

# Subtract hole from block
region box block -20 20 -20 20 -20 20
region hole sphere 0 0 0 5
region final subtract box hole
```

## 关键字 / Keywords

| Keyword | Description |
|---------|-------------|
| `side` | Use region side (in/out) |
| `units` | Units (box or lattice) |
| `open` | Open region for particles |

## 常见用法 / Common Use Cases

### 1. Atom Selection
```lammps
group solute region 1
```

### 2. Boundary Conditions
```lammps
fix wall1 all wall/reflect region 1
```

### 3. Delete Atoms
```lammps
delete_atoms region 1
```

### 4. Create Atoms
```lammps
create_atoms 1 region 1
```

## 布尔运算 / Boolean Operations

```lammps
# Union: atoms in either region
region union_region union 2 region1 region2

# Intersection: atoms in both regions
region inter_region intersect 2 region1 region2

# Subtraction: atoms in region1 but not region2
region diff_region subtract 3 region1 region2 region2
```

## 限制条件 / Restrictions

- Region must fit within simulation box
- Complex boolean operations may be slow
- Some fixes require specific region types

## 相关命令 / Related Commands

- `group`: Select atoms by region
- `delete_atoms`: Remove atoms in region
- `fix wall/*`: Wall boundaries

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/region.md`
:::
