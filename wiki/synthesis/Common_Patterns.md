# Common Patterns / 常见模式

## 综合概述 / Synthesis Overview

Frequently used LAMMPS input script patterns for common simulation scenarios.

## 1. Temperature Ramp / 温度 ramp

### Gradual Heating
```lammps
variable T equal 300.0 + step*0.1
fix 1 all nvt temp ${T} ${T} 100.0
run 1000  # Linear increase from 300 to 400 K
```

### Stepwise Temperature
```lammps
variable T index 300.0 350.0 400.0 450.0 500.0
label loop_T
  fix 1 all nvt temp ${T} ${T} 100.0
  run 10000
  unfix 1
  next T
jump SELF loop_T
```

### Annealing
```lammps
variable T equal 500.0 - step*0.05
fix 1 all nvt temp ${T} ${T} 100.0
run 4000  # Cool from 500 to 300 K
```

## 2. Pressure Ramp / 压力 ramp

### Compression
```lammps
variable P equal step*0.01
fix 1 all npt temp 300.0 300.0 100.0 iso ${P} ${P} 1000.0
run 10000  # Compress from 0 to 100 atm
```

### Decompression
```lammps
variable P equal 100.0 - step*0.01
fix 1 all npt temp 300.0 300.0 100.0 iso ${P} ${P} 1000.0
run 10000  # Decompress from 100 to 0 atm
```

## 3. Constraint Patterns / 约束模式

### Fix Atom Positions
```lammps
group fixed type 1
fix 1 fixed setforce 0.0 0.0 0.0
```

### Freeze Direction
```lammps
fix 1 all setforce NULL NULL 0.0  # Freeze z-motion
```

### Harmonic Restraint
```lammps
fix 1 all spring/self 10.0  # Restrain to initial positions
```

### Wall Constraints
```lammps
fix 1 all wall/reflect xlo EDGE
fix 2 all wall/reflect xhi EDGE
```

## 4. Selection Patterns / 选择模式

### By Type
```lammps
group solute type 1 2 3
group solvent type 4 5 6
```

### By Region
```lammps
region slab block -10 10 -10 10 0 5
group surface region slab
```

### By Property
```lammps
group mobile type 2 3 4
group fixed subtract all mobile
```

### Dynamic Update
```lammps
group high_temp dynamic all temp gt 400.0
```

## 5. Output Patterns / 输出模式

### Periodic Restart
```lammps
restart 10000 restart.restart
restart 10000 restart.restart.restart10000
```

### Conditional Output
```lammps
variable dump_freq equal 1000
dump 1 all custom ${dump_freq} traj.lammpstrj id type x y z
```

### Multiple Dumps
```lammps
dump 1 all atom 1000 dump.atom
dump 2 solute custom 1000 solute.lammpstrj id type x y z
dump 3 solvent custom 5000 solvent.lammpstrj id type x y z
```

## 6. Equilibration Patterns / 平衡模式

### Standard Protocol
```lammps
# 1. Minimize
minimize 1.0e-4 1.0e-6 100 1000

# 2. NVE
reset_timestep 0
velocity all create 300.0 4928459
fix 1 all nve
run 10000

# 3. NVT
unfix 1
fix 1 all nvt temp 300.0 300.0 100.0
run 10000

# 4. NPT (if needed)
unfix 1
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
run 10000
```

### Quick Equilibration
```lammps
# Use Berendsen for fast equilibration
fix 1 all nve
fix 2 all temp/berendsen 300.0 300.0 100.0
fix 3 all press/berendsen iso 0.0 0.0 1000.0
run 5000

# Switch to Nose-Hoover for production
unfix 2
unfix 3
fix 4 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
```

## 7. Analysis Patterns / 分析模式

### RDF Calculation
```lammps
compute 1 all rdf 100
fix 1 all ave/time 100 1 100 c_1[1] c_1[2] file rdf.dat mode vector
```

### MSD Calculation
```lammps
compute 1 all msd
thermo_style custom step temp c_1[4]  # MSD in column 4
```

### Energy Monitoring
```lammps
compute pe_atom all pe/atom
compute ke_atom all ke/atom
variable total_energy equal c_pe_atom + c_ke_atom
```

## 8. Simulation Box Patterns / 模拟盒模式

### Create Box
```lammps
lattice fcc 3.52
region box block 0 10 0 10 0 10
create_box 3 box
create_atoms 1 box
```

### Read from Data
```lammps
read_data system.data
change_box all boundary p p f  # Change boundaries
```

### Deform Box
```lammps
fix 1 all deform 1 x erate 0.001 units box
# Strain at 0.001 rate
```

## 9. Loop Patterns / 循环模式

### Simple Loop
```lammps
variable i loop 10
label loop_i
  print "Iteration = ${i}"
  variable i loop 10
  next i
jump SELF loop_i
```

### Nested Loop
```lammps
variable T index 300.0 400.0 500.0
variable P index 1.0 10.0 100.0
label loop_T
  label loop_P
    fix 1 all npt temp ${T} ${T} 100.0 iso ${P} ${P} 1000.0
    run 10000
    unfix 1
    next P
  jump SELF loop_P
  next T
jump SELF loop_T
```

## 10. Error Handling / 错误处理

### Conditional Execution
```lammps
variable energy equal pe
if "${energy} > -10000.0" then "print 'Warning: High energy'"
```

### Safe Minimization
```lammps
minimize 1.0e-4 1.0e-6 1000 10000
if "${etotal} > 0" then "print 'Error: Energy positive!'"
```

## 相关概念 / Related Concepts

- **Equilibration**: System equilibration
- **Production**: Data collection
- **Restart**: Simulation restart

## 相关命令 / Related Commands

- `variable`: Define variables
- `if`: Conditional execution
- `jump`: Jump to labels
- `label`: Define labels

## 参考资料 / References

:::info
**Source**: LAMMPS common patterns documentation
:::
