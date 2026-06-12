# Lattice Command / 晶格命令

## 实体概述 / Entity Overview

Defines a lattice unit cell for use with `create_atoms` and other commands.

## 语法格式 / Syntax Format

```lammps
lattice style scale keyword values ...
```

## 晶格样式 / Lattice Styles

### 1. fcc (Face-Centered Cubic)
```lammps
lattice fcc 3.52
# style scale (a0 parameter)
```

### 2. bcc (Body-Centered Cubic)
```lammps
lattice bcc 2.86
```

### 3. sc (Simple Cubic)
```lammps
lattice sc 4.0
```

### 4. hex (Hexagonal)
```lammps
lattice hex 2.5
```

### 5. diamond (Diamond Cubic)
```lammps
lattice diamond 5.43
```

### 6. sq (Square 2D)
```lammps
lattice sq 1.0
```

### 7. custom (Custom Unit Cell)
```lammps
lattice custom 2.0 &
  a1 1.0 0.0 0.0 &
  a2 0.0 1.0 0.0 &
  a3 0.0 0.0 1.0 &
  basis 0.0 0.0 0.0 &
  basis 0.5 0.5 0.5
```

## 参数说明 / Parameter Description

| Parameter | Description |
|-----------|-------------|
| `style` | Lattice type |
| `scale` | Lattice constant (distance units) |
| `a1, a2, a3` | Lattice vectors (custom style) |
| `origin` | Origin offset |
| `spacing` | Spacing between lattice points |

## 使用示例 / Usage Example

### FCC Metal
```lammps
lattice fcc 3.52
region box block 0 10 0 10 0 10
create_box 1 box
create_atoms 1 box
```

### Custom Lattice
```lammps
lattice custom 2.0 &
  a1 1.0 0.0 0.0 &
  a2 0.0 1.0 0.0 &
  a3 0.0 0.0 1.0 &
  basis 0.0 0.0 0.0
```

### With Origin
```lammps
lattice fcc 3.52 origin 0.5 0.5 0.5
```

## Units / 单位

### Lattice Units
```lammps
lattice fcc 3.52
create_atoms 1 box units lattice
# Positions in lattice units
```

### Box Units
```lammps
lattice fcc 3.52
create_atoms 1 box units box
# Positions in box units
```

## 限制条件 / Restrictions

- Scale must be positive
- Custom lattices require valid vectors
- Basis atoms must be within unit cell

## 相关命令 / Related Commands

- `create_atoms`: Use lattice to create atoms
- `region`: Define regions
- `create_box`: Create simulation box

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/lattice.md`
:::
