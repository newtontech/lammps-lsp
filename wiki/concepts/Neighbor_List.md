# Neighbor List / 近邻列表

## 概念概述 / Concept Overview

A data structure that efficiently tracks which atoms are within interaction range, enabling O(N) scaling instead of O(N²) for force calculations.

## 基本原理 / Basic Principle

```
For each atom i:
  Only consider atoms j where:
    distance(i,j) < cutoff + skin
  Update periodically (not every step)
```

## LAMMPS 实现 / LAMMPS Implementation

```lammps
# Basic setup
neighbor 0.3 bin
neigh_modify every 1 delay 0 check yes
```

## 参数说明 / Parameter Explanation

### Skin Distance
```
┌─────────────────────────────────┐
│                                 │
│  ●───────cutoff───────●         │
│         ◄───skin───►            │
│  ┊─────────────────────┊        │
│       neighbor sphere           │
└─────────────────────────────────┘
```

- **cutoff**: Force cutoff distance
- **skin**: Extra buffer distance
- **rebuild**: When atoms move > skin

### Update Frequency
- `every N`: Rebuild every N steps
- `delay N`: Delay rebuilding by N steps
- `check yes/no`: Check if rebuild needed

## 算法选择 / Algorithm Choice

### Bin-based (default)
```lammps
neighbor 0.3 bin
```
- O(N) scaling
- Spatial binning
- Efficient for most systems

### N-squared
```lammps
neighbor 0.3 nsq
```
- O(N²) scaling
- All-pairs check
- Only for very small systems

## 性能优化 / Performance Optimization

### Tuning Skin Distance

#### Too Small
```
→ Frequent rebuilding
→ Slow simulation
```

#### Too Large
```
→ Large neighbor lists
→ Slow force calculations
```

#### Optimal
```lammps
# Start with default
neighbor 0.3 bin

# Monitor rebuild frequency
run 1000
stat

# Adjust if needed
neighbor 0.5 bin  # If rebuilding too often
```

## 排除规则 / Exclusion Rules

### Molecular Exclusions
```lammps
# Exclude 1-2, 1-3, 1-4 interactions
neigh_modify exclude molecule/all

# Exclude within same molecule
neigh_modify exclude group/intra solute
```

### Group Exclusions
```lammps
# Exclude specific group pairs
neigh_modify exclude group solute solvent
```

## 特殊情况 / Special Cases

### Triclinic Boxes
```lammps
# Neighbor list handles tilt
neighbor 0.3 bin
# Automatically accounts for box tilt
```

### Restart
```lammps
# Neighbor list rebuilt after restart
read_restart file.restart
```

## 调试 / Debugging

### Check Rebuild Frequency
```lammps
run 10000
stat
# Look for "dangerous builds" message
```

### Optimal Settings
```lammps
# No dangerous builds = good
# Occasional dangerous builds = increase skin
# Frequent dangerous builds = significantly increase skin
```

## 相关概念 / Related Concepts

- **Cutoff radius**: Force calculation range
- **Skin buffer**: Neighbor list buffer
- **Verlet list**: Classic neighbor list algorithm
- **Cell lists**: Spatial partitioning

## 相关命令 / Related Commands

- `neighbor`: Set skin and algorithm
- `neigh_modify`: Modify neighbor list behavior
- `stat`: Print neighbor list statistics
- `timestep`: Affects atom movement → skin needs

## 参考资料 / References

:::info
**Source**: LAMMPS neighbor list documentation
:::
