# Run Command / 运行命令

## 实体概述 / Entity Overview

Executes a dynamics simulation for a specified number of timesteps.

## 语法格式 / Syntax Format

```lammps
run N keyword values ...
```

- `N`: Number of timesteps to run

## 关键字 / Keywords

| Keyword | Description |
|---------|-------------|
| `upto` | Run up to specified timestep |
| `start` | Start at specified timestep |
| `stop` | Stop at specified timestep |
| `prepend` | Prepend operations |
| `every` | Execute operation every N steps |
| `until` | Loop until condition met |

## 使用示例 / Usage Example

```lammps
# Basic run
run 10000

# Run up to timestep 50000
run 10000 upto

# Run from 1000 to 5000
run 4000 start 1000 stop 5000

# With intermediate operation
run 100000 every 1000 "print 'Step = $t'"

# Until condition
run 100000 until "v_myvar > 100.0"
```

## 运行控制 / Run Control

### Prepending Operations
```lammps
run 10000 prepend "write_restart refined.restart"
```

### Periodic Operations
```lammps
run 1000000 every 10000 "write_data data.$t"
```

## 变量访问 / Variable Access

```lammps
variable steps equal 10000
run ${steps}

variable temp equal temp
run 10000 until "v_temp > 500.0"
```

## 中断条件 / Interruption Conditions

```lammps
# Stop when temperature exceeds threshold
run 100000 until "temp > 1000.0"

# Stop when variable condition met
run 100000 until "v_convergence < 1.0e-6"
```

## 能量最小化后运行 / Running After Minimization

```lammps
minimize 1.0e-4 1.0e-6 100 1000
run 10000
```

## 相关命令 / Related Commands

- `minimize`: Energy minimization
- `timestep`: Set timestep size
- `run_style`: Choose integration method

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/run.md`
:::
