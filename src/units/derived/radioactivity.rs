use crate::{dimension::Unit, impl_derived_units, units::time};

use super::constant;

impl_derived_units! {
    /// `s^{-1}`
    BECQUEREL => [
        <| [
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        description = "The unit of radioactivity in the International System of Units.",
        link = "https://en.wikipedia.org/wiki/Becquerel",
        aliases = ["Bq"],
        metric = true
    ],
    /// `37 GBq`
    CURIE => [
        <| [
            constant!(37e9, 0.0),
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        description = "Originally defined as the quantity or mass of radium emanation in equilibrium with one gram of radium (element), but is currently defined as 1 Ci = 3.7*10^{10} decays per second.",
        link = "https://en.wikipedia.org/wiki/Curie_(unit)",
        aliases = ["Ci"],
        metric = true
    ],
    /// `1 MBq`
    RUTHERFORD => [
        <| [
            constant!(1e6, 0.0),
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        description = "Defined as the activity of a quantity of radioactive material in which one million nuclei decay per second.",
        link = "https://en.wikipedia.org/wiki/Rutherford_(unit)",
        aliases = ["Rd"],
        metric = true
    ]
}
