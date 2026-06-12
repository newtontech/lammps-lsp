# Neighbor Command / 近邻列表命令

## 实体概述 / Entity Overview

Sets parameters for neighbor list construction, which determines which atoms interact.

## 语法格式 / Syntax Format

```lammps
neighbor skin style
```

- `skin`: Extra distance beyond cutoff for neighbor list building
- `style`: `nsq` or `bin` (default)

## 参数说明 / Parameter Description

### Skin
- Extra distance beyond force cutoff
- Atoms within `cutoff + skin` are included
- Typical values: 0.2 to 0.5 of cutoff
- Too small: frequent rebuilding (slow)
- Too large: unnecessary computations (slow)

### Style
- `bin`: Bin-based algorithm (default, efficient)
- `nsq`: N-squared algorithm (all-pairs, slow)

## 使用示例 / Usage Example

```lammps
# Default: bin with 0.3 skin
neighbor 0.3 bin

# Larger skin for fast-moving systems
neighbor 0.5 bin

# Smaller skin for dense systems
neighbor 0.2 bin

# N-squared for very small systems
neighbor 0.3 nsq
```

## 相关设置 / Related Settings

### Neigh_modify
```lammps
neigh_modify every 1 delay 0 check yes
```

| Keyword | Description |
|---------|-------------|
| `every` | Rebuild every N steps |
| `delay` | Delay rebuilding by N steps |
| `check` | Check if rebuild needed |
| `page` | Pages for neighbor list |
| `one` | Build single list |
| `exclude` | Exclude atom pairs |

## 排除设置 / Exclusion Settings

```lammps
# Exclude molecule intra-molecular pairs
neigh_modify exclude molecule/all

# Exclude within same molecule
neigh_modify exclude group/intra one two

# Exclude specific group pairs
neigh_modify exclude group two three
```

## 性能优化 / Performance Optimization

### Guidelines
1. **Skin tuning**: Start with 0.3, adjust based on rebuild frequency
2. **Rebuild frequency**: Monitor with `stat` command
3. **Bin size**: Default is usually optimal

### Check rebuild frequency
```lammps
run 10000
stat
```

## 特殊情况 / Special Cases

### Triclinic boxes
```lammps
neighbor 0.3 bin
```

### Restart
```lammps
# Neighbor list rebuilt on restart
read_restart restart.file
```

## 常见问题 / Common Issues

| Symptom | Solution |
|---------|----------|
| Frequent rebuilds | Increase skin |
| Slow neighbor build | Check bin/nsq choice |
| Missed interactions | Decrease delay |

## 相关命令 / Related Commands

- `neigh_modify`: Modify neighbor list parameters
- `stat`: Print neighbor list statistics
- `timestep`: Larger timestep → larger skin

## 参考资料 / References

:::info
**Source**: `lammps_docs_md/neighbor.md`
:::
