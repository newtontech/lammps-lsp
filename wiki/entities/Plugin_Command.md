# Plugin Command

**Category**: LAMMPS Command - Extension Mechanism
**Source**: https://docs.lammps.org/plugin.html

## Overview / 概述

The `plugin` command allows loading and unloading additional styles and commands into a LAMMPS binary from dynamic shared object (DSO) files. This enables extending LAMMPS without recompiling.

`plugin`命令允许从动态共享对象(DSO)文件加载和卸载额外的样式和命令，无需重新编译LAMMPS即可扩展功能。

## Syntax / 语法

```
plugin load plugin_file.so
plugin unload style name
plugin list
plugin clear
```

## Commands / 子命令

### plugin load
Load a plugin from a DSO file. The plugin registers new styles (pair, fix, compute, etc.) that become available for use in the input script.

### plugin unload
Unload a specific style from a previously loaded plugin.

- `style`: pair, bond, angle, dihedral, improper, fix, compute, kspace, region, dump, command
- `name`: the style name to unload

### plugin list
List all currently loaded plugins with their registered styles.

### plugin clear
Unload all plugins and remove all registered plugin styles.

## Supported Style Types / 支持的样式类型

Plugins can register any of these:
- `pair` - Pair styles
- `bond` - Bond styles
- `angle` - Angle styles
- `dihedral` - Dihedral styles
- `improper` - Improper styles
- `fix` - Fix styles
- `compute` - Compute styles
- `kspace` - KSpace styles
- `region` - Region styles
- `dump` - Dump styles
- `command` - New commands

## Usage Example / 使用示例

```
# Load a custom pair style plugin
plugin load my_pair_style.so

# Now use the custom style
pair_style my/custom/pair
pair_coeff * * params.txt

# List loaded plugins
plugin list

# Unload a specific style
plugin unload pair my/custom/pair

# Clear all plugins
plugin clear
```

## Plugin Development / 插件开发

Plugins must implement a registration function:

```cpp
extern "C" void lammpsplugin_init(void *lmp, void *handle, void *regfunc) {
    // Register styles with LAMMPS
}
```

Build as shared library:
```bash
g++ -shared -fPIC -I${LAMMPS_SRC}/src plugin.cpp -o plugin.so
```

## Restrictions / 限制

- Plugins must be ABI-compatible with the LAMMPS version
- Plugin styles are not available until after the plugin load command
- Not all internal LAMMPS classes are accessible from plugins

## Related / 相关

- Developer_plugins.md (in raw docs)
- Developer_write_pair.md
- Developer_write_fix.md
