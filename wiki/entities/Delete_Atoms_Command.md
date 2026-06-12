# Delete Atoms Command / 删除原子命令

## 实体概述 / Entity Overview

Deletes atoms from the simulation based on various criteria.

## 语法格式 / Syntax Format

```lammps
delete_atoms group-ID keyword values ...
```

## 删除样式 / Deletion Styles

### By Group
```lammps
group solute type 1 2 3
delete_atoms group solute
```

### By Region
```lammps
region slab block -10 10 -10 10 0 5
delete_atoms region slab
```

### By Variable
```lammps
variable high_ke atom ke > 10.0
delete_atoms variable high_ke
```

## 关键字 / Keywords

### Bond (Delete Bonded Atoms)
```lammps
delete_atoms group solute bond yes
# Also delete atoms bonded to deleted atoms
```

### Compress (Reset IDs)
```lammps
delete_atoms group solute compress yes
# Renumber atoms after deletion
```

### Overlay (Keep Topology)
```lammps
delete_atoms region 1 bond yes overlay yes
# Keep bonds/angles/dihedrals
```

## 使用示例 / Usage Example

### Delete Solvent
```lammps
group solvent type 4 5 6
delete_atoms group solvent bond yes
```

### Create Vacuum
```lammps
region vacuum block -10 10 -10 10 5 10
delete_atoms region vacuum
```

### Delete High Energy Atoms
```lammps
compute per_atom all pe/atom
variable high_e atom "c_per_atom > -10.0"
delete_atoms variable high_e
```

### Delete Outside Region
```lammps
region box block 0 10 0 10 0 10
delete_atoms region box compress yes
```

## 限制条件 / Restrictions

- Cannot delete atoms involved in fixes
- May leave orphaned topology
- Bonds to deleted atoms become invalid

## 相关命令 / Related Commands

- `create_atoms`: Create atoms
- `group`: Define atom groups
- `region`: Define spatial regions

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/delete_atoms.md`
:::
