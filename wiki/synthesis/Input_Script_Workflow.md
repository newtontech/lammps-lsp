# Input Script Workflow / 输入脚本工作流

## 综合概述 / Synthesis Overview

Typical LAMMPS input script structure and workflow for molecular dynamics simulations.

## 标准工作流 / Standard Workflow

### Phase 1: Initialization / 初始化

```lammps
# 1. Units and boundary
units real
boundary p p p
atom_style full

# 2. Simulation box and atoms
read_data system.data

# 3. Force field parameters
pair_style lj/cut/coul/long 10.0
bond_style harmonic
angle_style harmonic
dihedral_style harmonic
improper_style harmonic
kspace_style pppm 1.0e-4

# 4. Coefficients
pair_coeff * * 0.0 0.0  # Default
pair_coeff 1 1 0.1 3.0   # Type 1-1
bond_coeff * 500.0 1.0
angle_coeff * 100.0 109.47
```

### Phase 2: Minimization / 最小化

```lammps
# 1. Initial relaxation
minimize 1.0e-4 1.0e-6 100 1000

# 2. Reset timestep
reset_timestep 0
```

### Phase 3: Equilibration / 平衡

```lammps
# 1. NVE equilibration
velocity all create 300.0 4928459
fix 1 all nve
run 10000

# 2. NVT equilibration
unfix 1
fix 1 all nvt temp 300.0 300.0 100.0
run 10000

# 3. NPT equilibration (optional)
unfix 1
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
run 10000
```

### Phase 4: Production / 生产运行

```lammps
# 1. Production run (NVT or NPT)
unfix 1
fix 1 all nvt temp 300.0 300.0 100.0

# 2. Output
thermo_style custom step temp pe ke etotal press vol
thermo 1000

dump 1 all custom 1000 production.lammpstrj id type x y z

# 3. Run
run 100000
```

## 完整示例 / Complete Example

```lammps
# LAMMPS Input Script for Liquid Simulation

# --- Initialization ---
units real
boundary p p p
atom_style full

# --- Setup ---
read_data water.data

pair_style lj/cut/coul/long 10.0
bond_style harmonic
angle_style harmonic
kspace_style pppm 1.0e-4

pair_coeff * * 0.0 0.0
pair_coeff 1 1 0.1553 3.166
bond_coeff * 450.0 0.9572
angle_coeff * 55.0 104.52

# --- Analysis ---
compute 1 all temp
compute 2 all pe
compute 3 all ke
compute 4 all pressure temp 1

# --- Minimization ---
minimize 1.0e-4 1.0e-6 100 1000
reset_timestep 0

# --- Equilibration ---
velocity all create 300.0 4928459
fix 1 all nvt temp 300.0 300.0 100.0

thermo_style custom step temp c_1 c_2 c_3 c_4
thermo 1000

dump 1 all custom 1000 eq.lammpstrj id type x y z

run 10000
undump 1

# --- Production ---
dump 2 all custom 1000 prod.lammpstrj id type x y z vx vy vz

run 100000
```

## 命令顺序 / Command Order

### Required Order
1. **Settings**: `units`, `boundary`, `atom_style`
2. **Setup**: `read_data`, `lattice`, `region`, `create_box`
3. **Forces**: `pair_style`, `bond_style`, etc.
4. **Coefficients**: `pair_coeff`, `bond_coeff`, etc.
5. **Analysis**: `compute`, `variable`
6. **Dynamics**: `velocity`, `fix`, `timestep`
7. **Output**: `thermo_style`, `dump`, `restart`
8. **Run**: `run`, `minimize`

## 常见模式 / Common Patterns

### Temperature Ramp
```lammps
variable T equal 300.0 + step*0.1
fix 1 all nvt temp ${T} ${T} 100.0
run 1000
```

### Pressure Ramp
```lammps
variable P equal 0.0 + step*0.01
fix 1 all npt temp 300.0 300.0 100.0 iso ${P} ${P} 1000.0
```

### Loop Over Parameters
```lammps
variable T index 300.0 350.0 400.0
label loop_T
  fix 1 all nvt temp ${T} ${T} 100.0
  run 10000
  unfix 1
  next T
jump SELF loop_T
```

## 最佳实践 / Best Practices

1. **Comments**: Use `#` for documentation
2. **Units**: Always specify first
3. **Restart**: Save periodic restarts
4. **Output**: Monitor key quantities
5. **Validation**: Check energy conservation

## 调试技巧 / Debugging Tips

```lammps
# Print variables
print "Temperature = ${T}"

# Check forces
compute force all reduce sum fx fy fz
thermo_style custom step c_force

# Monitor convergence
thermo_style custom step temp pe etotal
```

## 相关概念 / Related Concepts

- **Minimization**: Energy minimization workflow
- **Equilibration**: System equilibration phases
- **Production**: Data collection phase

## 相关命令 / Related Commands

- `units`: Set units
- `read_data`: Read structure
- `fix`: Apply dynamics
- `run`: Execute simulation

## 参考资料 / References

:::info
**Source**: LAMMPS input script documentation
:::
