# Change Box Command / 改变盒子命令

## 实体概述 / Entity Overview

Modifies the simulation box dimensions and boundary conditions.

## 语法格式 / Syntax Format

```lammps
change_box group-ID keyword values ...
```

## 主要关键字 / Main Keywords

### Boundary (Change Boundary Conditions)
```lammps
change_box all boundary p p f
# Change to periodic in x,y and fixed in z
```

### Ortho/Triclinic (Box Type)
```lammps
change_box all triclinic
# Convert to triclinic box

change_box all ortho
# Convert to orthogonal box
```

### Set (Set Dimensions)
```lammps
change_box all x final 0.0 20.0
change_box all y final 0.0 20.0
change_box all z final 0.0 20.0
```

### Scale (Scale Dimensions)
```lammps
change_box all x scale 1.1
# Scale x dimension by 10%

change_box all xy scale 0.05
# Add tilt to xy plane
```

### Move (Shift Origin)
```lammps
change_box all x shift -5.0
# Shift x origin by -5.0
```

## 使用示例 / Usage Example

### Create Vacuum Layer
```lammps
# Initial box: 0 to 20 in z
# Add vacuum from 20 to 40
change_box all z final 0.0 40.0 boundary p p f
```

### Change Periodicity
```lammps
# From 3D periodic to slab
change_box all boundary p p f
```

### Add Tilt
```lammps
# Create triclinic box
change_box all triclinic
change_box all xy final 0.0 5.0
change_box all xz final 0.0 2.0
```

### Expand Box
```lammps
# Double box size
change_box all x scale 2.0
change_box all y scale 2.0
change_box all z scale 2.0
```

## Triclinic Box / 三斜盒子

### Parameters
```
xlo xhi xy
ylo yhi xz yz
zlo zhi
```

### Example
```lammps
# Orthogonal to triclinic
change_box all triclinic

# Add tilt
change_box all xy tilt 0.1
change_box all xz tilt 0.05
change_box all yz tilt 0.02
```

## 限制条件 / Restrictions

- Must preserve atom positions within box
- Cannot create invalid geometry
- Some pair styles may not support triclinic boxes

## 相关命令 / Related Commands

- `boundary`: Set boundary conditions
- `fix deform`: Continuously deform box
- `read_data`: Read box dimensions

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/change_box.md`
:::
