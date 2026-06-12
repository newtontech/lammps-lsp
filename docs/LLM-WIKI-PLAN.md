# LLM Wiki Plan for lammps-lsp / lammps-lsp的LLM维基计划

## Project Overview / 项目概述

**Target**: LAMMPS Language Server (lammps-lsp)
**Purpose**: Create comprehensive LLM-maintainable knowledge base for LAMMPS molecular dynamics simulation
**Format**: Karpathy-style LLM Wiki with bilingual format (Chinese headings, English terms)

## Wiki Structure / 维基结构

```
lammps-lsp/
├── raw/
│   └── assets/           # Source evidence files
│       ├── README.md
│       ├── lammps_docs_md/  # 979 LAMMPS documentation files
│       └── *.rs            # Source code extracts
├── wiki/
│   ├── entities/          # LAMMPS-specific entities
│   ├── concepts/          # Cross-cutting concepts
│   └── synthesis/        # Integration and workflows
├── index.md               # Navigation hub
└── log.md                 # Change log
```

## Content Plan / 内容计划

### Phase 1: Core Commands (Completed) / 核心命令（已完成）

#### Entity Pages (22 created)
1. **Pair_Style_LJ_Cut_Coul_Cut** - Lennard-Jones + Coulomb potential
2. **Pair_Style_Hybrid** - Multiple pair styles
3. **Pair_Coeff_Command** - Pair coefficients
4. **Fix_Command** - Ongoing operations overview
5. **Compute_Command** - Property calculations
6. **Units_Command** - Unit systems
7. **Atom_Style_Command** - Atom attributes
8. **Read_Data_Command** - Structure file input
9. **Boundary_Command** - Boundary conditions
10. **Region_Command** - Geometric regions
11. **Thermo_Style_Command** - Output configuration
12. **Run_Command** - Execute simulation
13. **Minimize_Command** - Energy minimization
14. **Neighbor_Command** - Neighbor list setup
15. **Timestep_Command** - Integration timestep
16. **Dump_Command** - Trajectory output
17. **Group_Command** - Atom selection
18. **Variable_Command** - Script variables
19. **Kspace_Style_Command** - Long-range electrostatics
20. **Velocity_Command** - Initial velocities
21. **Change_Box_Command** - Modify simulation box
22. **Delete_Atoms_Command** - Remove atoms
23. **Create_Atoms_Command** - Add atoms
24. **Lattice_Command** - Crystal lattices

### Phase 2: MD Concepts (Completed) / MD概念（已完成）

#### Concept Pages (8 created)
1. **Ensemble** - Statistical ensembles (NVE, NVT, NPT)
2. **Periodic_Boundary_Conditions** - PBC fundamentals
3. **Neighbor_List** - Neighbor list algorithms
4. **Integrator** - Time integration methods
5. **Force_Field** - Potential energy functions
6. **Thermostat** - Temperature control methods
7. **Barostat** - Pressure control methods
8. **Units_System** - Unit systems and conversions

### Phase 3: Synthesis (Completed) / 综合（已完成）

#### Synthesis Pages (4 created)
1. **Input_Script_Workflow** - Standard LAMMPS workflow
2. **LAMMPS_Data_Format** - Data file specification
3. **Output_Files** - Output file reference
4. **Common_Patterns** - Reusable script patterns

## Future Expansion Plans / 未来扩展计划

### Additional Entities (Potential)
- More pair_styles: airebo, tersoff, eam, sw, reax
- More fix styles: npt variants, wall types, rigid bodies
- More compute styles: rdf, msd, stress/atom, gyration
- Bond/Angle/Dihedral styles
- Advanced pair styles: table, hybrid/overlay, hybrid/scaled

### Additional Concepts (Potential)
- Minimum image convention
- Verlet integration
- Ewald summation
- PPPM method
- Constraint algorithms (SHAKE, RATTLE)
- Temperature calculation
- Pressure calculation

### Additional Synthesis (Potential)
- Advanced workflows (parallel tempering, metadynamics)
- Force field parameterization
- Analysis workflows
- Visualization workflows
- Performance optimization

## File Naming Convention / 文件命名约定

- Use PascalCase for entity pages: `Pair_Style_LJ_Cut.md`
- Use PascalCase for concept pages: `Periodic_Boundary_Conditions.md`
- Use PascalCase for synthesis pages: `Input_Script_Workflow.md`
- Use `.md` extension for all files

## Page Template / 页面模板

```markdown
# Title / 标题

## 实体概述 / Entity Overview
Brief description in English

## 语法格式 / Syntax Format
```lammps
command syntax
```

## 使用示例 / Usage Example
```lammps
example code
```

## 相关命令 / Related Commands
- command1
- command2

## 参考资料 / References
:::info
**Source**: source_location
:::
```

## Bilingual Format / 双语格式

- **Chinese headings**: 主要章节标题
- **English terms**: 专业术语保持英文
- **English content**: 描述和解释使用英文
- **Code blocks**: LAMMPS脚本代码

## Source Attribution / 来源归属

Each page includes:
:::info
**Source**: source_file_or_reference
:::

This provides traceability to original documentation.

## Maintenance Guidelines / 维护指南

1. **Update on changes**: When LAMMPS adds commands/features
2. **Add examples**: Practical use cases
3. **Cross-reference**: Link related pages
4. **Version tracking**: Note LAMMPS version compatibility
5. **Error corrections**: Fix inaccuracies promptly

## Success Metrics / 成功指标

- **Coverage**: Key LAMMPS commands documented
- **Accuracy**: Information matches LAMMPS manual
- **Usability**: Clear examples and explanations
- **Completeness**: Workflow from setup to analysis
- **Maintainability**: Easy to update and extend

---

**Document Version**: 1.0
**Created**: 2025-06-12
**Last Updated**: 2025-06-12
