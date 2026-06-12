# bond_style rheo/shell command

Source: https://docs.lammps.org/bond_rheo_shell.html

## Syntax

```
bond_style rheo/shell keyword value attribute1 attribute2 ...
```

- required keyword = t/form
- optional keyword = store/local

```
t/form value = formation time for a bond (time units)

store/local values = fix_ID N attributes ...
   fix_ID = ID of associated internal fix to store data
   N = prepare data for output every this many timesteps
   attributes = id1, id2, time, x, y, z, x/ref, y/ref, z/ref
```

## Examples

```
bond_style rheo/shell t/form 10.0
bond_coeff 1 1.0 0.05 0.1
```

## Description

Added in version 29Aug2024.

The rheo/shell bond style is designed to work with fix rheo/oxidation which creates candidate bonds between eligible surface or near-surface particles. When a bond is first created, it computes no forces and starts a timer. Forces are not computed until the timer reaches the specified bond formation time, t/form, and the bond is enabled.

Force: F = 2k(r - r_0) + (2k / (r_0^2 * epsilon_c^2)) * (r - r_0)^3

Damping: F_D = -gamma * w * (r_hat . v)

Coefficients:
- k (force/distance units)
- epsilon_c (unitless, maximum strain)
- gamma (force/velocity units)

Unlike other BPM-style bonds, this bond style does not update special bond settings when bonds are created or deleted.

This bond style is part of the RHEO package.

## Related commands

bond_coeff, fix rheo/oxidation

## References

(Clemmer) Clemmer, Pierce, O'Connor, Nevins, Jones, Lechman, Tencer, Appl. Math. Model., 130, 310-326 (2024).
