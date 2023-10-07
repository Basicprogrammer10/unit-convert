use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::VarNum, length, mass, time},
};

const JOULE: [Unit; 3] = [
    Unit::new(&mass::Gram, 1.0, 3.0),
    Unit::new(&length::Meter, 2.0, 0.0),
    Unit::new(&time::Second, -2.0, 0.0),
];

impl_derived_units! {
    /// `kg*m^2*s^{−2}`
    Joule => [
        <| JOULE,
        aliases = ["J"],
        metric = true
    ],
    /// `kg*m^2*s^{−3}`
    Watt => [
        <| [
            Unit::new(&mass::Gram, 1.0, 3.0),
            Unit::new(&length::Meter, 2.0, 0.0),
            Unit::new(&time::Second, -3.0, 0.0),
        ],
        aliases = ["W"],
        metric = true
    ],
    /// `1055.06 J`
    /// IT thermal unit.
    Btu => [
        <| join_arrays!(JOULE, [
            Unit::new(&VarNum::new(1055.06), 1.0, 0.0),
        ]),
        metric = true
    ],
    /// `4.184 J`
    /// Thermochemical calorie.
    Calorie => [
        <| join_arrays!(JOULE, [
            Unit::new(&VarNum::new(4.184), 1.0, 0.0),
        ]),
        aliases = ["cal"],
        metric = true
    ],
    /// `1.602176634×10−19 J`
    Electronvolt => [
        <| join_arrays!(JOULE, [
            Unit::new(&VarNum::new(1.602176634e-19), 1.0, 0.0),
        ]),
        aliases = ["eV"],
        metric = true
    ],
    /// `10−7 J`
    Erg => [
        <| join_arrays!(JOULE, [
            Unit::new(&VarNum::new(1e-7), 1.0, 0.0),
        ]),
        metric = true
    ],
    /// The `lbf` part of the Foot-Pound (`ft*lbf`).
    Lbf => [
        <| [
            Unit::new(&length::Foot, 2.0, 1.0),
            Unit::new(&time::Second, -2.0, 0.0),
        ]
    ]
}
