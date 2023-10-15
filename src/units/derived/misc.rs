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
        description = "The unit of frequency in the International System of Units. Equal to 1 cycle per second.",
        link = "https://en.wikipedia.org/wiki/Hertz",
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
        description = "The unit of electrical capacitance in the International System of Units. Equal to 1 coulomb per volt.",
        link = "https://en.wikipedia.org/wiki/Farad",
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
        description = "The unit of electrical resistance in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Ohm",
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
        description = "The unit of electric conductance, electric susceptance, and electric admittance in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Siemens_(unit)",
        aliases = ["S", "mho", "℧"],
        metric = true
    ],
    /// `kg*m^2*s^{-2}*A^{-1}`
    WEBER => [
        <| _WEBER,
        description = "The unit of magnetic flux in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Weber_(unit)",
        aliases = ["Wb"],
        metric = true
    ],
    /// `10^{-8} Wb`
    MAXWELL => [
        <| join_arrays!(_WEBER, [
            constant!(1e-8, 0.0)
        ]),
        description = "The CGS (centimetre-gram-second) unit of magnetic flux.",
        link = "https://en.wikipedia.org/wiki/Maxwell_%28unit%29",
        aliases = ["Mx"]
    ],
    /// `kg*s^{−2}*A^{-1}`
    TESLA => [
        <| [
            Unit::new(&mass::GRAM, 1.0, 3.0),
            Unit::new(&time::SECOND, -2.0, 0.0),
            Unit::new(&electric_current::AMPERE, -1.0, 0.0)
        ],
        description = "The unit of magnetic flux density (also called magnetic B-field strength) in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Tesla_(unit)",
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
        description = "The unit of electrical inductance in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Henry_(unit)",
        aliases = ["H"],
        metric = true
    ],
    /// `cd*sr`
    LUMEN => [
        <| [
            Unit::new(&luminous_intensity::CANDELA, 1.0, 0.0),
            Unit::new(&angle::STERADIAN, 1.0, 0.0)
        ],
        description = "The unit of luminous flux, a measure of the total quantity of visible light emitted by a source per unit of time, in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Lumen_(unit)",
        aliases = ["lm"]
    ],
    /// `mol/s`
    KATAL => [
        <| [
            Unit::new(&quantity::MOLE, 1.0, 0.0),
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        description = "The unit of catalytic activity in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Katal",
        aliases = ["kat"],
        metric = true
    ]
}
