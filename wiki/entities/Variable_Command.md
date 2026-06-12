# Variable Command / 变量命令

## 实体概述 / Entity Overview

Defines variables for use in input scripts, enabling dynamic and parameterized simulations.

## 语法格式 / Syntax Format

```lammps
variable name style args ...
```

## 变量样式 / Variable Styles

### 1. Index / 索引变量
```lammps
variable temp_index index 300.0 350.0 400.0
variable press_index index 1.0 10.0 100.0

# Use in loop
variable i loop 10
label loop_i
  # commands
  variable i loop 10
  next i
jump SELF loop_i
```

### 2. Loop / 循环变量
```lammps
variable i loop 100
label loop_start
  # operations
  next i
jump SELF loop_start
```

### 3. Equal / 相等变量
```lammps
variable temp equal temp
variable press equal press
variable vol equal vol

# Mathematical expressions
variable kBT equal 1.0*3.0/2.0*temp
variable density equal mass/vol
```

### 4. World / World变量
```lammps
# Used with -var command line flag
# lammps -var input_file data.in
variable name world value1 value2 value3
```

### 5. Universe / 宇宙变量
```lammps
variable t universe 300.0 400.0 500.0
# Divides work across multiple partitions
```

### 6. Atom / 原子变量
```lammps
# Per-atom property
variable kinetic atom ke
```

### 7. Format / 格式变量
```lammps
variable str format string "Temperature = %g"
variable filename format step_%d.dump
```

## 使用示例 / Usage Example

```lammps
# Define variables
variable T equal 300.0
variable P equal 1.0
variable dt equal 0.001

# Use in commands
fix 1 all nvt temp ${T} ${T} 100.0
fix 2 all npt temp ${T} ${T} 100.0 iso ${P} ${P} 1000.0
timestep ${dt}

# Conditional execution
variable energy equal pe
if "${energy} > -10000.0" then "print 'Energy not minimized'"

# File operations
variable dumpfile format "dump.${T}.${P}.dump"
dump 1 all atom 1000 ${dumpfile}
```

## 数学表达式 / Mathematical Expressions

```lammps
# Basic operations
variable sum equal v1 + v2
variable diff equal v1 - v2
variable prod equal v1 * v2
variable div equal v1 / v2

# Functions
variable sqrt equal sqrt(v1)
variable pow equal v1^2
variable exp equal exp(-v1)
variable log equal log(v1)

# Trigonometric
variable sin_val equal sin(v1)
variable cos_val equal cos(v1)
```

## 向量操作 / Vector Operations

```lammps
# Compute-based vectors
compute 1 all reduce sum c_force[1] c_force[2] c_force[3]
variable force_vec equal c_1
```

## 条件逻辑 / Conditional Logic

```lammps
variable step equal step
if "${step} < 10000" then &
  "fix 1 all nve" &
else "fix 1 all nvt temp 300.0 300.0 100.0"
```

## 循环模式 / Loop Patterns

### 1. Temperature Sweep
```lammps
variable T index 300.0 350.0 400.0 450.0 500.0
label loop_T
  fix 1 all nvt temp ${T} ${T} 100.0
  run 10000
  unfix 1
  next T
jump SELF loop_T
```

### 2. Pressure Sweep
```lammps
variable P index 1.0 10.0 100.0
label loop_P
  fix 1 all npt temp 300.0 300.0 100.0 iso ${P} ${P} 1000.0
  run 10000
  unfix 1
  next P
jump SELF loop_P
```

## 相关命令 / Related Commands

- `print`: Print variable values
- `if`: Conditional execution
- `label`: Define jump labels
- `jump`: Jump to labels

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/variable.md`
:::
