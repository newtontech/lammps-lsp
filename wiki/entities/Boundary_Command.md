# Boundary Conditions / 边界条件

## 实体概述 / Entity Overview

Controls simulation box boundary behavior for particles and fields.

## 语法格式 / Syntax Format

```lammps
boundary x y z
```

Each dimension (x, y, z) can be:
- `p` or `f`: Periodic (default)
- `f`: Fixed (non-periodic)
- `s` or `m`: Shrinking/Expanding (non-periodic)
- `o` or `m`: Open

## 边界类型 / Boundary Types

| Spec | Description | Example |
|------|-------------|---------|
| `p` | Periodic (wrapping) | `boundary p p p` |
| `f` | Fixed (hard wall) | `boundary f f f` |
| `s` | Shrinking (wall moves) | `boundary s s s` |
| `m` | Shrinking (alias for s) | `boundary m m m` |
| `o` | Open (no wall) | `boundary p p o` |

## 使用示例 / Usage Example

```lammps
# 3D periodic (default)
boundary p p p

# 2D slab with vacuum in z
boundary p p p

# Fixed walls in all directions
boundary f f f

# Slab with open top
boundary p p f
```

## 组合边界 / Boundary Combinations

### p p p (Periodic - 3D)
- Standard bulk simulation
- Images replicate in all directions

### p p f (Slab - 2D)
- Periodic in x and y
- Fixed wall in z
- Surface simulations

### f f f (Confined - 0D)
- All walls fixed
- Cluster/molecule in box

### p p o (Open surface)
- Periodic in x and y
- Open in z (no wall)
- Free surface

## 固定边界行为 / Fixed Boundary Behavior

With fixed walls:
- Particles bounce off walls
- No periodic images
- Wall can be reflective or absorbing (via fix wall)

## 周期性边界 / Periodic Boundaries

- Particles wrap around edges
- Minimum image convention applied
- Nearest neighbor searches respect PBC

## 陷阱 / Pitfalls

- `boundary s` requires shrink-wrapped fixes
- `boundary o` loses particles if they exit box
- Some pair styles require periodic boundaries

## 相关命令 / Related Commands

- `fix wall/reflect`: Reflective walls
- `fix wall/lj93`: LJ 9-3 wall potential
- `change_box`: Modify box dimensions

## 参考资料 / References

:::info
**Source**: `lammps_docsmd/boundary.md`
:::
