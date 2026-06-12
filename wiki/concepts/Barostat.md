# Barostat / 恒压器

## 概念概述 / Concept Overview

Algorithms that maintain constant pressure in molecular dynamics simulations by adjusting simulation box dimensions.

## 压力控制原理 / Pressure Control Principle

**Instantaneous Pressure:**
$$P = \frac{2}{3 V} (KE) + \frac{1}{3 V} \sum_{i<j} \vec{r}_{ij} \cdot \vec{F}_{ij}$$

**Target:** Maintain $P \approx P_{target}$ by scaling box dimensions

## 主要恒压器类型 / Main Barostat Types

### 1. Nose-Hoover Barostat

#### Principle
Extended system method adding barostat variables for box dimensions.

#### Implementation
```lammps
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
# fix-ID group npt Tstart Tstop Tdamp Pstart Pstop Pdamp
```

#### Parameters
- `iso`: Isotropic pressure control
- `aniso`: Anisotropic control
- `x, y, z`: Individual dimension control
- `xy, xz, yz`: Tilt control

#### Isotropic Control
```lammps
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
# Scale all dimensions equally
```

#### Anisotropic Control
```lammps
fix 1 all npt temp 300.0 300.0 100.0 aniso 0.0 0.0 1000.0
# Scale dimensions independently
```

### 2. Berendsen Barostat

#### Principle
Box scaling with coupling constant.

#### Implementation
```lammps
fix 1 all press/berendsen iso 0.0 0.0 1000.0
# fix-ID group press/berendsen Pstart Pstop Pdamp
```

#### Characteristics
- Deterministic
- Does not sample isobaric ensemble exactly
- Good for equilibration
- Not for production runs

### 3. Parrinello-Rahman Barostat

#### Implementation
```lammps
fix 1 all npt temp 300.0 300.0 100.0 &
  aniso 0.0 0.0 1000.0 couple xyz
```

#### Characteristics
- Allows box shape changes
- Good for solids under stress

## 压力控制模式 / Pressure Control Modes

### Isotropic (All directions equal)
```lammps
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
```

### Semi-isotropic (2D equal, 1D different)
```lammps
fix 1 all npt temp 300.0 300.0 100.0 aniso 0.0 0.0 1000.0
fix_modify 1 couple xy xz yz
```

### Anisotropic (All directions independent)
```lammps
fix 1 all npt temp 300.0 300.0 100.0 &
  tri 0.0 0.0 0.0 0.0 0.0 1000.0
```

### Individual Dimensions
```lammps
# X only
fix 1 all npt temp 300.0 300.0 100.0 x 0.0 0.0 1000.0

# Y and Z only
fix 1 all npt temp 300.0 300.0 100.0 y 0.0 0.0 1000.0 z 0.0 0.0 1000.0
```

## 使用场景 / Use Cases

### 1. Liquid Simulation
```lammps
# Isotropic pressure control
fix 1 all npt temp 300.0 300.0 100.0 iso 1.0 1.0 1000.0
```

### 2. Membrane Simulation
```lammps
# Semi-isotropic (in-plane vs normal)
fix 1 all npt temp 300.0 300.0 100.0 aniso 0.0 0.0 1000.0
fix_modify 1 couple xy
```

### 3. Solid Under Stress
```lammps
# Anisotropic with fixed stress
fix 1 all npt temp 300.0 300.0 100.0 &
  aniso 0.0 0.0 1000.0 0.0 0.0 0.0
```

### 4. Volume Compression
```lammps
# Ramp pressure
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 1000.0 1000.0
# Compress from 0 to 1000 atm
```

## 参数调优 / Parameter Tuning

### Damping Parameter (Pdamp)

#### Guidelines
```lammps
# Start with default
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0

# If pressure fluctuates too much:
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 500.0  # Tighter

# If coupling too strong:
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 2000.0  # Looser
```

#### Rule of Thumb
$$\text{Pdamp} \approx 1000 \times \text{timestep}$$

## 组合配置 / Combined Configuration

### NPT (Temperature + Pressure)
```lammps
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
# Thermostat + barostat in one fix
```

### NPH (Pressure + Enthalpy)
```lammps
fix 1 all nph iso 0.0 0.0 1000.0
# Barostat only (no thermostat)
```

### Separate Thermostat + Barostat
```lammps
fix 1 all nvt temp 300.0 300.0 100.0
fix 2 all press/berendsen iso 0.0 0.0 1000.0
# Thermostat + separate barostat
```

## 常见问题 / Common Issues

### 1. Box Collapse

**Symptom:** Box dimensions shrink to zero

**Causes:**
- Pressure too high
- Attractive forces dominate
- No repulsive core

**Solutions:**
```lammps
# Add repulsive interactions
pair_style lj/cut 10.0

# Reduce pressure
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 100.0

# Constrain minimum box size
fix 2 all box/relax aniso 0.0
```

### 2. Box Explosion

**Symptom:** Box expands indefinitely

**Solutions:**
```lammps
# Increase pressure
fix 1 all npt temp 300.0 300.0 100.0 iso 100.0 100.0 1000.0

# Check force field
pair_style lj/cut/coul/long 10.0
```

### 3. Slow Convergence

**Symptom:** Pressure takes long time to equilibrate

**Solutions:**
```lammps
# Tighter coupling
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 500.0

# Or use Berendsen for equilibration
fix 1 all press/berendsen iso 0.0 0.0 1000.0
run 10000
unfix 1
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
```

## 监控压力 / Monitoring Pressure

```lammps
# Compute pressure components
compute 1 all pressure temp 1
compute pxx all reduce sum c_1[1]
compute pyy all reduce sum c_1[2]
compute pzz all reduce sum c_1[3]

# Output
thermo_style custom step temp press c_pxx c_pyy c_pzz
```

## 相关概念 / Related Concepts

- **Thermostat**: Temperature control
- **Ensemble**: NPT ensemble
- **Stress Tensor**: Pressure components

## 相关命令 / Related Commands

- `fix npt`: NPT ensemble (thermostat + barostat)
- `fix nph`: NPH ensemble
- `fix press/berendsen`: Berendsen barostat
- `fix deform`: Manual box deformation

## 参考资料 / References

:::info
**Source**: LAMMPS fix npt, fix nph documentation
:::
