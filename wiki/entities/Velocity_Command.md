# Velocity Command / 速度命令

## 实体概述 / Entity Overview

Sets initial velocities for atoms, typically used to initialize temperature.

## 语法格式 / Syntax Format

```lammps
velocity group-ID style args keyword values ...
```

## 速度样式 / Velocity Styles

### 1. Create (Thermal Distribution)
```lammps
velocity all create 300.0 4928459
# group-ID create temp seed
```

- Creates Maxwell-Boltzmann distribution
- `temp`: Target temperature
- `seed`: Random number seed

### 2. Set (Uniform Value)
```lammps
velocity all set 1.0 0.0 0.0
# group-ID set vx vy vz
```

- Sets all atoms to same velocity
- Useful for flow initialization

### 3. Scale (Rescale)
```lammps
velocity all scale 0.9
# group-ID scale factor
```

- Scales existing velocities
- Used for temperature adjustment

### 4. Ramp (Linear Profile)
```lammps
velocity all ramp vx 0.0 1.0 y 0 10
# group-ID ramp v dim v1 v2 dim lo hi
```

- Creates linear velocity gradient
- Useful for shear flow

### 5. Zero (Reset to Zero)
```lammps
velocity all zero
# group-ID zero
```

- Zeros all velocities
- Used before minimization

## 使用示例 / Usage Example

### Initial Temperature
```lammps
# Create thermal distribution
velocity all create 300.0 4928459

# Verify temperature
compute 1 all temp
thermo_style custom step temp
```

### Flow Initialization
```lammps
# Poiseuille flow profile
velocity all ramp vx 0.0 1.0 y 0 10
```

### Temperature Rescaling
```lammps
# Current temperature is 500 K, want 300 K
velocity all scale 0.6  # 300/500 = 0.6
```

### Zero for Minimization
```lammps
velocity all zero
minimize 1.0e-4 1.0e-6 100 1000
```

## 关键字 / Keywords

### Units
```lammps
velocity all create 300.0 4928459 units box
# Box vs lattice units
```

### Bias (Remove COM Motion)
```lammps
velocity all create 300.0 4928459 bias yes
# Remove center-of-mass motion
```

### Loop (Local Bias)
```lammps
velocity all create 300.0 4928459 loop local
# Local thermal velocity
```

### Dist (Gaussian Profile)
```lammps
velocity gaussian create 300.0 4928459 dist gaussian
# Gaussian distribution
```

## 限制条件 / Restrictions

- Must be defined after atom creation
- Seed must be positive integer
- Some styles require specific setups

## 相关命令 / Related Commands

- `fix nve`: Starts dynamics
- `fix langevin`: Adds thermostat
- `compute temp`: Calculate temperature
- `minimize`: Energy minimization

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/velocity.md`
:::
