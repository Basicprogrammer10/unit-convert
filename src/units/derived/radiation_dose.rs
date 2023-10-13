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
        description = "Unit of ionizing radiation dose in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Gray_(unit)",
        aliases = ["Gy"]
    ],
    /// `0.01 Gy`
    RADIATION_UNIT => [
        <| join_arrays!(_GRAY, [
            constant!(0.01)
        ]),
        description = "Defined as 1 rad = 0.01 Gy = 0.01 J/kg.",
        link = "https://en.wikipedia.org/wiki/Rad_(radiation_unit)",
        metric = true
    ],
    /// `m^2*s^{-2}`
    SIEVERT => [
        <| _SIEVERT,
        description = "Unit in the International System of Units intended to represent the stochastic health risk of ionizing radiation.",
        link = "https://en.wikipedia.org/wiki/Sievert",
        aliases = ["Sv"]
    ],
    /// `0.01 Sv`
    ROENTGEN_EQUIVALENT_MAN => [
        <| join_arrays!(_SIEVERT, [
            constant!(0.01)
        ]),
        description = "A CGS unit of equivalent dose, effective dose, and committed dose.",
        link = "https://en.wikipedia.org/wiki/Roentgen_equivalent_man",
        aliases = ["rem"],
        metric = true
    ]
}
