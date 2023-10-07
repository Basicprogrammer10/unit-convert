use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::VarNum, length, mass, time},
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
            Unit::new(&VarNum::new(6.894757), 1.0, 3.0)
        ]),
        metric = true
    ],
    /// `100 kPa`
    Bar => [
        <| join_arrays!(PASCAL, [
            Unit::new(&VarNum::new(100.0), 1.0, 3.0)
        ]),
        metric = true
    ],
    /// `101.325 kPa`
    Atm => [
        <| join_arrays!(PASCAL, [
            Unit::new(&VarNum::new(101.325), 1.0, 3.0)
        ])
    ],
    /// `133.3224 Pa`
    Torr => [
        <| join_arrays!(PASCAL, [
            Unit::new(&VarNum::new(133.322), 1.0, 0.0)
        ])
    ]
}