# Fix Command / Fix 命令

## 实体概述 / Entity Overview

Applies ongoing operations during a simulation run (thermostats, barostats, constraints, etc.).

## 语法格式 / Syntax Format

```lammps
fix ID group-name style style-args
```

- `ID`: User-defined name for the fix
- `group-name`: Atom group to apply fix to
- `style`: Fix style name
- `style-args`: Style-specific arguments

## 主要分类 / Main Categories

### 1. Integration / 积分类

- `fix nve`: Constant NVE integration
- `fix nvt`: Constant NVT (thermostat)
- `fix npt`: Constant NPT (thermostat + barostat)
- `fix nph`: Constant NPH (barostat)

### 2. Thermostats / 恒温器

- `fix langevin`: Langevin thermostat
- `fix temp/berendsen`: Berendsen thermostat
- `fix temp/rescale`: Velocity rescaling

### 3. Barostats / 恒压器

- `fix press/berendsen`: Berendsen barostat
- `fix npt`: Nose-Hoover pressure control

### 4. Constraints / 约束

- `fix shake`: SHAKE bond constraints
- `fix rigid`: Rigid body dynamics
- `fix spring`: Harmonic spring restraint

### 5. Boundary Conditions / 边界条件

- `fix wall/reflect`: Reflective walls
- `fix wall/lj93`: LJ 9-3 wall potential

## 使用示例 / Usage Example

```lammps
# NVT ensemble
fix 1 all nvt temp 300.0 300.0 100.0

# Langevin thermostat
fix 2 fluid langevin 300.0 300.0 100.0 48279

# Rigid molecules
fix 3 molecules rigid/nve/small molecule

# Wall boundary
fix 4 all wall/reflect xlo EDGE xhi EDGE
```

## Fix Modification / Fix 修改

```lammps
fix_modify ID temperature new_temp
fix_modify ID pressure new_press
unfix ID
```

## 相关命令 / Related Commands

- `unfix`: Remove a fix
- `fix_modify`: Modify fix parameters
- `compute`: Calculate properties
- `thermo_style`: Output fix results

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/fix_*.md`
:::
