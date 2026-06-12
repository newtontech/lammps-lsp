# LLM Wiki Change Log / LLM维基变更日志

## 2026-06-12 / 2026年6月12日

### Documentation Expansion / 文档扩展

**Created by**: Claude (GLM-5.1)
**Purpose**: Fill documentation gaps for RHEO, ML potentials, plugins, Python API, and advanced features

#### Raw Assets Added / 新增原始资源

**LAMMPS doc files (979 -> 990)** - 11 new pages:
- `fix_rheo.md` - RHEO core SPH integration fix
- `fix_rheo_pressure.md` - RHEO pressure equation of state
- `fix_rheo_viscosity.md` - RHEO viscosity model
- `fix_rheo_thermal.md` - RHEO thermal evolution
- `fix_rheo_oxidation.md` - RHEO surface oxidation bonding
- `pair_rheo.md` - RHEO pressure and viscous forces
- `pair_rheo_solid.md` - RHEO solid body contact forces
- `pair_dispersion_d3.md` - DFT-D3 dispersion correction (new 4Feb2025)
- `bond_rheo_shell.md` - RHEO elastic shell bonds
- `compute_rheo_property_atom.md` - RHEO atom property compute
- `Howto_rheo.md` - RHEO howto guide

**Compiled reference documents (6 new files)**:
- `lammps-ml-potentials.md` - ML potentials (PACE, SNAP, POD, ML-IAP, HDNNP, QUIP)
- `lammps-python-api.md` - Python interface (lammps, PyLammps, IPyLammps)
- `lammps-plugins.md` - Plugin mechanism (DSO loading, registration)
- `lammps-rheo-package.md` - Complete RHEO package reference
- `lammps-advanced-tutorials.md` - Advanced tutorials compilation
- `lammps-new-features-2024-2025.md` - Changelog of recent additions

#### Wiki Pages Added / 新增维基页面

**Entity Pages (+4, total 28)**:
- RHEO_Package - RHEO SPH fluid dynamics package
- ML_Potentials - Machine learning interatomic potentials
- Plugin_Command - Plugin extension mechanism
- Python_Interface - Python API to LAMMPS

**Concept Pages (+1, total 9)**:
- Machine_Learning_Potentials - ML-IAP concepts and descriptors

**Synthesis Pages (+1, total 5)**:
- Advanced_Features - Enhanced sampling, free energy, multiscale, ML workflows

#### Key Gaps Filled / 填补的关键空白

1. **RHEO Package**: Complete documentation for new SPH package (added 29Aug2024)
2. **ML Potentials**: Comprehensive reference for PACE, SNAP, POD, ML-IAP, HDNNP, QUIP
3. **Plugin System**: Plugin command and development guide
4. **Python API**: Full Python interface documentation
5. **New Features**: dispersion/d3, new fix/compute/pair styles from 2024-2025
6. **Advanced Methods**: Metadynamics, replica exchange, PIMD, NEB, hyperdynamics

#### Source Materials / 源材料

- docs.lammps.org (fetched live documentation pages)
- Web search for LAMMPS ML potentials ecosystem
- LAMMPS changelog (bug2024.html, bug2025.html)
- GitHub repos for MACE, SevenNet, FitSNAP integration docs

---

## 2025-06-12 / 2025年6月12日

### Initial Wiki Creation / 初始维基创建

**Created by**: Claude (Opus 4.8)
**Purpose**: Establish comprehensive LAMMPS knowledge base for lammps-lsp

#### Structure Created / 创建的结构

```
lammps-lsp/
├── raw/
│   └── assets/
│       ├── README.md
│       ├── lib_rs.txt
│       ├── input_script_rs.txt
│       ├── pair_styles_rs.txt
│       └── lammps_docs_md/ (979 files)
├── wiki/
│   ├── entities/ (22 files)
│   ├── concepts/ (8 files)
│   └── synthesis/ (4 files)
├── index.md
└── log.md (this file)
```

#### Content Summary / 内容摘要

**Entity Pages (22)**: Command and style documentation
- Pair styles: lj/cut/coul/cut, hybrid
- Commands: pair_coeff, fix, compute, units, atom_style, read_data, boundary, region, thermo_style, run, minimize, neighbor, timestep, dump, group, variable, kspace_style, velocity, change_box, delete_atoms, create_atoms, lattice

**Concept Pages (8)**: MD fundamentals
- Statistical ensembles, periodic boundary conditions, neighbor lists, time integration, force fields, thermostats, barostats, unit systems

**Synthesis Pages (4)**: Integration guides
- Input script workflow, data file format, output files, common patterns

#### Source Materials / 源材料

- Project README.md
- Source code: lib.rs, input_script.rs, pair_styles.rs
- 979 LAMMPS documentation files from lammps_docs_md/

#### Coverage / 覆盖范围

- **Core commands**: 22 entity pages covering essential LAMMPS commands
- **MD concepts**: 8 concept pages explaining fundamental principles
- **Workflows**: 4 synthesis pages with practical patterns
- **Bilingual format**: Chinese headings with English terminology

#### Future Enhancements / 未来增强

Potential additions:
- More pair_style variants (airebo, tersoff, EAM)
- Additional fix styles (npt variants, wall types)
- Compute styles (rdf, msd, stress/atom)
- Advanced patterns (parallel tempering, metadynamics)
- LSP-specific implementation details

---

## Template for Future Entries / 未来条目模板

```markdown
## YYYY-MM-DD

### Added / Added
- New page title

### Updated / Updated
- Page changes

### Fixed / Fixed
- Corrections made

### Notes / Notes
- Additional context
```
