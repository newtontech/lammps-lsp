# LAMMPS Data File Format / LAMMPS数据文件格式

## 综合概述 / Synthesis Overview

The LAMMPS data file contains atomic structure, topology, and force field parameters in a structured text format.

## 文件结构 / File Structure

```
# LAMMPS data file

# Header section
atoms           N
bonds           Nbonds
angles          Nangles
dihedrals       Ndihedrals
impropers       Nimpropers

atom types      Ntypes
bond types      Nbond_types
angle types     Nangle_types
dihedral types  Ndihedral_types
improper types  Nimproper_types

xlo xhi
ylo yhi
zlo zhi

# Optional: triclinic tilt
xy xz yz

# Masses section
Masses

# Atoms section
Atoms

# Optional: velocities
Velocities

# Optional: topology sections
Bonds
Angles
Dihedrals
Impropers

# Optional: force field coefficients
Bond Coeffs
Angle Coeffs
Dihedral Coeffs
Improper Coeffs
```

## 头部详解 / Header Details

### Basic Counts
```
atoms           1000
bonds           500
angles          250
dihedrals       50
impropers       0
```

### Type Counts
```
atom types      5
bond types      3
angle types     2
dihedral types  1
improper types  0
```

### Box Dimensions
```
xlo xhi   -10.0  10.0
ylo yhi   -10.0  10.0
zlo zhi    -5.0   5.0
```

### Triclinic Box (Optional)
```
xlo xhi   -10.0  10.0
ylo yhi   -10.0  10.0
zlo zhi    -5.0   5.0
xy xz yz   0.0   0.0   0.0
```

## Masses Section / 质量部分

```
Masses

1 12.011
2 1.008
3 15.999
```

## Atoms Section / 原子部分

### Format depends on atom_style

#### atomic
```
Atoms

1 1 0.0 0.0 0.0
2 1 1.0 0.0 0.0
3 2 0.5 0.5 0.0
```
Columns: atom-ID, atom-type, x, y, z

#### charge
```
Atoms

1 1 -0.8 0.0 0.0 0.0
2 2 0.4 1.0 0.0 0.0
3 2 0.4 0.0 1.0 0.0
```
Columns: atom-ID, atom-type, charge, x, y, z

#### molecular
```
Atoms

1 1 1 0.0 0.0 0.0
2 1 2 1.0 0.0 0.0
3 2 2 0.0 1.0 0.0
```
Columns: atom-ID, molecule-ID, atom-type, x, y, z

#### full
```
Atoms

1 1 1 1 -0.8 0.0 0.0 0.0
2 1 2 2 0.4 1.0 0.0 0.0
3 1 3 2 0.4 0.0 1.0 0.0
```
Columns: atom-ID, molecule-ID, atom-type, charge, x, y, z

## Velocities Section / 速度部分

```
Velocities

1 0.0 0.0 0.0
2 0.1 0.0 0.0
3 -0.1 0.1 0.0
```
Columns: atom-ID, vx, vy, vz

## Topology Sections / 拓扑部分

### Bonds
```
Bonds

1 1 1 2
2 1 2 3
3 2 3 4
```
Columns: bond-ID, bond-type, atom1, atom2

### Angles
```
Angles

1 1 1 2 3
2 1 2 3 4
```
Columns: angle-ID, angle-type, atom1, atom2, atom3

### Dihedrals
```
Dihedrals

1 1 1 2 3 4
```
Columns: dihedral-ID, dihedral-type, atom1, atom2, atom3, atom4

### Impropers
```
Impropers

1 1 1 2 3 4
```
Columns: improper-ID, improper-type, atom1, atom2, atom3, atom4

## Coefficients Sections / 系数部分 (Optional)

### Bond Coefficients
```
Bond Coeffs

1 500.0 1.0
2 400.0 1.5
```
Columns: type, K, r0

### Angle Coefficients
```
Angle Coeffs

1 100.0 109.47
```
Columns: type, K, theta0

### Dihedral Coefficients
```
Dihedral Coeffs

1 1.0 1 0
```
Columns: type, K, n, d, phi0

## 完整示例 / Complete Example

```
# Water molecule system

atoms           3
bonds           2
angles          1

atom types      2
bond types      1
angle types     1

xlo xhi   -10.0  10.0
ylo yhi   -10.0  10.0
zlo zhi   -10.0  10.0

Masses

1 15.999
2 1.008

Atoms

1 1 1 1 -0.8 0.0 0.0 0.0
2 1 1 2 0.4 1.0 0.0 0.0
3 1 1 2 0.4 0.0 1.0 0.0

Bonds

1 1 1 2
2 1 1 3

Angles

1 1 2 1 3

Bond Coeffs

1 450.0 0.9572

Angle Coeffs

1 55.0 104.52
```

## 限制条件 / Restrictions

1. **Order matters**: Sections must appear in order
2. **Format matters**: Must match atom_style
3. **Counts must match**: Header counts must match actual data
4. **No extra spaces**: Consistent formatting

## 相关命令 / Related Commands

- `read_data`: Read data files
- `write_data`: Write data files
- `atom_style`: Define atom format

## 参考资料 / References

:::info
**Source**: LAMMPS read_data documentation
:::
