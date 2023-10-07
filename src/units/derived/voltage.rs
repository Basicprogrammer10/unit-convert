use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, electric_current, length, mass, time},
};

const _VOLT: [Unit; 4] = [
    Unit::new(&mass::GRAM, 1.0, 3.0),
    Unit::new(&length::METER, 2.0, 0.0),
    Unit::new(&time::SECOND, -3.0, 0.0),
    Unit::new(&electric_current::AMPERE, -1.0, 0.0),
];

impl_derived_units! {
    /// `kg*m^2*s^{-3}*A^{-1}
    VOLT => [
        <| _VOLT,
        aliases = ["V"],
        metric = true
    ],
    /// `10^−6 V`
    STATVOLT => [
        <| join_arrays!(_VOLT, [
            constant!(299.792458)
        ]),
        aliases = ["statV"],
        metric = true
    ]
}
