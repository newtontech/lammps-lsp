# Minimize Command / 能量最小化命令

## 实体概述 / Entity Overview

Performs energy minimization to find local minima of the potential energy surface.

## 语法格式 / Syntax Format

```lammps
minimize etol ftol maxiter maxeval
```

## 参数说明 / Parameter Description

| Parameter | Description |
|-----------|-------------|
| `etol` | Energy convergence tolerance (energy units) |
| `ftol` | Force convergence tolerance (force units) |
| `maxiter` | Maximum iterations |
| `maxeval` | Maximum force evaluations |

## 使用示例 / Usage Example

```lammps
# Standard minimization
minimize 1.0e-4 1.0e-6 100 1000

# Tight convergence
minimize 1.0e-10 1.0e-10 1000 10000

# Loose convergence for initial relaxation
minimize 1.0e-3 1.0e-5 50 500

# With limits
minimize 0.0 0.0 10000 100000
```

## 最小化算法 / Minimization Algorithms

```lammps
min_style style
```

Available styles:
- `cg`: Conjugate gradient (default)
- `hftn`: Hybrid Fletcher-Reeves / Polak-Ribiere
- `sd`: Steepest descent
- `quickmin`: Quick-min
- `fire`: Fast inertial relaxation engine

### FIRE Algorithm
```lammps
min_style fire
minimize 1.0e-4 1.0e-6 100 1000
```

## 收敛标准 / Convergence Criteria

Minimization stops when either:
1. Maximum force component < `ftol`
2. Energy change between iterations < `etol`
3. Maximum iterations/evaluations reached

## 最小化限制 / Minimization Constraints

```lammps
# Fix atoms during minimization
fix 1 all setforce 0.0 0.0 0.0
group mobile type 2 3 4
fix 2 mobile setforce NULL NULL NULL

# Minimize only mobile atoms
minimize 1.0e-4 1.0e-6 100 1000
unfix 2
```

## 典型工作流 / Typical Workflow

```lammps
# 1. Initial structure
read_data system.data

# 2. Initial relaxation (loose)
minimize 1.0e-3 1.0e-5 50 500

# 3. Tighter minimization
min_style fire
minimize 1.0e-10 1.0e-10 1000 10000

# 4. Run dynamics
run 10000
```

## 相关命令 / Related Commands

- `min_style`: Choose minimization algorithm
- `min_modify`: Modify minimization parameters
- `undump`: Remove dump during minimization

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/minimize.md`
:::
