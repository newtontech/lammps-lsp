# Periodic Boundary Conditions / 周期性边界条件

## 概念概述 / Concept Overview

Periodic boundary conditions (PBC) simulate bulk materials by replicating the simulation box infinitely in all periodic directions. This eliminates surface effects and approximates bulk matter.

## 基本原理 / Basic Principle

```
┌─────┬─────┬─────┐
│     │     │     │
│  ●  │  ●  │  ●  │  ← Atom at x=0.1
│     │     │     │        also at x=L+0.1
├─────┼─────┼─────┤
│     │     │     │
│  ●  │  ●  │  ●  │
│     │     │     │
└─────┴─────┴─────┘
```

## LAMMPS 实现 / LAMMPS Implementation

```lammps
# 3D periodic (default)
boundary p p p

# 2D periodic + fixed z
boundary p p f

# Non-periodic
boundary f f f
```

## 最小镜像约定 / Minimum Image Convention

For pairwise distances, use the shortest distance accounting for periodicity:

$$r_{ij} = \min(|\vec{r}_i - \vec{r}_j|, |\vec{r}_i - \vec{r}_j \pm L|)$$

### Example
```lammps
# Box: 0 to 20 in x
# Atom 1 at x=1
# Atom 2 at x=19
# Distance: min(18, 2) = 2 (through periodic boundary)
```

## 设置周期性 / Setting Periodicity

### All Periodic (3D)
```lammps
boundary p p p
# Standard bulk simulation
```

### Slab Geometry (2D)
```lammps
boundary p p f
# Surface with vacuum layer
# Fixed walls in z-direction
```

### Wire Geometry (1D)
```lammps
boundary p f f
# Nanowire simulation
```

### Cluster (0D)
```lammps
boundary f f f
# Isolated molecule/cluster
# No periodic images
```

## 周期性相关计算 / PBC-Related Calculations

### Pairwise Interactions
```lammps
pair_style lj/cut 10.0
# Automatically respects PBC
# Cutoff must be < box/2
```

### Long-Range Electrostatics
```lammps
pair_style lj/cut/coul/long 10.0
kspace_style pppm 1.0e-4
# Ewald summation handles PBC
```

## 盒子尺寸要求 / Box Size Requirements

### Minimum Image Convention
```
cutoff < box_length / 2
```

### Example
```lammps
# If cutoff = 10.0
# Box must be > 20.0 in each periodic dimension
```

## 非周期边界 / Non-Periodic Boundaries

### Fixed Walls
```lammps
boundary f f f
# Hard walls at box edges
# Particles bounce off walls
```

### Reflective Walls
```lammps
boundary p p p
fix wall1 all wall/reflect xlo EDGE
fix wall2 all wall/reflect xhi EDGE
```

### Soft Walls
```lammps
fix wall3 all wall/lj93 zlo EDGE 1.0 1.0 2.5
# LJ 9-3 potential wall
```

## 盒子类型 / Box Types

### Orthogonal Box
```lammps
# Default
xlo xhi
ylo yhi
zlo zhi
```

### Triclinic Box
```lammps
# Tilted box
xlo xhi xy
ylo yhi xz yz
zlo zhi
```

## 常见陷阱 / Common Pitfalls

1. **Cutoff too large**: Violates minimum image
2. **Wrong boundary setting**: Surface vs bulk
3. **Box deformation**: Extreme triclinic tilt
4. **Unwrapped coordinates**: Forgetting to wrap

## 相关命令 / Related Commands

- `boundary`: Set boundary conditions
- `change_box`: Modify box dimensions
- `fix wrap`: Wrap coordinates into box
- `fix deform`: Deform box (NPT)

## 参考资料 / References

:::info
**Source**: LAMMPS boundary conditions documentation
:::
