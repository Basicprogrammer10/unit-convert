use crate::impl_units;

// todo: mark `#[cfg(feature = "wacky_units")]`

impl_units! {
    TIME => {
        SECOND => [
            <| |s| s,
            |> |s| s,
            aliases = ["s", "sec"],
            metric = true
        ],
        MINUTE => [
            <| |m| m * 60.0,
            |> |s| s / 60.0,
            aliases = ["min"]
        ],
        HOUR => [
            <| |h| h * 3600.0,
            |> |s| s / 3600.0,
            aliases = ["h", "hr"]
        ],
        WEEK => [
            <| |w| w * 604800.0,
            |> |s| s / 604800.0,
            aliases = ["wk"]
        ],
        /// Apparent interval between two successive returns of the Sun to the same meridian
        SOL => [
            <| |s| s * 88740.244,
            |> |s| s / 88740.244
        ],
        DAY => [
            <| |d| d * 86400.0,
            |> |s| s / 86400.0,
            aliases = ["D"]
        ],
        JULIAN_YEAR => [
            <| |jy| jy * 31557600.0,
            |> |s| s / 31557600.0,
            aliases = ["julian year"]
        ],
        FORTNIGHT => [
            <| |ftn| ftn * 1209600.0,
            |> |s| s / 1209600.0,
            aliases = ["ftn"]
        ],
        PLANCK_TIME => [
            <| |tp| tp * 5.391247e-44,
            |> |s| s / 5.391247e-44,
            aliases = ["planck time"]
        ],
        ATOM => [
            <| |atom| atom * 0.15957,
            |> |s| s / 0.15957
        ],
        MARTIAN_VERNAL_EQUINOX_YEAR => [
            <| |mvey| mvey * 59264867.1384,
            |> |s| s / 59264867.1384,
            aliases = ["martian vernal equinox year"]
        ],
        GHURRY => [
            <| |ghurry| ghurry * 1440.0,
            |> |s| s / 1440.0
        ],
        LUSTRE => [
            <| |lustre| lustre * 157788000.0,
            |> |s| s / 157788000.0
        ],
        NUNDINE => [
            <| |nundine| nundine * 777600.0,
            |> |s| s / 777600.0
        ],
        PUNCT => [
            <| |punct| punct * 900.0,
            |> |s| s / 900.0
        ],
        QUADRANT => [
            <| |quadrant| quadrant * 21600.0,
            |> |s| s / 21600.0
        ],
        ///This one needs an accent
        QUINZIEME => [
            <| |quinzieme| quinzieme * 1296000.0,
            |> |s| s / 1296000.0
        ],
        JUBILEE => [
            <| |jubilee| jubilee * 1577880000.0,
            |> |s| s / 1577880000.0
        ],
        SIDEREAL_DAY => [
            <| |sday| sday * 86164.0891217,
            |> |s| s / 86164.0891217,
            aliases = ["sidereal day"]
        ],
        SHAKE => [
            <| |shake| shake * 1e-8,
            |> |s| s / 1e-8
        ],
        JIFFY => [
            <| |jiffy| jiffy * 3.33564e-11,
            |> |s| s / 3.33564e-11
        ],
        GALACTIC_YEAR => [
            <| |gay| gay * 19440000e6,
            |> |s| s / 19440000e6,
            aliases = ["galactic year"]
        ],
        KERMIT => [
            <| |kermit| kermit * 864.0,
            |> |s| s / 864.0
        ],
        THIRD => [
            <| |third| third / 60.0,
            |> |s| s * 60.0
        ],
        FOURTH => [
            <| |fourth| fourth / 3600.0,
            |> |s| s * 3600.0
        ]
    }
}
