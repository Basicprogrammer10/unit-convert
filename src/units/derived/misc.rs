use crate::{dimension::Unit, impl_derived_units, units::time};

impl_derived_units! {
    /// `s^{-1}`
    Hz => [
        <| [
            Unit::new(&time::Second, -1.0, 0.0)
        ],
        metric = true
    ]
}
