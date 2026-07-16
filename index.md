# LAMMPS LSP LLM Wiki / LAMMPS语言服务器LLM维基

**LAMMPS Large Scale Atomic/Molecular Massively Parallel Simulator - Language Server Protocol Knowledge Base**

This wiki contains comprehensive knowledge about LAMMPS molecular dynamics simulation and the lammps-lsp Language Server implementation.

## Wiki Structure / 维基结构

### [raw/](raw/) - Source Evidence / 原始证据
- [raw/assets/](raw/assets/) - Source documentation and code extracts
  - `README.md` - Project overview
  - `lammps_docs_md/` - 990 LAMMPS documentation files
  - `*.rs` - Source code extracts
  - `lammps-ml-potentials.md` - ML potentials reference
  - `lammps-python-api.md` - Python interface reference
  - `lammps-plugins.md` - Plugin mechanism reference
  - `lammps-rheo-package.md` - RHEO SPH package reference
  - `lammps-advanced-tutorials.md` - Advanced tutorials compilation
  - `lammps-new-features-2024-2025.md` - Recent features changelog
  - `lammps-upstream-sources.md` - Official upstream source manifest
  - `examples/in.melt` - Official Lennard-Jones melt tutorial input

### [wiki/entities/](wiki/entities/) - Entity Pages / 实体页面
LAMMPS-specific commands and concepts:
- [Pair_Style_LJ_Cut_Coul_Cut](wiki/entities/Pair_Style_LJ_Cut_Coul_Cut.md) - Lennard-Jones potential with Coulomb
- [Pair_Style_Hybrid](wiki/entities/Pair_Style_Hybrid.md) - Multiple pair styles
- [Pair_Coeff_Command](wiki/entities/Pair_Coeff_Command.md) - Pair coefficients
- [Fix_Command](wiki/entities/Fix_Command.md) - Ongoing operations
- [Compute_Command](wiki/entities/Compute_Command.md) - Property calculations
- [Units_Command](wiki/entities/Units_Command.md) - Unit systems
- [Atom_Style_Command](wiki/entities/Atom_Style_Command.md) - Atom attributes
- [Read_Data_Command](wiki/entities/Read_Data_Command.md) - Read structure files
- [Boundary_Command](wiki/entities/Boundary_Command.md) - Boundary conditions
- [Region_Command](wiki/entities/Region_Command.md) - Geometric regions
- [Thermo_Style_Command](wiki/entities/Thermo_Style_Command.md) - Output configuration
- [Run_Command](wiki/entities/Run_Command.md) - Execute simulation
- [Minimize_Command](wiki/entities/Minimize_Command.md) - Energy minimization
- [Neighbor_Command](wiki/entities/Neighbor_Command.md) - Neighbor list setup
- [Timestep_Command](wiki/entities/Timestep_Command.md) - Integration timestep
- [Dump_Command](wiki/entities/Dump_Command.md) - Output trajectories
- [Group_Command](wiki/entities/Group_Command.md) - Atom selection
- [Variable_Command](wiki/entities/Variable_Command.md) - Script variables
- [Kspace_Style_Command](wiki/entities/Kspace_Style_Command.md) - Long-range electrostatics
- [Velocity_Command](wiki/entities/Velocity_Command.md) - Initial velocities
- [Change_Box_Command](wiki/entities/Change_Box_Command.md) - Modify simulation box
- [Delete_Atoms_Command](wiki/entities/Delete_Atoms_Command.md) - Remove atoms
- [Create_Atoms_Command](wiki/entities/Create_Atoms_Command.md) - Add atoms
- [Lattice_Command](wiki/entities/Lattice_Command.md) - Crystal lattices
- [RHEO_Package](wiki/entities/RHEO_Package.md) - RHEO SPH fluid dynamics
- [ML_Potentials](wiki/entities/ML_Potentials.md) - Machine learning potentials
- [Plugin_Command](wiki/entities/Plugin_Command.md) - Plugin extension mechanism
- [Python_Interface](wiki/entities/Python_Interface.md) - Python API

### [wiki/concepts/](wiki/concepts/) - Concept Pages / 概念页面
Cross-cutting MD concepts:
- [Ensemble](wiki/concepts/Ensemble.md) - Statistical ensembles (NVE, NVT, NPT)
- [Periodic_Boundary_Conditions](wiki/concepts/Periodic_Boundary_Conditions.md) - PBC fundamentals
- [Neighbor_List](wiki/concepts/Neighbor_List.md) - Neighbor list algorithms
- [Integrator](wiki/concepts/Integrator.md) - Time integration methods
- [Force_Field](wiki/concepts/Force_Field.md) - Potential energy functions
- [Thermostat](wiki/concepts/Thermostat.md) - Temperature control
- [Barostat](wiki/concepts/Barostat.md) - Pressure control
- [Units_System](wiki/concepts/Units_System.md) - Unit systems and conversions
- [Machine_Learning_Potentials](wiki/concepts/Machine_Learning_Potentials.md) - ML-IAP concepts

### [wiki/synthesis/](wiki/synthesis/) - Synthesis Pages / 综合页面
Workflows and references:
- [Input_Script_Workflow](wiki/synthesis/Input_Script_Workflow.md) - Standard LAMMPS workflow
- [LAMMPS_Data_Format](wiki/synthesis/LAMMPS_Data_Format.md) - Data file specification
- [Output_Files](wiki/synthesis/Output_Files.md) - Output file types
- [Common_Patterns](wiki/synthesis/Common_Patterns.md) - Reusable patterns
- [Advanced_Features](wiki/synthesis/Advanced_Features.md) - Advanced workflows and methods
- [openqc-agent-context](wiki/synthesis/openqc-agent-context.md) - OpenQC agent LSP contract
- [diagnostic-engine-v1](wiki/concepts/diagnostic-engine-v1.md) - Diagnostic envelope and blocking policy

## Project Context / 项目背景

**LAMMPS Analyser** is a Language Server for LAMMPS input scripts that provides:
- Syntax checking and error detection
- Command validation
- Style-specific argument checking
- Hover documentation
- In-editor diagnostics

## Key Features / 主要特性

1. **Real-time validation** without running simulation
2. **Command syntax checking** via tree-sitter grammar
3. **Style-specific validation** for pair_style, fix_style, compute_style
4. **Comprehensive documentation** from LAMMPS manual
5. **Cross-reference checking** for variables and labels

## LSP Implementation / LSP 实现

- **Language**: Rust
- **Parser**: tree-sitter-lammps
- **Documentation**: 990 LAMMPS manual pages
- **Architecture**: Source in `src/`, docs in `lammps_docs_md/`

## Navigation / 导航

- Start with [Input_Script_Workflow](wiki/synthesis/Input_Script_Workflow.md) for workflow understanding
- See [entities](wiki/entities/) for specific command documentation
- Check [concepts](wiki/concepts/) for MD fundamentals
- Reference [synthesis](wiki/synthesis/) for integration patterns
- See [upstream source manifest](raw/assets/lammps-upstream-sources.md) for official doc links
- Run `bash scripts/check-llm-wiki.sh` to validate wiki navigation links

## Version / 版本

This wiki was generated for lammps-lsp project.
Last updated: 2025-06-12
Expanded: 2026-06-12 (added RHEO, ML potentials, plugins, Python API, advanced features)

## Source / 来源

:::info
**Project**: https://github.com/newtontech/lammps-lsp
**LAMMPS**: https://lammps.org/
:::
