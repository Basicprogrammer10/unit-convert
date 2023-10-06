use crate::{
    dimension::Unit,
    impl_derived_units, join_arrays,
    units::{derived::VarNum, length, mass, time},
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
            Unit::new(&VarNum::new(4.448222), 1.0, 0.0)
        ]),
        aliases = ["lbf"],
        metric = true
    ],
    /// `N*10^{-5}`
    Dyne => [
        <| join_arrays!(NEWTON, [
            Unit::new(&VarNum::new(1e-5), 1.0, 0.0),
        ]),
        aliases = ["dyn"]
    ],
    /// `N*10^3`
    Kip => [
        <| join_arrays!(NEWTON, [
            Unit::new(&VarNum::new(4448.222), 1.0, 0.0),
        ])
    ],
    Kilopond => [
        <| join_arrays!(NEWTON, [
            Unit::new(&VarNum::new(9.80665), 1.0, 0.0),
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
