# Output Files Reference / 输出文件参考

## 综合概述 / Synthesis Overview

LAMMPS produces various output files during simulation for analysis, visualization, and restart purposes.

## 主要输出类型 / Main Output Types

### 1. Log File / 日志文件

#### Default: log.lammps
```
LAMMPS (...)
# Input script variables
# ...
# Memory usage
# ...
# Loop time of ...
```

#### Contents
- Thermodynamic output
- Warning/error messages
- Performance statistics
- Memory usage

### 2. Dump Files / 轨迹文件

#### Atom Format (.atom)
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
2 1 1.0 0.0 0.0
...
```

#### Custom Format (.lammpstrj)
```lammps
dump 1 all custom 1000 traj.lammpstrj id type x y z vx vy vz fx fy fz
```

#### XYZ Format (.xyz)
```
100
Step = 0
C 0.0 0.0 0.0
H 1.0 0.0 0.0
...
```

### 3. Restart Files / 重启文件

#### Binary Restart (.restart)
```lammps
restart 10000 restart.restart
restart 10000 restart.restart.restart10000
```

- Complete system state
- Can resume simulation
- Binary format

#### Restart2 (ASCII)
```lammps
write_restart restart.restart
```

### 4. Data Files / 数据文件

#### Write Data
```lammps
write_data current.data
```

- Atomic structure
- Topology
- Box dimensions

## 输出配置 / Output Configuration

### Thermodynamic Output
```lammps
thermo_style custom step temp pe ke etotal press vol
thermo 1000
```

#### Available Columns
- `step`: Timestep
- `atoms`: Number of atoms
- `temp`: Temperature
- `press`: Pressure
- `pe`: Potential energy
- `ke`: Kinetic energy
- `etotal`: Total energy
- `vol`: Volume
- `lx,ly,lz`: Box dimensions
- `pxx,pyy,pzz`: Pressure components

### Dump Output
```lammps
# Basic
dump 1 all atom 1000 dump.atom

# Custom
dump 2 all custom 1000 traj.lammpstrj id type x y z q

# Compressed
dump 3 all atom/gz 1000 dump.atom.gz

# Multiple
dump 4 all atom 1000 dump.atom
dump 5 all custom 1000 traj.lammpstrj id type x y z
```

### Dump Modify Options
```lammps
dump_modify 1 append yes
dump_modify 1 scale 0.5
dump_modify 1 format float "%20.15g"
dump_modify 1 sort id
```

## 可视化兼容性 / Visualization Compatibility

| Format | VMD | OVITO | PyMOL | MDAnalysis |
|--------|-----|-------|-------|------------|
| .atom | ✓ | ✓ | ✗ | ✓ |
| .lammpstrj | ✓ | ✓ | ✗ | ✓ |
| .xyz | ✓ | ✓ | ✓ | ✓ |
| .dcd | ✓ | ✓ | ✗ | ✓ |
| .cfg | ✓ | ✓ | ✗ | ✗ |
| .xtc | ✓ | ✓ | ✗ | ✓ |

## 输出优化 / Output Optimization

### Compression
```lammps
# Gzip compression
dump 1 all atom/gz 1000 dump.atom.gz

# Via dump_modify
dump 1 all atom 1000 dump.atom
dump_modify 1 compress gzip
```

### Variable Frequency
```lammps
variable dump_freq equal 1000
dump 1 all atom ${dump_freq} dump.atom
```

### Region-Based Output
```lammps
region 1 block -5 5 -5 5 0 10
dump 1 region1 atom 1000 region.dump
```

## 分析工作流 / Analysis Workflow

### Basic Analysis
```bash
# Extract trajectory
lammps -in input.lammps > log.txt

# Visualize
vmd -e load_traj.tcl

# Analyze
python analyze.py traj.lammpstrj
```

### Common Tools
- **VMD**: Visualization and analysis
- **OVITO**: Visualization and analysis
- **MDAnalysis**: Python analysis
- **pytraj**: Python trajectory analysis

## 文件大小管理 / File Size Management

### Strategies

#### Reduce Frequency
```lammps
dump 1 all atom 10000 dump.atom  # Less frequent
```

#### Output Subsets
```lammps
dump 1 solute custom 1000 solute.lammpstrj id type x y z
```

#### Compression
```lammps
dump 1 all atom/gz 1000 dump.atom.gz
```

#### Time Averaging
```lammps
fix 1 all ave/time 1000 1 1000 c_thermo_temp file temp.dat
```

## 相关命令 / Related Commands

- `dump`: Define trajectory output
- `dump_modify`: Modify dump settings
- `thermo_style`: Set thermodynamic output
- `write_restart`: Write restart file
- `write_data`: Write data file

## 参考资料 / References

:::info
**Source**: LAMMPS dump, thermo_style documentation
:::
