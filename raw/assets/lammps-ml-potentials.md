# LAMMPS Machine Learning Potentials - Comprehensive Reference

Source: https://docs.lammps.org/ (multiple pages), compiled 2026-06-12

## Overview

LAMMPS provides several packages for machine learning interatomic potentials (ML-IAPs). These allow using ML-trained potentials within LAMMPS simulations.

## ML Packages

| Package | Potential/Method | pair_style | Description |
|---------|-----------------|------------|-------------|
| ML-IAP | General interface | mliap | Python/PyTorch support for custom models |
| ML-PACE | Atomic Cluster Expansion (ACE) | pace | Multi-body basis function expansion |
| ML-SNAP | Spectral Neighbor Analysis | snap | Bispectrum component descriptors |
| ML-POD | Proper Orthogonal Descriptors | pod | POD-based potential |
| ML-HDNNP | High-Dimensional Neural Network | hdnnp | Behler-Parrinello type NN |
| ML-QUIP | QUIP interface | quip | GAP/SOAP and other QUIP potentials |
| ML-RANN | Rapid Atomistic Neural Network | rann | Rapid ANN potentials |

## pair_style mliap

General interface for machine-learning interatomic potentials. Supports:
- **linear** model: Linear regression on descriptors
- **mliappy** model: Python-based models (requires Python integration, supports PyTorch)
- ACE descriptors require building with ML-PACE package

Syntax:
```
pair_style mliap ...
pair_coeff * * <potential_file> <element1> <element2> ...
```

Related compute: `compute mliap` - computes ML-IAP descriptors

## pair_style pace

Atomic Cluster Expansion (ACE) - a general expansion of the atomic energy in multi-body basis functions.

Syntax:
```
pair_style pace
pair_coeff * * <potential_file> <element1> <element2> ...
```

Variants:
- `pace/extrapolation` - with extrapolation grade calculation
- `pace/apip` - active learning variant
- `pace/fast/apip` - optimized active learning
- `pace/precise/apip` - high precision active learning

## pair_style snap

Spectral Neighbor Analysis Potential (SNAP) - a machine-learning interatomic potential using bispectrum components.

Syntax:
```
pair_style snap
pair_coeff * * <snapcoeff_file> <snapparam_file> <element1> <element2> ...
```

## pair_style pod

Proper Orthogonal Descriptors potential - uses POD descriptors.

Syntax:
```
pair_style pod
pair_coeff * * <podparam_file> <coefficients_file> <element1> <element2> ...
```

Related command: `fitpod_command` - fits POD potential parameters

## pair_style hdnnp

High-Dimensional Neural Network Potential - Behler-Parrinello type.

Syntax:
```
pair_style hdnnp
pair_coeff * * <input_file> <scaling_file> <weights_files> ...
```

## pair_style quip

Interface to QUIP potentials including GAP/SOAP.

Syntax:
```
pair_style quip
pair_coeff * * <quip_params> <element1> <element2> ...
```

## External ML Frameworks Using ML-IAP

- **MACE**: Equivariant message-passing potentials, uses ML-IAP interface with cuEquivariance acceleration and multi-GPU inference
- **SevenNet**: Neural network potential, uses ML-IAP interface
- **FitSNAP**: Tool for fitting SNAP potentials (separate package)

## Key Build Requirements

- ML-IAP package requires ML-SNAP package
- ML-PACE required for ACE descriptors in ML-IAP
- mliappy model requires Python integration and PyTorch
- All ML packages are optional and must be explicitly enabled during build

## Usage Example (PACE)

```
pair_style pace
pair_coeff * * CuNi.yaml Cu Ni
```

## Usage Example (ML-IAP with PyTorch)

```
pair_style mliap model mliappy LAMMPS_mliap_model.pt descriptor sna ...
pair_coeff * * Cu Ni
```

## Sources

- https://docs.lammps.org/pair_mliap.html
- https://docs.lammps.org/pair_pace.html
- https://docs.lammps.org/pair_snap.html
- https://docs.lammps.org/pair_pod.html
- https://docs.lammps.org/pair_hdnnp.html
- https://docs.lammps.org/compute_mliap.html
- https://docs.lammps.org/Packages_details.html
- https://mace-docs.readthedocs.io/en/latest/guide/lammps_mliap.html
