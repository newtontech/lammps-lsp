# Time Integration / 时间积分

## 概念概述 / Concept Overview

Numerical integration of Newton's equations of motion to evolve atomic positions and velocities over time.

## 牛顿方程 / Newton's Equations

$$\vec{F}_i = m_i \frac{d^2\vec{r}_i}{dt^2}$$

$$\frac{d\vec{r}_i}{dt} = \vec{v}_i$$

$$\frac{d\vec{v}_i}{dt} = \frac{\vec{F}_i}{m_i}$$

## 主要积分器 / Main Integrators

### 1. Velocity Verlet (Default)

#### Algorithm
```
1. v(t + 0.5*dt) = v(t) + 0.5 * a(t) * dt
2. r(t + dt) = r(t) + v(t + 0.5*dt) * dt
3. Calculate forces: a(t + dt)
4. v(t + dt) = v(t + 0.5*dt) + 0.5 * a(t + dt) * dt
```

#### Properties
- Time-reversible
- Symplectic (conserves phase space volume)
- Second-order accurate
- Energy-stable

```lammps
fix 1 all nve  # Uses Velocity Verlet
```

### 2. NVE Integration

```lammps
fix 1 all nve
```
- Constant energy
- No thermostat/barostat
- Pure Velocity Verlet

### 3. NVT Integration

#### Nose-Hoover
```lammps
fix 1 all nvt temp 300.0 300.0 100.0
```
- Extended system method
- Adds thermostat variable
- Samples canonical ensemble

#### Langevin
```lammps
fix 1 all nve
fix 2 all langevin 300.0 300.0 100.0 48279
```
- Stochastic dynamics
- Adds random force + friction
- Good for large systems

### 4. NPT Integration

```lammps
fix 1 all npt temp 300.0 300.0 100.0 iso 0.0 0.0 1000.0
```
- Nose-Hoover + barostat
- Variable box dimensions
- Samples isothermal-isobaric ensemble

## Run Style Options / 运行样式选项

```lammps
run_style verlet  # Default (Velocity Verlet)
run_style respa  # rRESPA (multiple timestep)
```

### rRESPA (Multiple Timestep)

```lammps
run_style respa 3 2 3 1 2 3
# Inner, middle, outer timesteps
# Bond, angle, dihedral
# Pair, kspace
```

## 积分器选择 / Integrator Selection

| Property | NVE | NVT (NH) | NVT (Langevin) | NPT |
|----------|-----|----------|-----------------|-----|
| Energy | Constant | Fluctuating | Fluctuating | Fluctuating |
| Temp | Fluctuating | Constant | Constant | Constant |
| Volume | Constant | Constant | Constant | Fluctuating |
| Accuracy | High | High | Medium | High |
| Use Case | Microcanonical | Canonical | Large systems | Isobaric |

## 时间步长选择 / Timestep Selection

### Guidelines

```lammps
# Bonded systems (no constraints)
timestep 0.5  # fs (real units)

# With SHAKE constraints
timestep 2.0  # Can use larger dt

# Non-bonded only
timestep 1.0  # Can use larger dt
```

### Stability Criterion

$$\Delta t < \frac{1}{10 \pi \nu_{max}}$$

Where $\nu_{max}$ is highest vibration frequency.

## 约束积分 / Constrained Integration

### SHAKE

```lammps
fix 1 all shake 0.0001 10 100 b 1 2 a 1
timestep 2.0  # Can use larger dt
```

### RATTLE

```lammps
fix 1 all rattle 0.0001 10 100 b 1 2 a 1
```

## 相关概念 / Related Concepts

- **Thermostat**: Temperature control
- **Barostat**: Pressure control
- **Ensemble**: Thermodynamic ensemble
- **Symplectic integrator**: Phase space conservation

## 相关命令 / Related Commands

- `fix nve`: NVE integration
- `fix nvt`: NVT integration
- `fix npt`: NPT integration
- `run_style`: Choose integration method
- `timestep`: Set timestep size

## 参考资料 / References

:::info
**Source**: LAMMPS integration documentation
:::
