# Python Interface to LAMMPS

**Category**: LAMMPS Interface - Python
**Source**: docs.lammps.org/Python_*.html

## Overview / 概述

LAMMPS can be built as a shared library and driven through a Python interface. Three Python classes are provided: `lammps` (low-level), `PyLammps` (high-level), and `IPyLammps` (IPython-optimized).

LAMMPS可以构建为共享库并通过Python接口驱动。提供三个Python类：`lammps`（底层）、`PyLammps`（高层）和`IPyLammps`（IPython优化）。

## Installation / 安装

```bash
# From source: build shared library
cmake -DBUILD_SHARED_LIBS=on ../cmake
make

# Install Python module
cd python && pip install .

# Or unofficial PyPI
pip install lammps
```

## Three Interfaces / 三个接口

### lammps class (Core)
Low-level ctypes wrapper around LAMMPS C library API.

```python
from lammps import lammps
lmp = lammps()
lmp.command("units metal")
lmp.command("run 1000")
```

### PyLammps class (High-level)
Object-oriented wrapper with method-based API.

```python
from lammps import PyLammps
lmp = PyLammps()
lmp.units("metal")
lmp.run(1000)
print(lmp.system.natoms)
```

### IPyLammps class (IPython)
IPython/Jupyter-optimized with inline visualization.

## Key API Functions / 关键API函数

### Command Execution
- `lmp.command(cmd)` - Execute single command
- `lmp.commands_list(lst)` - Execute list of commands
- `lmp.file(path)` - Execute input script file

### Data Access
- `lmp.extract_global(name, type)` - Global properties
- `lmp.extract_atom(name, type)` - Per-atom data
- `lmp.extract_compute(id, style, type)` - Compute results
- `lmp.extract_fix(id, style, type)` - Fix results
- `lmp.extract_variable(name, ...)` - Variable values

### Atom Data
- `lmp.gather_atoms(name, count)` - Gather to numpy array
- `lmp.scatter_atoms(name, data)` - Scatter from numpy array

### System Properties
- `lmp.get_natoms()` - Number of atoms
- `lmp.get_thermo(key)` - Thermo data

## Complete Example / 完整示例

```python
from lammps import lammps
import numpy as np

lmp = lammps()
lmp.command("units metal")
lmp.command("dimension 3")
lmp.command("boundary p p p")
lmp.command("atom_style atomic")
lmp.command("lattice fcc 3.615")
lmp.command("region box block 0 4 0 4 0 4")
lmp.command("create_box 1 box")
lmp.command("create_atoms 1 box")
lmp.command("pair_style eam/alloy")
lmp.command("pair_coeff * * Cu_u3.eam.alloy Cu")
lmp.command("velocity all create 300.0 87287")
lmp.command("fix 1 all npt temp 300 300 0.1 iso 0 0 1")
lmp.command("thermo 100")
lmp.command("run 1000")

# Access results
natoms = lmp.get_natoms()
print(f"Atoms: {natoms}")
```

## Parallel Execution / 并行执行

Using pylammpsmpi for parallel execution from Python:
```python
from pylammpsmpi import PyLammps
lmp = PyLammps(cores=4)
```

## Error Handling / 错误处理

Build with `-DLAMMPS_EXCEPTIONS=on` for Python-compatible error handling.

## Build Requirements / 构建要求

```bash
cmake -DBUILD_SHARED_LIBS=on -DLAMMPS_EXCEPTIONS=on ../cmake
```

## Related / 相关

- Library.md (C library interface)
- Howto_library.md
- Howto_pylammps.md
- Python module pages (Python_module.md, Python_overview.md, etc.)
