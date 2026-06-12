# Machine Learning Potentials in LAMMPS

**Category**: LAMMPS Package - Machine Learning Interatomic Potentials
**Source**: docs.lammps.org

## Overview / 概述

LAMMPS supports multiple machine learning interatomic potentials (ML-IAPs) through dedicated packages. These enable using neural network and other ML-trained potentials for molecular dynamics simulations.

LAMMPS通过专用包支持多种机器学习原子间势(ML-IAP)，可以使用神经网络和其他ML训练的势进行分子动力学模拟。

## Available ML Packages / 可用的ML包

| Package | pair_style | Method | Description |
|---------|-----------|--------|-------------|
| ML-IAP | `mliap` | General interface | Python/PyTorch support for custom models |
| ML-PACE | `pace` | ACE | Atomic Cluster Expansion |
| ML-SNAP | `snap` | SNAP | Spectral Neighbor Analysis Potential |
| ML-POD | `pod` | POD | Proper Orthogonal Descriptors |
| ML-HDNNP | `hdnnp` | HDNNP | High-Dimensional Neural Network (Behler-Parrinello) |
| ML-QUIP | `quip` | GAP/SOAP | QUIP interface |
| ML-RANN | `rann` | RANN | Rapid Atomistic Neural Network |

## pair_style mliap / 通用ML-IAP接口

General interface supporting:
- **linear** model: Linear regression on descriptors
- **mliappy** model: Python-based models (requires PyTorch)
- ACE descriptors available when built with ML-PACE

```
pair_style mliap model linear <model> descriptor sna <desc_params>
pair_coeff * * <elements>
```

Related compute: `compute mliap` (computes ML-IAP descriptors)

## pair_style pace / ACE势

Atomic Cluster Expansion using multi-body basis functions.

```
pair_style pace
pair_coeff * * <potential.yaml> <elements>
```

Variants: `pace/extrapolation`, `pace/apip`, `pace/fast/apip`, `pace/precise/apip`

## pair_style snap / SNAP势

Spectral Neighbor Analysis Potential using bispectrum components.

```
pair_style snap
pair_coeff * * <snapcoeff> <snapparam> <elements>
```

## pair_style pod / POD势

Proper Orthogonal Descriptors potential.

```
pair_style pod
pair_coeff * * <podparam> <coefficients> <elements>
```

Related: `fitpod_command` for fitting POD parameters.

## pair_style hdnnp / 高维神经网络势

High-Dimensional Neural Network Potential (Behler-Parrinello type).

```
pair_style hdnnp
pair_coeff * * <input> <scaling> <weights> ...
```

## pair_style quip / QUIP接口

Interface to QUIP potentials (GAP, SOAP, etc.).

```
pair_style quip
pair_coeff * * <quip_params> <elements>
```

## External Frameworks Using ML-IAP / 使用ML-IAP的外部框架

- **MACE**: Equivariant message-passing potentials, multi-GPU support
- **SevenNet**: Neural network potential via ML-IAP
- **FitSNAP**: Tool for fitting SNAP potentials

## Build Requirements / 构建要求

```bash
# Enable specific ML packages
cmake -DPKG_ML-IAP=on -DPKG_ML-PACE=on -DPKG_ML-SNAP=on ../cmake

# ML-IAP requires ML-SNAP
# ACE descriptors in ML-IAP require ML-PACE
# mliappy requires Python and PyTorch
```

## DFT-D3 Dispersion with ML Potentials / ML势的DFT-D3色散校正

New in 4Feb2025: `pair_style dispersion/d3` can be combined with ML potentials via hybrid/overlay.

```
pair_style hybrid/overlay pace dispersion/d3 original pbe 30.0 20.0
pair_coeff * * pace potential.yaml Cu Ni
pair_coeff * * dispersion/d3 C Ni
```

## Usage Example / 使用示例

```
# PACE potential example
pair_style pace
pair_coeff * * CuNi.yaml Cu Ni

# Or with D3 correction
pair_style hybrid/overlay pace dispersion/d3 original pbe 30.0 20.0
pair_coeff * * pace CuNi.yaml Cu Ni
pair_coeff * * dispersion/d3 Cu Ni
```
