# Force Field / 力场

## 概念概述 / Concept Overview

A mathematical model describing the potential energy of a system of atoms as a function of atomic positions.

## 总能量公式 / Total Energy Formula

$$E_{total} = E_{bond} + E_{angle} + E_{dihedral} + E_{improper} + E_{pair} + E_{kspace}$$

## 项组成 / Energy Components

### 1. Bond Energy / 键能

#### Harmonic
$$E_{bond} = k_b (r - r_0)^2$$

```lammps
bond_style harmonic
bond_coeff 1 100.0 1.5
# k = 100.0, r0 = 1.5
```

#### FENE (Finite Extensible Nonlinear Elastic)
$$E_{bond} = -0.5 K R_0^2 \ln(1 - (r/R_0)^2) + k_{ramp} r^2$$

```lammps
bond_style fene
bond_coeff 1 30.0 1.5 1.0 1.0
```

### 2. Angle Energy / 角能

#### Harmonic
$$E_{angle} = k_\theta (\theta - \theta_0)^2$$

```lammps
angle_style harmonic
angle_coeff 1 75.0 109.47
```

#### Cosine Harmonic
$$E_{angle} = K [1 + \cos(n\theta - \theta_0)]$$

```lammps
angle_style cosine
angle_coeff 1 75.0 3 0.0
```

### 3. Dihedral Energy / 二面角能

#### Harmonic
$$E_{dihedral} = K [1 + d \cos(n\phi)]$$

```lammps
dihedral_style harmonic
dihedral_coeff 1 1.0 1 0  # K, n, d, phi0
```

#### OPLS
$$E_{dihedral} = \frac{1}{2} [K_1 (1 + \cos\phi) + K_2 (1 - \cos 2\phi) + K_3 (1 + \cos 3\phi) + K_4 (1 - \cos 4\phi)]$$

```lammps
dihedral_style opls
dihedral_coeff 1 1.0 0.5 0.25 0.0
```

### 4. Improper Energy / 非正常二面角能

```lammps
improper_style harmonic
improper_coeff 1 10.0 0.0
```

### 5. Pair Energy / 非键相互作用

#### Lennard-Jones
$$E_{LJ} = 4\epsilon [(\frac{\sigma}{r})^{12} - (\frac{\sigma}{r})^6]$$

```lammps
pair_style lj/cut 10.0
pair_coeff * * 0.010 2.5
```

#### Coulomb
$$E_{Coul} = \frac{q_i q_j}{4\pi\epsilon_0 r}$$

```lammps
pair_style lj/cut/coul/long 10.0
kspace_style pppm 1.0e-4
```

#### Buckingham
$$E = A \exp(-Br) - \frac{C}{r^6}$$

```lammps
pair_style buck 10.0
pair_coeff * * 1000.0 2.5 50.0
```

#### Morse
$$E = D_0 [1 - \exp(-\alpha(r - r_0))]^2$$

```lammps
pair_style morse 10.0
pair_coeff * * 0.1 2.0 3.0
```

### 6. Long-Range Electrostatics / 长程静电

#### Ewald Summation
```lammps
pair_style lj/cut/coul/long 10.0
kspace_style ewald 1.0e-6
```

#### PPPM
```lammps
pair_style lj/cut/coul/long 10.0
kspace_style pppm 1.0e-4
```

## 常用力场 / Common Force Fields

### CHARMM
```lammps
read_data charmm.data
pair_style lj/charmm/coul/long 10.0
bond_style harmonic
angle_style harmonic
dihedral_style charmm
```

### AMBER
```lammps
read_data amber.data
pair_style lj/cut/coul/long 10.0
bond_style harmonic
angle_style harmonic
dihedral_style harmonic
```

### OPLS
```lammps
read_data opls.data
pair_style lj/cut 10.0
bond_style harmonic
angle_style harmonic
dihedral_style opls
```

### ReaxFF (Reactive)
```lammps
pair_style reax/c control
```

### COMB (Charge Equilibration)
```lammps
pair_style comb
```

## 特殊力场 / Special Force Fields

### Coarse-Grained
```lammps
pair_style lj/cut 10.0
# Fewer interaction sites, larger timesteps
```

### Machine Learning
```lammps
pair_style deepmd
pair_style snap
pair_style pace
```

### Reactive
```lammps
pair_style reax/c
pair_style comb
pair_style aip
# Bond formation/breaking
```

## 参数选择 / Parameter Selection

### Rule of Mixtures
```lammps
pair_modify mix geometric  # Default
pair_modify mix arithmetic
pair_modify mix sixthpower
```

## 相关概念 / Related Concepts

- **Potential Energy Surface**: Energy as function of positions
- **Parameterization**: Fitting force field to data
- **Transferability**: Applicability across systems

## 相关命令 / Related Commands

- `pair_style`: Non-bonded interactions
- `bond_style`: Bonded interactions
- `angle_style`: Angle interactions
- `dihedral_style`: Dihedral interactions
- `kspace_style`: Long-range electrostatics

## 参考资料 / References

:::info
**Source**: LAMMPS pair_style, bond_style documentation
:::
