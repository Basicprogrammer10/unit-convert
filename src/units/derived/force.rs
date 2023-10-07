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
        aliases = ["N"],
        metric = true
    ],
    /// `N*10^{-5}`
    DYNE => [
        <| join_arrays!(_NEWTON, [
            constant!(1e-5, 0.0)
        ]),
        aliases = ["dyn"]
    ],
    /// `N*10^3`
    KIP => [
        <| join_arrays!(_NEWTON, [
            constant!(4448.222)
        ])
    ],
    KILOPOND => [
        <| join_arrays!(_NEWTON, [
            constant!(9.80665)
        ])
    ],
    /// `lb*ft*s^{−2}`
    POUNDAL => [
        <| [
            Unit::new(&mass::POUND, 1.0, 0.0),
            Unit::new(&length::FOOT, 1.0, 0.0),
            Unit::new(&time::SECOND, -2.0, 0.0)
        ],
        aliases = ["pdl"]
    ]
}
