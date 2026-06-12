# pair_style dispersion/d3 command

Source: https://docs.lammps.org/pair_dispersion_d3.html

## Syntax

```
pair_style dispersion/d3 damping functional cutoff cn_cutoff
```

- damping = damping function: original, zerom, bj, or bjm
- functional = XC functional form: pbe, pbe0, etc. (see list below)
- cutoff = global cutoff (distance units)
- cn_cutoff = coordination number cutoff (distance units)

## Examples

```
pair_style dispersion/d3 original pbe 30.0 20.0
pair_coeff * * C
```

## Description

Added in version 4Feb2025.

Style dispersion/d3 computes the dispersion energy-correction used in the DFT-D3 method of Grimme. It would typically be used with a machine learning (ML) potential that was trained with results from plain DFT calculations without the dispersion correction through pair_style hybrid/overlay.

The energy contribution E_i for an atom i is given by:

E_i = (1/2) * sum_{j != i} [ s_6 * C_{6,ij} / r^6_{ij} * f_6^{damp}(r_{ij}) + s_8 * C_{8,ij} / r^8_{ij} * f_8^{damp}(r_{ij}) ]

Available damping functions:
- **original**: zero-damping (Grimme1)
- **bj**: Becke-Johnson damping (Grimme2)
- **zerom**: revised zero-damping
- **bjm**: revised BJ damping

Available XC functionals include: pbe, pbe0, b3-lyp, revpbe, rpbe, tpss, hse06, b2-plyp, hf, lc-wpbe, m05, m06, m062x, cam-b3lyp, and many more (see full table in documentation).

All required coefficients are stored internally. The only information to provide are the chemical symbols of the atoms.

## Restrictions

Style dispersion/d3 is part of the EXTRA-PAIR package. The compiled-in parameters require the use of metal units.

It is currently not possible to calculate three-body dispersion contributions.

## Related commands

pair_coeff

## References

(Grimme1) S. Grimme, J. Antony, S. Ehrlich, and H. Krieg, J. Chem. Phys. 132, 154104 (2010).
(Grimme2) S. Grimme, S. Ehrlich and L. Goerigk, J. Comput. Chem. 32, 1456 (2011).
