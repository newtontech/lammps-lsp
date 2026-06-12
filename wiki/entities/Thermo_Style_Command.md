# Thermo Style Command / 热力学输出样式命令

## 实体概述 / Entity Overview

Defines what quantities are printed to the thermodynamic output.

## 语法格式 / Syntax Format

```lammps
thermo_style style args ...
thermo_style custom keyword1 keyword2 ...
```

## 预定义样式 / Predefined Styles

| Style | Output Columns |
|-------|---------------|
| `one` | Step, temp, epair, emol, etotal |
| `multi` | Extended list of quantities |

## 自定义关键字 / Custom Keywords

| Keyword | Description |
|---------|-------------|
| `step` | Timestep |
| `atoms` | Number of atoms |
| `temp` | Temperature |
| `press` | Pressure |
| `pe` | Potential energy |
| `ke` | Kinetic energy |
| `etotal` | Total energy |
| `vol` | Volume |
| `lx,ly,lz` | Box dimensions |
| `px,py,pz` | Pressure components |
| `cpu` | CPU time per timestep |

## 使用示例 / Usage Example

```lammps
# Simple style
thermo_style one

# Custom style
thermo_style custom step temp pe ke etotal press vol

# With compute values
thermo_style custom step temp c_thermo_press c_diff[1]

# Multiple columns
thermo_style custom step atoms temp pe ke etotal press vol pxx pyy pzz
```

## 访问计算值 / Accessing Compute Values

```lammps
compute 1 all temp
compute 2 all pressure temp 1

thermo_style custom step temp c_1 c_2[1]
```

## 访问Fix值 / Accessing Fix Values

```lammps
fix 1 all ave/time 100 1 100 c_thermo_press
thermo_style custom step temp f_1
```

## 输出频率 / Output Frequency

```lammps
thermo 1000    # Print every 1000 steps
thermo 100     # Print every 100 steps
```

## 格式化 / Formatting

```lammps
thermo_modify format float %20.15g
thermo_modify format line "%d %g %g"
```

## 常见组合 / Common Combinations

### Energy Monitoring
```lammps
thermo_style custom step pe ke etotal
```

### Structural Monitoring
```lammps
thermo_style custom step vol lx ly lz density
```

### Pressure Components
```lammps_style custom step press pxx pyy pzz pxy pxz pyz
```

## 相关命令 / Related Commands

- `thermo`: Set output frequency
- `thermo_modify`: Modify output format
- `compute`: Define computed quantities
- `fix`: Define fix quantities

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/thermo_style.md`
:::
