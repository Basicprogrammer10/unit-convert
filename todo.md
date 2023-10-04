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
- [ ] Support derived units Ex: J expands to ($kg*m^2*s^{−2}$)
  - [ ] Add pre-process step for unit expiation
  - [ ] Allow returning a string or pre-tokenized expression (enum)
    - [ ] Can we tokenize at compile time?
- [ ] Add lots of wacky units
- [ ] Scientific notation for inputs and outputs
