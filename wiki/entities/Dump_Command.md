# Dump Command / 输出命令

## 实体概述 / Entity Overview

Outputs simulation data in various formats during a run.

## 语法格式 / Syntax Format

```lammps
dump ID group-id style N file args
dump_modify ID keyword values ...
undump ID
```

## 主要样式 / Main Styles

| Style | Description | Extension |
|-------|-------------|------------|
| `atom` | Full atom coordinates | `.atom` |
| `atom/gz` | Compressed atom coordinates | `.atom.gz` |
| `cfg` | AtomEye CFG format | `.cfg` |
| `custom` | Custom attributes | `.lammpstrj` |
| `dcd` | DCD trajectory (CHARMM) | `.dcd` |
| `hdf5` | HDF5 format | `.h5` |
| `image` | Snapshot images | `.jpg` `.png` |
| `local` | Local atom properties | `.dump` |
| `xtc` | XTC (GROMACS) | `.xtc` |
| `xyz` | XYZ coordinates | `.xyz` |

## 使用示例 / Usage Example

```lammps
# Atom dump
dump 1 all atom 1000 dump.atom

# Custom dump
dump 2 all custom 1000 dump.lammpstrj id type x y z vx vy vz

# Compressed dump
dump 3 all atom/gz 1000 dump.atom.gz

# DCD trajectory
dump 4 all dcd 1000 trajectory.dcd

# XYZ format
dump 5 all xyz 1000 dump.xyz
```

## Custom 属性 / Custom Attributes

```lammps
# Basic
dump 1 all custom 1000 dump.lammpstrj id type x y z

# With velocities
dump 2 all custom 1000 dump.lammpstrj id type x y z vx vy vz

# With forces
dump 3 all custom 1000 dump.lammpstrj id type x y z fx fy fz

# With charges
dump 4 all custom 1000 dump.lammpstrj id type q x y z

# With compute values
compute 1 all property/atom xu yu zu
dump 5 all custom 1000 dump.lammpstrj id type c_1[1] c_1[2] c_1[3]
```

## Dump Modify / 修改输出

```lammps
# Append timestamp
dump_modify 1 append yes

# Compression (for uncompressed styles)
dump_modify 2 compress gzip

# Scale factor
dump_modify 3 scale 0.5

# Format specification
dump_modify 4 format float "%20.15g"

# Element names
dump_modify 5 element C H O N
```

## 输出频率 / Output Frequency

```lammps
# Every N steps
dump 1 all atom 1000 dump.atom

# With variable frequency
dump 2 all atom v_dump_freq dump.atom
variable dump_freq equal 1000
```

## 区域限制 / Region Limiting

```lammps
region 1 block -10 10 -10 10 0 10
dump 1 region1 atom 1000 region.atom
```

## 格式详解 / Format Details

### LAMMPS Trajectory (.lammpstrj)
```
ITEM: TIMESTEP
0
ITEM: NUMBER OF ATOMS
100
ITEM: BOX BOUNDS
xlo xhi
ylo yhi
zlo zhi
ITEM: ATOMS
1 1 0.0 0.0 0.0
2 2 1.0 0.0 0.0
...
```

### XYZ Format
```
100
Step = 0
C 0.0 0.0 0.0
H 1.0 0.0 0.0
...
```

## 相关命令 / Related Commands

- `undump`: Stop dump
- `read_dump`: Read dump files
- `dump_modify`: Modify dump settings
- `restart`: Restart files

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/dump.md`
:::
