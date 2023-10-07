use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, electric_current, time},
};

const COULOMB: [Unit; 2] = [
    Unit::new(&electric_current::Ampere, 1.0, 0.0),
    Unit::new(&time::Second, 1.0, 0.0),
];

impl_derived_units! {
    Coulomb => [
        <| COULOMB,
        aliases = ["C"],
        metric = true
    ],
    /// `1.602176634×10−19 C`
    ElementaryCharge => [
        <| join_arrays!(COULOMB, [
            constant!(1.602176634e-19)
        ]),
        aliases = ["e"],
        metric = true
    ],
    /// `9.64853321233100184e4 C`
    Faraday => [
        <| join_arrays!(COULOMB, [
            constant!(9.64853321233100184e4)
        ]),
        aliases = ["F"],
        metric = true
    ],
    AmpereHour => [
        <| [
            Unit::new(&electric_current::Ampere, 1.0, 0.0),
            Unit::new(&time::Hour, 1.0, 0.0),
        ],
        aliases = ["Ah"],
        metric = true
    ]
}
