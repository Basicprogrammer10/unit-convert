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
        aliases = ["lx"]
    ],
    /// 10000 lx
    PHOT => [
        <| join_arrays!(_LUX, [
            constant!(10000.0)
        ]),
        aliases = ["ph"]
    ],
    /// `10.76 lux`
    FOOT_CANDLE => [
        <| join_arrays!(_LUX, [
            constant!(10.763910416709722)
        ]),
        aliases = ["fc"]
    ]
}
