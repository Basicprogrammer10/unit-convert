use crate::impl_units;

impl_units! {
    LENGTH => {
        METER => [
            <| |m| m,
            |> |m| m,
            aliases = ["m", "metre"],
            metric = true
        ],
        INCH => [
            <| |i| i * 0.0254,
            |> |m| m / 0.0254,
            aliases = ["in"]
        ],
        THOU => [
            <| |th| th * 0.0000254,
            |> |m| m / 0.0000254,
            aliases = ["mil"]
        ],
        FOOT => [
            <| |f| f * 0.3048,
            |> |m| m / 0.3048,
            aliases = ["ft", "feet"]
        ],
        YARD => [
            <| |y| y * 0.9144,
            |> |m| m / 0.9144,
            aliases = ["yd"]
        ],
        STATUTE_MILE => [
            <| |mi| mi * 1609.344,
            |> |m| m / 1609.344,
            aliases = ["mi", "statute mile", "mile"]
        ],
        LEAGUE => [
            <| |l| l * 4828.0417,
            |> |m| m / 4828.0417
        ],
        ASTRONMICAL_UNIT => [
            <| |au| au * 1.495978707e11,
            |> |m| m / 1.495978707e11,
            aliases = ["au", "astronomical unit"]
        ],
        SIRIOMETER => [
            <| |sir| sir * 1.495978707e17,
            |> |m| m / 1.495978707e17,
            aliases = ["sir"]
        ],
        LIGHT_YEAR => [
            <| |ly| ly * 9460730472580800.0,
            |> |m| m / 9460730472580800.0,
            aliases = ["ly", "light-year"]
        ],
        PARSEC => [
            <| |pc| pc * 3.0856776e16,
            |> |m| m / 3.0856776e16,
            aliases = ["pc"]
        ],
        #[cfg(feature = "wacky_units")]
        POTRZEBIE => [
            <| |p| p * 0.0022633484517438173216473,
            |> |m| m / 0.0022633484517438173216473
        ],
        FURLONG => [
            <| |fur| fur * 201.168,
            |> |m| m / 201.168,
            aliases = ["fur"]
        ],
        PLANCK_LENGTH => [
            <| |lp| lp * 1.616255e-35,
            |> |m| m / 1.616255e-35,
            aliases = ["planck length"]
        ],
        ROD => [
            <| |rod| rod * 5.0292,
            |> |m| m / 5.0292
        ],
        NAUTICAL_MILE => [
            <| |nmi| nmi * 1852.0,
            |> |m| m / 1852.0,
            aliases = ["nmi", "nautical mile"]
        ],
        HAMMER_UNIT => [
            <| |qu| qu * 0.01905,
            |> |m| m / 0.01905,
            aliases = ["qu", "hammer unit"]
        ],
        RACK_UNIT => [
            <| |u| u * 0.04445,
            |> |m| m / 0.04445,
            aliases = ["U", "rack unit"]
        ],
        HAND => [
            <| |hh| hh * 0.1016,
            |> |m| m / 0.1016,
            aliases = ["hh"]
        ],
        LIGHT_SECOND => [
            <| |ls| ls * 299792458.0,
            |> |m| m / 299792458.0,
            aliases = ["ls", "light-second"]
        ],
        EARTH_RADIUS => [
            <| |re| re * 6.3781e6,
            |> |m| m / 6.3781e6,
            aliases = ["earth radius"]
        ],
        LUNAR_DISTANCE => [
            <| |ld| ld * 3.84399e8,
            |> |m| m / 3.84399e8,
            aliases = ["LD", "lunar distance"]
        ],
        #[cfg(feature = "wacky_units")]
        SMOOT => [
            <| |s| s * 1.7,
            |> |m| m / 1.7
        ],
        #[cfg(feature = "wacky_units")]
        MEGALITHIC_YARD => [
            <| |megalithic_yard| megalithic_yard * 0.83,
            |> |m| m / 0.83,
            aliases = ["megalithic yard"]
        ],
        #[cfg(feature = "wacky_units")]
        FINGER_BREADTH => [
            <| |fingerbreadth| fingerbreadth * 0.01905,
            |> |m| m / 0.01905
        ],
        #[cfg(feature = "wacky_units")]
        DOUBLE_DECKER_BUS => [
            <| |ddb| ddb * 18.75,
            |> |m| m / 18.75,
            aliases = ["double-decker bus"]
        ],
        BARLEYCORN => [
            <| |barley| barley * 0.00846667,
            |> |m| m / 0.00846667
        ],
        #[cfg(feature = "wacky_units")]
        MICKEY => [
            <| |mickey| mickey * 1.27e-4,
            |> |m| m / 1.27e-4
        ],
        #[cfg(feature = "wacky_units")]
        NAIL => [
            <| |nail| nail * 0.05715,
            |> |m| m / 0.05715
        ],
        #[cfg(feature = "wacky_units")]
        ALTUVE => [
            <| |altuve| altuve * 1.68,
            |> |m| m / 1.68
        ],
        METRIC_INCH => [
            <| |metrinch| metrinch * 0.025,
            |> |m| m / 0.025,
            aliases = ["metric inch"]
        ],
        METIC_FOOT => [
            <| |metrifoot| metrifoot * 0.3,
            |> |m| m / 0.3,
            aliases = ["metric foot"]
        ],
        METRIC_CHAIN => [
            <| |metrichain| metrichain * 20.0,
            |> |m| m / 20.0,
            aliases = ["metric chain"]
        ],
        METRIC_LIEUE => [
            <| |metrilieue| metrilieue * 4000.0,
            |> |m| m / 4000.0,
            aliases = ["metric lieue"]
        ],
        SCANDINAVIAN_MILE => [
            <| |scanmile| scanmile * 10000.0,
            |> |m| m / 10000.0,
            aliases = ["scandinavian mile"]
        ],
        FERMI => [
            <| |fm| fm * 1e-15,
            |> |m| m / 1e-15,
            aliases = ["fm"]
        ],
        PICA => [
            <| |pica| pica * 0.0042333,
            |> |m| m / 0.0042333
        ]
    }
}
