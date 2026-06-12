# LAMMPS Plugin Mechanism - Comprehensive Reference

Source: https://docs.lammps.org/Developer_plugins.html, https://docs.lammps.org/plugin.html, compiled 2026-06-12

## Overview

LAMMPS plugins provide a mechanism to add functionality (new styles and commands) to a LAMMPS executable without recompiling LAMMPS. Plugins are dynamic shared objects (DSOs) that can be loaded at runtime.

## The plugin Command

```
plugin load plugin_file.so
plugin unload style name
plugin list
plugin clear
```

- **load**: Load a plugin DSO file
- **unload**: Unload a specific style from a plugin
- **list**: List all loaded plugins
- **clear**: Unload all plugins

## Writing Plugins

### Plugin Registration

Plugins must implement a registration function that LAMMPS calls when loading the DSO:

```cpp
#include "lammpsplugin.h"
#include "version.h"

static const char *style_name = "my/pair";

// Create function for the style
static Pair *my_pair_creator(LAMMPS *lmp) {
    return new PairMyPair(lmp);
}

// Plugin registration function
extern "C" void lammpsplugin_init(void *lmp, void *handle, void *regfunc) {
    lammpsplugin_t plugin;
    plugin.version = LAMMPS_VERSION;
    plugin.style = "pair";
    plugin.name = style_name;
    plugin.info = "My custom pair style";
    plugin.creator.v1 = (lammpsplugin_factory1 *) &my_pair_creator;
    (*register_plugin)(®func, &plugin);
}
```

### Supported Style Types

Plugins can register any of these style types:
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

### Building Plugins

Plugins are compiled as shared libraries:

```bash
g++ -shared -fPIC -I${LAMMPS_SRC_DIR}/src my_plugin.cpp -o my_plugin.so
```

## Plugin Architecture

- Plugins are loaded using dlopen() (Unix) or LoadLibrary() (Windows)
- Multiple styles can be registered from a single plugin DSO
- Plugins must be compatible with the LAMMPS version they are loaded into
- Plugin styles are treated identically to built-in styles after loading

## Use Cases

1. **Custom potentials**: Add new interatomic potentials without rebuilding LAMMPS
2. **External packages**: Distribute LAMMPS extensions as plugin packages
3. **Research codes**: Keep custom modifications separate from LAMMPS source
4. **Commercial extensions**: Distribute proprietary potentials as binary plugins

## Restrictions

- Plugins must be compiled against the same LAMMPS version
- ABI compatibility is required
- Not all LAMMPS internal classes are accessible from plugins
- Plugin loading order may matter if there are dependencies

## Developer Documentation

The `Developer_plugins.md` page in the raw docs provides detailed guidance on:
- Plugin file structure
- Registration callback mechanism
- Version compatibility
- Multi-style plugins
- Plugin build systems

## Sources

- https://docs.lammps.org/Developer_plugins.html
- https://docs.lammps.org/plugin.html
- https://docs.lammps.org/Developer.html
