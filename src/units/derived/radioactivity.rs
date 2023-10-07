use crate::{dimension::Unit, impl_derived_units, units::time};

use super::constant;

impl_derived_units! {
    /// `s^{-1}`
    BECQUEREL => [
        <| [
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        aliases = ["Bq"],
        metric = true
    ],
    /// `37 GBq`
    CURIE => [
        <| [
            constant!(37e9, 0.0),
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        aliases = ["Ci"],
        metric = true
    ],
    /// `1 MBq`
    RUTHERFORD => [
        <| [
            constant!(1e6, 0.0),
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        aliases = ["Rd"],
        metric = true
    ]
}
