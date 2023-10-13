use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, electric_current, time},
};

const _COULOMB: [Unit; 2] = [
    Unit::new(&electric_current::AMPERE, 1.0, 0.0),
    Unit::new(&time::SECOND, 1.0, 0.0),
];

impl_derived_units! {
    COULOMB => [
        <| _COULOMB,
        description = "The unit of electric charge in the International System of Units. Equal to the electric charge delivered by a 1 ampere constant current in 1 second.",
        link = "https://en.wikipedia.org/wiki/Coulomb",
        aliases = ["C"],
        metric = true
    ],
    /// `1.602176634×10−19 C`
    ELEMENTARY_CHARGE => [
        <| join_arrays!(_COULOMB, [
            constant!(1.602176634e-19)
        ]),
        description = "The electric charge carried by a single proton. 1.602176634*10^{-19} C",
        link = "https://en.wikipedia.org/wiki/Elementary_charge",
        aliases = ["e"],
        metric = true
    ],
    // TODO: Per mol?
    /// `9.64853321233100184e4 C`
    FARADAY => [
        <| join_arrays!(_COULOMB, [
            constant!(9.64853321233100184e4)
        ]),
        description = "Defined as the quotient of the total electric charge (q) by the amount (n) of elementary charge carriers in any given sample of matter.",
        link = "https://en.wikipedia.org/wiki/Faraday_constant",
        metric = true
    ],
    AMPERE_HOUR => [
        <| [
            Unit::new(&electric_current::AMPERE, 1.0, 0.0),
            Unit::new(&time::HOUR, 1.0, 0.0),
        ],
        description = "Equal to the charge transferred by a steady current of one ampere flowing for one hour, or 3,600 coulombs.",
        link = "https://en.wikipedia.org/wiki/Ampere-hour",
        aliases = ["Ah"],
        metric = true
    ]
}
