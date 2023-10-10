# TODO

- [x] Parse base dimensions from units
  - [x] Tokenize
  - [x] Tree
  - [x] Expand - div inverts sign while walking tree
  - [x] Simplify - combine like terms, adding exponents of like terms
- [x] CONVERT UNITS
- [x] Unit convert dimension check
- [x] Error Handling
- [x] Simplify `&'static &'static` to `&'static`
- [x] Nice cli
- [x] [Split into new repo?](https://docs.github.com/en/get-started/using-git/splitting-a-subfolder-out-into-a-new-repository)
- [x] Support derived units Ex: J expands to ($kg*m^2*s^{−2}$)
- [ ] Scientific notation for inputs and outputs
- [x] Respect unit case?
- [x] Unit shortcuts
- [x] maybe like dont use dynamic dispatch
- [x] Fix `1 ohm => 0.000001 ohm*ohm/ohm`.
      Should return 1.
- [x] Add all [SI derived units](https://en.wikipedia.org/wiki/SI_derived_unit)
- [ ] Allow roots and non int powers
  - Take in a power in the from and to methods, or
  - Return a sequence of operations

## Time spent on this project

- 14:35 from misc repo
