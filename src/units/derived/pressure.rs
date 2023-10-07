use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, length, mass, time},
};

const _PASCAL: [Unit; 3] = [
    Unit::new(&mass::GRAM, 1.0, 3.0),
    Unit::new(&length::METER, -1.0, 0.0),
    Unit::new(&time::SECOND, -2.0, 0.0),
];

impl_derived_units! {
    /// `kg*m^{−1}*s^{−2}`
    PASCAL => [
        <| _PASCAL,
        aliases = ["Pa"],
        metric = true
    ],
    /// `6.894757 kPa`
    PSI => [
        <| join_arrays!(_PASCAL, [
            constant!(6.894757, 3.0)
        ]),
        metric = true
    ],
    /// `100 kPa`
    BAR => [
        <| join_arrays!(_PASCAL, [
            constant!(100.0, 3.0)
        ]),
        metric = true
    ],
    /// `101.325 kPa`
    ATM => [
        <| join_arrays!(_PASCAL, [
            constant!(101.325, 3.0)
        ])
    ],
    /// `133.3224 Pa`
    TORR => [
        <| join_arrays!(_PASCAL, [
            constant!(133.322, 0.0)
        ])
    ]
}
