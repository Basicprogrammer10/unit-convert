use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{angle, derived::constant, length, luminous_intensity},
};

const _LUX: [Unit; 3] = [
    Unit::new(&luminous_intensity::CANDELA, 1.0, 0.0),
    Unit::new(&angle::STERADIAN, 1.0, 0.0),
    Unit::new(&length::METER, -2.0, 0.0),
];

impl_derived_units! {
    /// `cd*sr*m^{-2}`
    LUX => [
        <| _LUX,
        description = "The unit of illuminance, or luminous flux per unit area, in the International System of Units. Equal to one lumen per square metre.",
        link = "https://en.wikipedia.org/wiki/Lux",
        aliases = ["lx"]
    ],
    /// 10000 lx
    PHOT => [
        <| join_arrays!(_LUX, [
            constant!(10000.0)
        ]),
        description = "Equal to 10000 lx. From the centimetre-gram-second system of units.",
        link = "https://en.wikipedia.org/wiki/Phot",
        aliases = ["ph"]
    ],
    /// `10.76 lux`
    FOOT_CANDLE => [
        <| join_arrays!(_LUX, [
            constant!(10.763910416709722)
        ]),
        description = "Defined as one lumen per square foot.",
        link = "https://en.wikipedia.org/wiki/Foot-candle",
        aliases = ["fc"]
    ]
}
