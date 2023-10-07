use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::VarNum, electric_current, length, mass, time},
};

const VOLT: [Unit; 4] = [
    Unit::new(&mass::Gram, 1.0, 3.0),
    Unit::new(&length::Meter, 2.0, 0.0),
    Unit::new(&time::Second, -3.0, 0.0),
    Unit::new(&electric_current::Ampere, -1.0, 0.0),
];

impl_derived_units! {
    /// `kg*m^2*s^{-3}*A^{-1}
    Volt => [
        <| VOLT,
        aliases = ["V"],
        metric = true
    ],
    /// `10^−6 V`
    Statvolt => [
        <| join_arrays!(VOLT, [
            Unit::new(&VarNum::new(299.792458), 1.0, 0.0),
        ]),
        aliases = ["statV"],
        metric = true
    ]
}
