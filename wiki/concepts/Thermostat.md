# Thermostat / 恒温器

## 概念概述 / Concept Overview

Algorithms that maintain constant temperature in molecular dynamics simulations by rescaling velocities or adding stochastic forces.

## 温度控制原理 / Temperature Control Principle

**Instantaneous Temperature:**
$$T = \frac{2}{3 k_B N} \sum_i \frac{1}{2} m_i v_i^2$$

**Target:** Maintain $T \approx T_{target}$

## 主要恒温器类型 / Main Thermostat Types

### 1. Nose-Hoover Thermostat

#### Principle
Extended system method adding a thermal reservoir variable.

#### Implementation
```lammps
fix 1 all nvt temp 300.0 300.0 100.0
# fix-ID group nvt Tstart Tstop Tdamp
```

#### Parameters
- `Tstart`: Initial temperature
- `Tstop`: Final temperature
- `Tdamp`: Damping parameter (time units)

#### Characteristics
- Deterministic
- Time-reversible
- Samples canonical ensemble
- Good for most applications

#### Damping Parameter
```lammps
# Typical: 100 time units
# Smaller = tighter coupling
# Larger = looser coupling
fix 1 all nvt temp 300.0 300.0 50.0   # Tight
fix 1 all nvt temp 300.0 300.0 200.0  # Loose
```

### 2. Langevin Thermostat

#### Principle
Stochastic thermostat adding random forces and friction.

#### Implementation
```lammps
fix 1 all nve
fix 2 all langevin 300.0 300.0 100.0 48279
# fix-ID group langevin Tstart Tstop Tdamp seed
```

#### Parameters
- `Tstart, Tstop`: Temperature range
- `Tdamp`: Damping parameter
- `seed`: Random number seed

#### Characteristics
- Stochastic (non-deterministic)
- Adds friction + random force
- Good for large systems
- Samples canonical ensemble

#### Equations
$$F_i = F_{conservative} - \gamma v_i + \sqrt{2 \gamma k_B T} R_i(t)$$

Where:
- $\gamma$: Friction coefficient
- $R_i(t)$: Random force

### 3. Berendsen Thermostat

#### Principle
Velocity rescaling with coupling constant.

#### Implementation
```lammps
fix 1 all temp/berendsen 300.0 300.0 100.0
```

#### Characteristics
- Deterministic
- Does not sample canonical ensemble exactly
- Good for equilibration
- Not for production runs

### 4. Velocity Rescaling

#### Implementation
```lammps
fix 1 all temp/rescale 10 300.0 300.0 10.0 1.0
# fix-ID group temp/rescale N Tstart Tstop window
```

#### Characteristics
- Direct velocity rescaling
- Every N timesteps
- Simple but not rigorous

## 恒温器选择 / Thermostat Selection

| Property | Nose-Hoover | Langevin | Berendsen |
|----------|-------------|----------|-----------|
| Deterministic | Yes | No | Yes |
| Ensemble | Canonical | Canonical | Approximate |
| Large Systems | Yes | Better | Good |
| Equilibration | Good | Good | Best |
| Production | Best | Good | Avoid |

## 使用场景 / Use Cases

### Equilibration Phase
```lammps
# Berendsen for fast equilibration
fix 1 all temp/berendsen 300.0 300.0 100.0
run 10000

# Switch to Nose-Hoover for production
unfix 1
fix 1 all nvt temp 300.0 300.0 100.0
```

### Large Systems
```lammps
# Langevin for efficiency
fix 1 all nve
fix 2 all langevin 300.0 300.0 100.0 48279
```

### Temperature Ramping
```lammps
# Nose-Hoover with temperature change
fix 1 all nvt temp 300.0 500.0 100.0
run 100000  # Linearly interpolate
```

## 参数调优 / Parameter Tuning

### Damping Parameter (Tdamp)

#### Guidelines
```lammps
# Start with default
fix 1 all nvt temp 300.0 300.0 100.0

# If temperature fluctuates too much:
fix 1 all nvt temp 300.0 300.0 50.0  # Tighter

# If coupling too strong:
fix 1 all nvt temp 300.0 300.0 200.0  # Looser
```

#### Rule of Thumb
$$\text{Tdamp} \approx 100 \times \text{timestep}$$

## 常见问题 / Common Issues

### 1. Temperature Drift

**Symptom:** Temperature deviates from target

**Solutions:**
```lammps
# Increase coupling
fix 1 all nvt temp 300.0 300.0 50.0

# Check system
compute 1 all temp
thermo_style custom step temp c_1
```

### 2. Large Fluctuations

**Symptom:** Temperature oscillates wildly

**Solutions:**
```lammps
# Tighter coupling
fix 1 all nvt temp 300.0 300.0 50.0

# Or use Langevin
fix 1 all nve
fix 2 all langevin 300.0 300.0 100.0 48279
```

### 3. Energy Non-Conservation

**Symptom:** Energy drifts (NVE expected)

**Check:**
```lammps
# Should NOT have thermostat for energy conservation
fix 1 all nve  # Only this, no thermostat
```

## 组合使用 / Combined Usage

### With Barostat (NPT)
```lammps
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
# Thermostat + barostat combined
```

### With Constraints
```lammps
fix 1 all shake 0.0001 10 100 b 1 2
fix 2 all nvt temp 300.0 300.0 100.0
# SHAKE + thermostat
```

## 相关概念 / Related Concepts

- **Barostat**: Pressure control
- **Ensemble**: Thermodynamic ensemble
- **Temperature**: Instantaneous vs averaged

## 相关命令 / Related Commands

- `fix nvt`: Nose-Hoover thermostat
- `fix langevin`: Langevin thermostat
- `fix temp/berendsen`: Berendsen thermostat
- `fix temp/rescale`: Velocity rescaling

## 参考资料 / References

:::info
**Source**: LAMMPS fix nvt, fix langevin documentation
:::
