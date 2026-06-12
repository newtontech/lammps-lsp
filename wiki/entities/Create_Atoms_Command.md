# Create Atoms Command / 创建原子命令

## 实体概述 / Entity Overview

Creates atoms in the simulation box, either on a lattice or at specified positions.

## 语法格式 / Syntax Format

```lammps
create_atoms type args keyword values ...
```

## 创建方式 / Creation Methods

### 1. Single Atom
```lammps
create_atoms 1 single 0.0 0.0 0.0
# type single x y z
```

### 2. Box (Lattice)
```lammps
lattice fcc 3.52
region box block 0 10 0 10 0 10
create_atoms 1 box
```

### 3. Region
```lammps
region sphere sphere 0.0 0.0 0.0 5.0
create_atoms 1 region sphere
```

### 4. Random
```lammps
create_atoms 1 random 100 12345 region box
# type random N seed region
```

## 关键字 / Keywords

### Units
```lammps
create_atoms 1 box units box
# Box vs lattice units
```

### Set (Set Properties)
```lammps
create_atoms 1 box set type 2
# Set atom type

create_atoms 1 box set charge 1.0
# Set charge

create_atoms 1 box set dipole 1.0 0.0 0.0
# Set dipole moment
```

### Vary (Random Variation)
```lammps
create_atoms 1 random 100 12345 region box vary 0.1
# Add random displacement
```

## 使用示例 / Usage Example

### Create Crystal Lattice
```lammps
lattice fcc 3.52
region box block 0 10 0 10 0 10
create_box 1 box
create_atoms 1 box
```

### Create Molecule
```lammps
molecule h2mol h2.data
create_atoms 0 single 0.0 0.0 0.0 mol h2mol 12345
```

### Add Ions to Solution
```lammps
region solution block -10 10 -10 10 0 10
create_atoms 2 random 50 12345 region solution
```

### Fill Defects
```lammps
region defect sphere 1.0 1.0 1.0 1.0
create_atoms 1 region defect
```

## 限制条件 / Restrictions

- Must create box first with `create_box`
- Atoms must be inside simulation box
- Cannot overlap with existing atoms (usually)

## 相关命令 / Related Commands

- `lattice`: Define lattice
- `region`: Define region
- `create_box`: Create simulation box
- `delete_atoms`: Remove atoms

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/create_atoms.md`
:::
