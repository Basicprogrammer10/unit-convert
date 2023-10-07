use crate::{dimension::Unit, impl_derived_units, units::time};

impl_derived_units! {
    /// `s^{-1}`
    HZ => [
        <| [
            Unit::new(&time::SECOND, -1.0, 0.0)
        ],
        metric = true
    ]
}
