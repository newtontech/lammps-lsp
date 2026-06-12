# Pair Style Hybrid / 配对样式 Hybrid

## 实体概述 / Entity Overview

Combines multiple pair styles in a single simulation, allowing different interactions for different atom type pairs.

## 语法格式 / Syntax Format

```lammps
pair_style hybrid style1 args style2 args ...
pair_coeff hybrid atom_type1 atom_type2 style_name coeff_args
```

## 使用场景 / Use Cases

1. **Mixed systems** with different interaction types
2. **Coarse-grained + atomistic** models
3. **Specialized interactions** for specific atom pairs
4. **Testing different potentials** in same simulation

## 使用示例 / Usage Example

```lammps
pair_style hybrid lj/cut 10.0 morse 8.0
pair_coeff hybrid 1 1 lj/cut 1.0 1.0
pair_coeff hybrid 1 2 morse 100.0 2.0 3.0
pair_coeff hybrid 2 2 morse 50.0 1.5 2.5
```

## 参数说明 / Parameter Description

| Parameter | Description |
|-----------|-------------|
| style1, style2, ... | Pair styles to combine |
| args | Arguments for each style |
| style_name | Which style to use for specific pair |
| coeff_args | Coefficients for chosen style |

## 限制条件 / Restrictions

- Each pair style must be separately documented
- Cannot use hybrid within hybrid (nested)
- All styles must support similar features
- Coefficient specification becomes complex

## 相关命令 / Related Commands

- `pair_coeff`: Set hybrid pair coefficients
- `pair_style single`: Non-hybrid alternative

## LSP Implementation / LSP 实现

```rust
// From src/styles/pair_styles.rs
PS::Hybrid => Some(PairStyleInfo {
    min_args: 1,
    max_args: None,  // Can have any number of args
    min_coeffs: 1,
    max_coeffs: None,  // Can have any number of coeffs
    arg_context: "style1 args style2 args...",
    coeff_context: "style args",
}),
```

:::info
**Source**: `src/styles/pair_styles.rs`, `lammps_docs_md/pair_hybrid.md`
:::
