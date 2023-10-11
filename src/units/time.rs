use crate::impl_units;

impl_units! {
    TIME => {
        SECOND => [
            <| |s| s,
            |> |s| s,
            description = "9192631770 oscillations of the unperturbed ground-state hyperfine transition frequency of the caesium 133 atom.",
            link = "https://en.wikipedia.org/wiki/Second",
            aliases = ["s", "sec"],
            metric = true
        ],
        MINUTE => [
            <| |m| m * 60.0,
            |> |s| s / 60.0,
            description = "60 seconds.",
            link = "https://en.wikipedia.org/wiki/Minute",
            aliases = ["min"]
        ],
        HOUR => [
            <| |h| h * 3600.0,
            |> |s| s / 3600.0,
            description = "60 minutes.",
            link = "https://en.wikipedia.org/wiki/Hour",
            aliases = ["h", "hr"]
        ],
        DAY => [
            <| |d| d * 86400.0,
            |> |s| s / 86400.0,
            description = "On average 24 hours. (Exactly 24 hours in this converter)",
            link = "https://en.wikipedia.org/wiki/Day",
            aliases = ["D"]
        ],
        WEEK => [
            <| |w| w * 604800.0,
            |> |s| s / 604800.0,
            description = "7 days.",
            link = "https://en.wikipedia.org/wiki/Week",
            aliases = ["wk"]
        ],
        SOL => [
            <| |s| s * 88740.244,
            |> |s| s / 88740.244,
            description = "Apparent interval between two successive returns of the Sun to the same meridian. Equal to 88740.244 seconds.",
            link = "https://en.wikipedia.org/wiki/Solar_time"
        ],
        JULIAN_YEAR => [
            <| |jy| jy * 31_557_600.0,
            |> |s| s / 31_557_600.0,
            description = "The average length of the year in the Julian calendar, 365.25 days.",
            link = "https://en.wikipedia.org/wiki/Julian_year_(astronomy)",
            aliases = ["julian year"]
        ],
        FORTNIGHT => [
            <| |ftn| ftn * 1209600.0,
            |> |s| s / 1209600.0,
            description = "Equal to 14 days (two weeks).",
            link = "https://en.wikipedia.org/wiki/Fortnight",
            aliases = ["ftn"]
        ],
        PLANCK_TIME => [
            <| |tp| tp * 5.391247e-44,
            |> |s| s / 5.391247e-44,
            description = "The time required for light to travel a distance of 1 Planck length in vacuum.",
            link = "https://en.wikipedia.org/wiki/Planck_units#Planck_time",
            aliases = ["planck time"]
        ],
        #[cfg(feature = "wacky_units")]
        ATOM => [
            <| |atom| atom * 0.15957,
            |> |s| s / 0.15957,
            description = "The shortest possible division of time- to the ancient Greek. Its about equal to 160 milliseconds",
            link = "https://en.wikipedia.org/wiki/Atom_(time)"
        ],
        // TODO: description and link
        #[cfg(feature = "wacky_units")]
        MARTIAN_VERNAL_EQUINOX_YEAR => [
            <| |mvey| mvey * 59264867.1384,
            |> |s| s / 59264867.1384,
            aliases = ["martian vernal equinox year"]
        ],
        #[cfg(feature = "wacky_units")]
        GHURRY => [
            <| |ghurry| ghurry * 1440.0,
            |> |s| s / 1440.0,
            description = "Derived from a Middle Age timekeeping divide. Equal to 24 minutes.",
            link = "https://en.wikipedia.org/wiki/Ghurry"
        ],
        #[cfg(feature = "wacky_units")]
        LUSTRE => [
            <| |lustre| lustre * 157788000.0,
            |> |s| s / 157788000.0,
            description = "From Ancient Rome, a lustrum was a five year period at the end of which a full census of the Roman population would be carried out.",
            link = "https://en.wikipedia.org/wiki/Lustrum"
        ],
        // TODO: Is this 8 or 9 days
        #[cfg(feature = "wacky_units")]
        NUNDINE => [
            <| |nundine| nundine * 777600.0,
            |> |s| s / 777600.0,
            description = "From Ancient Rome, a nundine was a period of 8 days.",
            link = "https://en.wikipedia.org/wiki/Nundinae"
        ],
        #[cfg(feature = "wacky_units")]
        PUNCT => [
            <| |punct| punct * 900.0,
            |> |s| s / 900.0,
            description = "From Ancient Rome, a punct was a period of 15 minutes."
        ],
        #[cfg(feature = "wacky_units")]
        QUADRANT => [
            <| |quadrant| quadrant * 21600.0,
            |> |s| s / 21600.0,
            description = "One quarter of a 24 hour day. Equal to 6 hours."
        ],
        #[cfg(feature = "wacky_units")]
        QUINZIEME => [
            <| |quinzieme| quinzieme * 1296000.0,
            |> |s| s / 1296000.0,
            description = "Literally means 'fifteenth' in French, and is equal to 15 days.",
            aliases = ["quinzième"]
        ],
        // TODO: Find info on this
        #[cfg(feature = "wacky_units")]
        JUBILEE => [
            <| |jubilee| jubilee * 1577880000.0,
            |> |s| s / 1577880000.0
        ],
        SIDEREAL_DAY => [
            <| |sday| sday * 86164.0891217,
            |> |s| s / 86164.0891217,
            description = "The time for a single rotation of the earth on its axis in reference to any star or to the vernal equinox at the meridian. Equal to 23 hours, 56 minutes, 4.09 seconds.",
            link = "https://en.wikipedia.org/wiki/Sidereal_time#Sidereal_day",
            aliases = ["sidereal day"]
        ],
        SHAKE => [
            <| |shake| shake * 1e-8,
            |> |s| s / 1e-8,
            description = "Informal metric unit of time equal to 10 nanoseconds. Originally used in nuclear physics mostly with neutron reactions",
            link = "https://en.wikipedia.org/wiki/Shake_(unit)"
        ],
        #[cfg(feature = "wacky_units")]
        JIFFY => [
            <| |jiffy| jiffy * 3.33564e-11,
            |> |s| s / 3.33564e-11,
            description = "Has many informal definitions. Here its the time it takes light to travel one centimeter in a vacuum. Equal to 33.3564 picoseconds.",
            link = "https://en.wikipedia.org/wiki/Jiffy_(time)"
        ],
        GALACTIC_YEAR => [
            <| |gay| gay * 1944e10, // 🏳️‍🌈
            |> |s| s / 1944e10,
            description = "The time it takes the Sun to orbit the center of the Milky Way galaxy. Equal to 19440000 million years.",
            link = "https://en.wikipedia.org/wiki/Galactic_year",
            aliases = ["galactic year"]
        ],
        #[cfg(feature = "wacky_units")]
        KERMIT => [
            <| |kermit| kermit * 864.0,
            |> |s| s / 864.0,
            description = "KerMetric time is a concept that divides the day into 100 equal parts called kermits. Each kermit is equivalent to 14.4 minutes.",
            link = "https://en.wikipedia.org/wiki/List_of_unusual_units_of_measurement#KerMetric_time"
        ],
        #[cfg(feature = "wacky_units")]
        THIRD => [
            <| |third| third / 60.0,
            |> |s| s * 60.0,
            description = "An extrapolation of the word second (2nd division of an hour), equal to 1/60 of a second.",
            link = "https://en.wikipedia.org/wiki/List_of_unusual_units_of_measurement#Thirds,_fourths"
        ],
        #[cfg(feature = "wacky_units")]
        FOURTH => [
            <| |fourth| fourth / 3600.0,
            |> |s| s * 3600.0,
            description = "An extrapolation of the word second (2nd division of an hour), equal to 1/3600 of a second.",
            link = "https://en.wikipedia.org/wiki/List_of_unusual_units_of_measurement#Thirds,_fourths"
        ]
    }
}
