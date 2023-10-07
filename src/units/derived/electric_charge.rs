use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, electric_current, time},
};

const _COULOMB: [Unit; 2] = [
    Unit::new(&electric_current::AMPERE, 1.0, 0.0),
    Unit::new(&time::SECOND, 1.0, 0.0),
];

impl_derived_units! {
    COULOMB => [
        <| _COULOMB,
        aliases = ["C"],
        metric = true
    ],
    /// `1.602176634×10−19 C`
    ELEMENTARY_CHARGE => [
        <| join_arrays!(_COULOMB, [
            constant!(1.602176634e-19)
        ]),
        aliases = ["e"],
        metric = true
    ],
    /// `9.64853321233100184e4 C`
    FARADAY => [
        <| join_arrays!(_COULOMB, [
            constant!(9.64853321233100184e4)
        ]),
        aliases = ["F"],
        metric = true
    ],
    AMPERE_HOUR => [
        <| [
            Unit::new(&electric_current::AMPERE, 1.0, 0.0),
            Unit::new(&time::HOUR, 1.0, 0.0),
        ],
        aliases = ["Ah"],
        metric = true
    ]
}
