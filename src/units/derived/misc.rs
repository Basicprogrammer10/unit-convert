use crate::{
    dimension::Unit,
    impl_derived_units,
    units::{length, mass, time},
};

impl_derived_units! {
    /// `s^{-1}`
    Hz => [
        <| [
            Unit::new(&time::Second, -1.0, 0.0)
        ],
        metric = true
    ],
    /// `kg*m^2*s^{−2}`
    Joule => [
        <| [
            Unit::new(&mass::Gram, 1.0, 3.0),
            Unit::new(&length::Meter, 2.0, 0.0),
            Unit::new(&time::Second, -2.0, 0.0)
        ],
        aliases = ["J"],
        metric = true
    ]
}
