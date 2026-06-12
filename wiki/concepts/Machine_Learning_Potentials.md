# Machine Learning Potentials in Molecular Dynamics

**Category**: MD Concept - Machine Learning Interatomic Potentials

## Overview / 概述

Machine learning interatomic potentials (ML-IAPs) use trained models to predict atomic forces and energies, combining the accuracy of quantum mechanical methods with the speed of classical force fields. LAMMPS supports multiple ML potential frameworks.

机器学习原子间势(ML-IAP)使用训练好的模型预测原子力和能量，结合了量子力学方法的精度和经典力场的速度。

## How ML Potentials Work / ML势的工作原理

1. **Descriptor Calculation**: Local atomic environment encoded as numerical descriptors (bispectrum, ACE, SOAP, etc.)
2. **Model Evaluation**: Trained model (neural network, linear regression, etc.) predicts per-atom energy
3. **Force Calculation**: Forces obtained as analytical derivatives of total energy
4. **Integration**: Standard MD integration (NVE, NVT, NPT, etc.)

## Descriptor Types / 描述符类型

| Descriptor | Used by | Characteristics |
|-----------|---------|-----------------|
| Bispectrum (SNAP) | ML-SNAP | Rotationally invariant, 4-body |
| ACE (Atomic Cluster Expansion) | ML-PACE | Systematically improvable, many-body |
| POD (Proper Orthogonal Descriptors) | ML-POD | Efficient low-dimensional |
| Symmetry functions | ML-HDNNP | Behler-Parrinello type |
| SOAP | ML-QUIP | Smooth overlap of atomic positions |

## Model Types / 模型类型

| Model | Framework | Description |
|-------|-----------|-------------|
| Linear | ML-IAP | Linear regression on descriptors |
| Neural Network | ML-HDNNP, ML-RANN | Multi-layer perceptron |
| Gaussian Approximation | ML-QUIP | Gaussian process regression |
| ACE expansion | ML-PACE | Linear combination of ACE basis |
| PyTorch model | ML-IAP (mliappy) | Any PyTorch architecture |

## Training Workflows / 训练工作流

### FitSNAP (for SNAP)
```bash
fitsnap_input.in
# Define training data, descriptor params, fitting method
```

### PACE (using pace_fit or external tools)
```
fitpod_command ...
# Fit POD parameters from training data
```

### MACE (external)
Train MACE model externally, then use in LAMMPS via ML-IAP interface.

## Combining ML Potentials with Other Methods / 与其他方法组合

### DFT-D3 Dispersion Correction (New 4Feb2025)
```
pair_style hybrid/overlay pace dispersion/d3 original pbe 30.0 20.0
pair_coeff * * pace potential.yaml Cu Ni
pair_coeff * * dispersion/d3 Cu Ni
```

### Hybrid with Classical Potentials
```
pair_style hybrid/overlay pace lj/cut 12.0
pair_coeff * * pace potential.yaml Cu Ni
pair_coeff 1 1 lj/cut 0.01 3.0
```

## Performance Considerations / 性能考虑

- ML potentials are typically 10-100x slower than classical force fields
- GPU acceleration available for some frameworks (MACE, SNAP)
- KOKKOS package can accelerate some ML potentials
- Consider using `run_style verlet` with multiple timesteps

## Accuracy / 精度

- ML potentials can approach DFT accuracy (meV/atom)
- Extrapolation beyond training data is risky
- PACE/extrapolation can monitor extrapolation grade
- Validate against known properties before production runs

## LAMMPS Packages / LAMMPS包

| Package | Required Dependencies |
|---------|----------------------|
| ML-IAP | ML-SNAP (required), ML-PACE (optional for ACE) |
| ML-PACE | None |
| ML-SNAP | None |
| ML-POD | None |
| ML-HDNNP | None |
| ML-QUIP | External QUIP library |
| ML-RANN | None |

## References / 参考文献

- Thompson et al., J. Comp. Phys., 285, 316 (2015) - SNAP
- Drautz, Phys. Rev. B, 99, 014104 (2019) - ACE
- Behler and Parrinello, Phys. Rev. Lett., 98, 146401 (2007) - HDNNP
- Batatia et al., arXiv:2401.0096 (2023) - MACE
