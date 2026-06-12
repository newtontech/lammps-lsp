# Timestep Command / 时间步长命令

## 实体概述 / Entity Overview

Sets the timestep size for molecular dynamics integration.

## 语法格式 / Syntax Format

```lammps
timestep dt
```

- `dt`: Timestep in time units

## 单位相关值 / Unit-Dependent Values

| Unit Style | Typical dt |
|------------|------------|
| `lj` | 0.005 - 0.01 tau |
| `real` | 1.0 - 2.0 fs |
| `metal` | 0.5 - 2.0 ps |
| `si` | 1.0e-15 - 1.0e-14 s |

## 使用示例 / Usage Example

```lammps
# Real units (biomolecules)
units real
timestep 1.0    # 1 femtosecond

# Metal units
units metal
timestep 0.001  # 1 picosecond

# LJ units
units lj
timestep 0.005  # Reduced time

# SHOCK/RATTLE for constraints
fix 1 all shake 0.0001 10 100 b 1 2 a 1
timestep 2.0    # Can use larger dt with constraints
```

## 选择指南 / Selection Guidelines

### For Bonded Systems
- **Without constraints**: 0.5 - 1.0 fs
- **With SHAKE**: 1.0 - 2.0 fs
- **With RATTLE**: 1.0 - 2.0 fs

### For Non-bonded Systems
- **LJ-only**: Can use larger timesteps
- **Charges**: Requires smaller timesteps

### For Temperature
- **Low T**: Can use larger dt
- **High T**: Need smaller dt

## 稳定性检查 / Stability Check

```lammps
# Monitor energy conservation
thermo_style custom step temp pe etotal

# If energy drifts:
# 1. Reduce timestep
# 2. Check fix settings
# 3. Verify pair_style
```

## 限制条件 / Restrictions

### Pair Style Limits
- Some pair styles have maximum stable dt
- Multi-body potentials may require smaller dt

### Fix Limits
- `fix nve`: Requires conservative timestep
- `fix shake`: Allows larger timestep

## 陷阱 / Pitfalls

1. **Too large**: Energy drift, simulation explosion
2. **Too small**: Wasted computation
3. **Unit mismatch**: Wrong time units
4. **System-specific**: No universal value

## 调优流程 / Tuning Workflow

```lammps
# 1. Start conservative
timestep 0.5

# 2. Run short test
run 1000

# 3. Check energy conservation
# 4. Increase if stable
timestep 1.0

# 5. Repeat
```

## 相关命令 / Related Commands

- `units`: Defines time units
- `fix shake`: Allows larger timestep
- `fix rattle`: Allows larger timestep
- `run`: Execute timesteps

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/timestep.md`
:::
