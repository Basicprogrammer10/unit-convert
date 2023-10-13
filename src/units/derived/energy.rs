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
        description = "Unit of energy in the International System of Units. It is equal to the amount of work done when a force of 1 newton displaces a mass through a distance of 1 metre in the direction of the force applied.",
        link = "https://en.wikipedia.org/wiki/Joule",
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
        description = "Unit of power or radiant flux in the International System of Units. Equal to 1 joule per second or 1 kg*m^2*s^{-3}",
        link = "https://en.wikipedia.org/wiki/Watt",
        aliases = ["W"],
        metric = true
    ],
    /// `1055.06 J`
    /// IT thermal unit.
    BTU => [
        <| join_arrays!(_JOULE, [
            constant!(1055.06)
        ]),
        description = "British thermal unit. 1,054.35 J.",
        link = "https://en.wikipedia.org/wiki/British_thermal_unit",
        metric = true
    ],
    /// `4.184 J`
    /// Thermochemical calorie.
    CALORIE => [
        <| join_arrays!(_JOULE, [
            constant!(4.184)
        ]),
        description = "Unit of energy that originated from the obsolete caloric theory of heat. Equal to 4.184 J.",
        link = "https://en.wikipedia.org/wiki/Calorie",
        aliases = ["cal"],
        metric = true
    ],
    /// `1.602176634×10−19 J`
    ELECTRONVOLT => [
        <| join_arrays!(_JOULE, [
            constant!(1.602176634e-19)
        ]),
        description = "The measure of an amount of kinetic energy gained by a single electron accelerating from rest through an electric potential difference of one volt in vacuum. Equal to 1.602176634*10^{-19} J.",
        link = "https://en.wikipedia.org/wiki/Electronvolt",
        aliases = ["eV"],
        metric = true
    ],
    /// `10−7 J`
    ERG => [
        <| join_arrays!(_JOULE, [
            constant!(1e-7)
        ]),
        description = "From the Centimetre-gram-second system of units. Equal to 10^{-7} J.",
        link = "",
        metric = true
    ]
}
