use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, length, mass, time},
};

const _NEWTON: [Unit; 3] = [
    Unit::new(&mass::GRAM, 1.0, 3.0),
    Unit::new(&length::METER, 1.0, 0.0),
    Unit::new(&time::SECOND, -2.0, 0.0),
];

impl_derived_units! {
    /// `kg*m*s^{−2}`
    NEWTON => [
        <| _NEWTON,
        description = "The unit of force in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Newton_(unit)",
        aliases = ["N"],
        metric = true
    ],
    /// `4.448222*N`
    POUND_FORCE => [
        <| join_arrays!(_NEWTON, [
            constant!(4.448222)
        ]),
        description = "Equal to 4.448222 N. Originally defined as the gravitational force exerted on a mass of one avoirdupois pound on the surface of Earth.",
        link = "https://en.wikipedia.org/wiki/Pound_(force)",
        aliases = ["lbf"],
        metric = true
    ],
    /// `N*10^{-5}`
    DYNE => [
        <| join_arrays!(_NEWTON, [
            constant!(1e-5, 0.0)
        ]),
        description = "Equal to 10^{-5} N. From the the centimetre-gram-second system.",
        link = "https://en.wikipedia.org/wiki/Dyne",
        aliases = ["dyn"]
    ],
    /// `N*10^3`
    KIP => [
        <| join_arrays!(_NEWTON, [
            constant!(4448.222)
        ]),
        description = "Equal to 1000 pounds-force. Used primarily by structural engineers.",
        link = ""
    ],
    KILOPOND => [
        <| join_arrays!(_NEWTON, [
            constant!(9.80665)
        ]),
        description = "Equal to the magnitude of the force exerted on one kilogram of mass in a 9.80665 m/s2 gravitational field.",
        link = "https://en.wikipedia.org/wiki/Kilogram-force",
        aliases = ["kilogram force"]
    ],
    /// `lb*ft*s^{−2}`
    POUNDAL => [
        <| [
            Unit::new(&mass::POUND, 1.0, 0.0),
            Unit::new(&length::FOOT, 1.0, 0.0),
            Unit::new(&time::SECOND, -2.0, 0.0)
        ],
        description = "The force necessary to accelerate 1 pound-mass at 1 foot per second per second. Equal to 0.138254954376 N.",
        link = "https://en.wikipedia.org/wiki/Poundal",
        aliases = ["pdl"]
    ]
}
