use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, length, mass, time},
};

const PASCAL: [Unit; 3] = [
    Unit::new(&mass::Gram, 1.0, 3.0),
    Unit::new(&length::Meter, -1.0, 0.0),
    Unit::new(&time::Second, -2.0, 0.0),
];

impl_derived_units! {
    /// `kg*m^{−1}*s^{−2}`
    Pascal => [
        <| PASCAL,
        aliases = ["Pa"],
        metric = true
    ],
    /// `6.894757 kPa`
    Psi => [
        <| join_arrays!(PASCAL, [
            constant!(6.894757, 3.0)
        ]),
        metric = true
    ],
    /// `100 kPa`
    Bar => [
        <| join_arrays!(PASCAL, [
            constant!(100.0, 3.0)
        ]),
        metric = true
    ],
    /// `101.325 kPa`
    Atm => [
        <| join_arrays!(PASCAL, [
            constant!(101.325, 3.0)
        ])
    ],
    /// `133.3224 Pa`
    Torr => [
        <| join_arrays!(PASCAL, [
            constant!(133.322, 0.0)
        ])
    ]
}
