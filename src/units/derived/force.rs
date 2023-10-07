use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::constant, length, mass, time},
};

const NEWTON: [Unit; 3] = [
    Unit::new(&mass::Gram, 1.0, 3.0),
    Unit::new(&length::Meter, 1.0, 0.0),
    Unit::new(&time::Second, -2.0, 0.0),
];

impl_derived_units! {
    /// `kg*m*s^{−2}`
    Newton => [
        <| NEWTON,
        aliases = ["N"],
        metric = true
    ],
    /// `4.448222*N`
    PoundForce => [
        <| join_arrays!(NEWTON, [
            constant!(4.448222)
        ]),
        aliases = ["lbf"],
        metric = true
    ],
    /// `N*10^{-5}`
    Dyne => [
        <| join_arrays!(NEWTON, [
            constant!(1e-5, 0.0)
        ]),
        aliases = ["dyn"]
    ],
    /// `N*10^3`
    Kip => [
        <| join_arrays!(NEWTON, [
            constant!(4448.222)
        ])
    ],
    Kilopond => [
        <| join_arrays!(NEWTON, [
            constant!(9.80665)
        ])
    ],
    /// `lb*ft*s^{−2}`
    Poundal => [
        <| [
            Unit::new(&mass::Pound, 1.0, 0.0),
            Unit::new(&length::Foot, 1.0, 0.0),
            Unit::new(&time::Second, -2.0, 0.0)
        ],
        aliases = ["pdl"]
    ]
}
