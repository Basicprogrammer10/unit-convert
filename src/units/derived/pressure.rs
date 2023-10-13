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
        description = "The unit of pressure in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Pascal_(unit)",
        aliases = ["Pa"],
        metric = true
    ],
    /// `6.894757 kPa`
    PSI => [
        <| join_arrays!(_PASCAL, [
            constant!(6.894757, 3.0)
        ]),
        description = "A unit of measurement of pressure or of stress based on avoirdupois units. Equal to 6.894757 kPa.",
        link = "https://en.wikipedia.org/wiki/Pound_per_square_inch"
    ],
    /// `100 kPa`
    BAR => [
        <| join_arrays!(_PASCAL, [
            constant!(100.0, 3.0)
        ]),
        description = "Exactly 100,000 Pa.",
        link = "https://en.wikipedia.org/wiki/Bar_(unit)",
        metric = true
    ],
    /// `101.325 kPa`
    ATM => [
        <| join_arrays!(_PASCAL, [
            constant!(101.325, 3.0)
        ]),
        description = "Equal to 101325 Pa.",
        link = "https://en.wikipedia.org/wiki/Standard_atmosphere_(unit)"
    ],
    /// `133.3224 Pa`
    TORR => [
        <| join_arrays!(_PASCAL, [
            constant!(133.322, 0.0)
        ]),
        description = "Exactly 1/760 of a standard atmosphere (101325 Pa). Thus one torr is exactly 101325/760 pascals (≈ 133.32 Pa).",
        link = "https://en.wikipedia.org/wiki/Torr"
    ]
}
