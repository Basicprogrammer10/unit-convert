use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, length, mass, time},
};

const _JOULE: [Unit; 3] = [
    Unit::new(&mass::GRAM, 1.0, 3.0),
    Unit::new(&length::METER, 2.0, 0.0),
    Unit::new(&time::SECOND, -2.0, 0.0),
];

impl_derived_units! {
    /// `kg*m^2*s^{−2}`
    JOULE => [
        <| _JOULE,
        aliases = ["J"],
        metric = true
    ],
    /// `kg*m^2*s^{−3}`
    WATT => [
        <| [
            Unit::new(&mass::GRAM, 1.0, 3.0),
            Unit::new(&length::METER, 2.0, 0.0),
            Unit::new(&time::SECOND, -3.0, 0.0),
        ],
        aliases = ["W"],
        metric = true
    ],
    /// `1055.06 J`
    /// IT thermal unit.
    BTU => [
        <| join_arrays!(_JOULE, [
            constant!(1055.06)
        ]),
        metric = true
    ],
    /// `4.184 J`
    /// Thermochemical calorie.
    CALORIE => [
        <| join_arrays!(_JOULE, [
            constant!(4.184)
        ]),
        aliases = ["cal"],
        metric = true
    ],
    /// `1.602176634×10−19 J`
    ELECTRONVOLT => [
        <| join_arrays!(_JOULE, [
            constant!(1.602176634e-19)
        ]),
        aliases = ["eV"],
        metric = true
    ],
    /// `10−7 J`
    ERG => [
        <| join_arrays!(_JOULE, [
            constant!(1e-7)
        ]),
        metric = true
    ],
    /// The `lbf` part of the Foot-Pound (`ft*lbf`).
    POUND_FORCE => [
        <| [
            Unit::new(&length::FOOT, 2.0, 1.0),
            Unit::new(&time::SECOND, -2.0, 0.0),
        ]
    ]
}
