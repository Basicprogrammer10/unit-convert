use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{
        angle, derived::constant, electric_current, length, luminous_intensity, mass, quantity,
        time,
    },
};

const _WEBER: [Unit; 4] = [
    Unit::new(&mass::GRAM, 1.0, 3.0),
    Unit::new(&length::METER, 2.0, 0.0),
    Unit::new(&time::SECOND, -2.0, 0.0),
    Unit::new(&electric_current::AMPERE, -1.0, 0.0),
];

impl_derived_units! {
    /// `s^{-1}`
    HZ => [
        <| [
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        metric = true
    ],
    /// `kg^{-1}*m^{-2}*s^4*A^2`
    FARAD => [
        <| [
            Unit::new(&mass::GRAM, -1.0, 3.0),
            Unit::new(&length::METER, -2.0, 0.0),
            Unit::new(&time::SECOND, 4.0, 0.0),
            Unit::new(&electric_current::AMPERE, 2.0, 0.0)
        ],
        aliases = ["F"],
        metric = true
    ],
    /// `kg*m^2*s^{-3}*A^{-2}`
    OHM => [
        <| [
            Unit::new(&mass::GRAM, 1.0, 3.0),
            Unit::new(&length::METER, 2.0, 0.0),
            Unit::new(&time::SECOND, -3.0, 0.0),
            Unit::new(&electric_current::AMPERE, -2.0, 0.0)
        ],
        aliases = ["Ω"],
        metric = true
    ],
    /// `Ω^{-1}`
    SIEMENS => [
        <| [
            Unit::new(&mass::GRAM, -1.0, 3.0),
            Unit::new(&length::METER, -2.0, 0.0),
            Unit::new(&time::SECOND, 3.0, 0.0),
            Unit::new(&electric_current::AMPERE, 2.0, 0.0)
        ],
        aliases = ["S"],
        metric = true
    ],
    /// `kg*m^2*s^{-2}*A^{-1}`
    WEBER => [
        <| _WEBER,
        aliases = ["Wb"],
        metric = true
    ],
    /// `10^{-8} Wb`
    MAXWELL => [
        <| join_arrays!(_WEBER, [
            constant!(1e-8, 0.0)
        ]),
        aliases = ["Mx"]
    ],
    /// `kg*s^{−2}*A^{-1}`
    TESLA => [
        <| [
            Unit::new(&mass::GRAM, 1.0, 3.0),
            Unit::new(&time::SECOND, -2.0, 0.0),
            Unit::new(&electric_current::AMPERE, -1.0, 0.0)
        ],
        aliases = ["T"],
        metric = true
    ],
    /// `kg*m^2*s^{-2}*A^{-2}`
    HENRY => [
        <| [
            Unit::new(&mass::GRAM, 1.0, 3.0),
            Unit::new(&length::METER, 2.0, 0.0),
            Unit::new(&time::SECOND, -2.0, 0.0),
            Unit::new(&electric_current::AMPERE, -2.0, 0.0)
        ],
        aliases = ["H"],
        metric = true
    ],
    /// `cd*sr`
    LUMEN => [
        <| [
            Unit::new(&luminous_intensity::CANDELA, 1.0, 0.0),
            Unit::new(&angle::STERADIAN, 1.0, 0.0)
        ],
        aliases = ["lm"]
    ],
    /// `mol/s`
    KATAL => [
        <| [
            Unit::new(&quantity::MOLE, 1.0, 0.0),
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        aliases = ["kat"],
        metric = true
    ]
}
