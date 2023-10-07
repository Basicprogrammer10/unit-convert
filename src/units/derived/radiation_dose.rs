use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, length, time},
};

const _GRAY: [Unit; 2] = [
    Unit::new(&length::METER, 2.0, 0.0),
    Unit::new(&time::SECOND, -2.0, 0.0),
];

const _SIEVERT: [Unit; 2] = [
    Unit::new(&length::METER, 2.0, 0.0),
    Unit::new(&time::SECOND, -2.0, 0.0),
];

impl_derived_units! {
    /// `m^2*s^{-2}`
    GRAY => [
        <| _GRAY,
        aliases = ["Gy"]
    ],
    /// `0.01 Gy`
    RADIATION_UNIT => [
        <| join_arrays!(_GRAY, [
            constant!(0.01)
        ]),
        metric = true
    ],
    /// `m^2*s^{-2}`
    SIEVERT => [
        <| _SIEVERT,
        aliases = ["Sv"]
    ],
    /// `0.01 Sv`
    ROENTGEN_EQUIVALENT_MAN => [
        <| join_arrays!(_SIEVERT, [
            constant!(0.01)
        ]),
        aliases = ["rem"],
        metric = true
    ]
}
