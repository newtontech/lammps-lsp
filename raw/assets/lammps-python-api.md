# LAMMPS Python Interface - Comprehensive Reference

Source: https://docs.lammps.org/Python_module.html, https://docs.lammps.org/Python_overview.html, compiled 2026-06-12

## Overview

LAMMPS can be built as a shared library and driven through a Python interface. The Python module (`lammps.py`) wraps the LAMMPS C library API using Python's ctypes module.

## Three Python Interfaces

1. **`lammps`** - Core ctypes-based wrapper around LAMMPS C library API
2. **`PyLammps`** - Higher-level object-oriented wrapper (from lammps import PyLammps)
3. **`IPyLammps`** - IPython-optimized variant with visualization support

## Installation

### From PyPI (unofficial):
```bash
pip install lammps
```

### From source:
Build LAMMPS as shared library, then install Python module:
```bash
cd python
pip install .
```

Required files:
- `lammps.py` (Python wrapper on LAMMPS library)
- LAMMPS shared library file (`.so` / `.dll` / `.dylib`)

## Core lammps Class

```python
from lammps import lammps

# Create instance
lmp = lammps()           # default
lmp = lammps(cmdargs)    # with command line args
lmp = lammps('', ptr)    # from existing pointer

# Execute commands
lmp.command("units metal")
lmp.command("dimension 3")
lmp.command("boundary p p p")

# Or use file
lmp.file("in.lammps")
```

## PyLammps Class

```python
from lammps import PyLammps

lmp = PyLammps()
lmp.units("metal")
lmp.dimension(3)
lmp.boundary("p p p")

# Access system info
print(lmp.system.natoms)
print(lmp.system.xlo, lmp.system.xhi)

# Run simulation
lmp.run(10000)
```

## Key API Functions

### System Properties
- `lmp.get_natoms()` - number of atoms
- `lmp.get_thermo(...)` - thermo data
- `lmp.extract_variable(name, ...)` - variable values
- `lmp.extract_global(name, type)` - global properties
- `lmp.extract_atom(name, type)` - per-atom data
- `lmp.extract_compute(id, style, type)` - compute data
- `lmp.extract_fix(id, style, type)` - fix data

### Atom Manipulation
- `lmp.gather_atoms(name, count)` - gather atom data to numpy
- `lmp.scatter_atoms(name, data)` - scatter data to atoms
- `lmp.create_atoms(...)` - create atoms

### Execution
- `lmp.command(cmd)` - execute single command
- `lmp.commands_list(lst)` - execute list of commands
- `lmp.file(path)` - execute input file
- `lmp.close()` - close instance

## Python Module Structure

| Module Page | Content |
|-------------|---------|
| Python_module | Core lammps class details |
| Python_overview | Overview of Python interface |
| Python_install | Installation instructions |
| Python_head | Module imports and initialization |
| Python_create | Creating LAMMPS instances |
| Python_execute | Running commands |
| Python_atoms | Accessing atom data |
| Python_properties | System properties |
| Python_config | Configuration queries |
| Python_run | Running simulations |
| Python_neighbor | Neighbor list access |
| Python_scatter | Scatter/gather operations |
| Python_objects | Object-oriented API |
| Python_call | Calling LAMMPS from Python |
| Python_formats | Data format handling |
| Python_ext | Extending the Python interface |
| Python_error | Error handling |
| Python_examples | Usage examples |
| Python_trouble | Troubleshooting |

## Build Requirements

LAMMPS must be compiled as shared library:
```bash
cmake -DBUILD_SHARED_LIBS=on ../cmake
make
```

Optional: `-DLAMMPS_EXCEPTIONS=on` for better Python error handling.

## Parallel Python Interface

For parallel execution, use `pylammpsmpi` (from pyiron):
```python
from pylammpsmpi import PyLammps
lmp = PyLammps(cores=4)
```

## Sources

- https://docs.lammps.org/Python_module.html
- https://docs.lammps.org/python.html
- https://docs.lammps.org/Python_overview.html
- https://docs.lammps.org/Howto_pylammps.html
- https://docs.lammps.org/Library.html
