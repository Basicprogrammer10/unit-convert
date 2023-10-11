use crate::impl_units;

impl_units! {
    LENGTH => {
        METER => [
            <| |m| m,
            |> |m| m,
            description = "The base unit of length in the International System of Units.",
            link = "https://en.wikipedia.org/wiki/Metre",
            aliases = ["m", "metre"],
            metric = true
        ],
        INCH => [
            <| |i| i * 0.0254,
            |> |m| m / 0.0254,
            description = "Unit of length in the British imperial and the United States customary systems of measurement. It is equal to 1/36 yard or 1/12 of a foot.",
            link = "https://en.wikipedia.org/wiki/Inch",
            aliases = ["in", "\""]
        ],
        THOU => [
            <| |th| th * 0.0000254,
            |> |m| m / 0.0000254,
            description = "A thousandth of an inch.",
            link = "https://en.wikipedia.org/wiki/Thou",
            aliases = ["mil"]
        ],
        FOOT => [
            <| |f| f * 0.3048,
            |> |m| m / 0.3048,
            description = "12 inches.",
            link = "https://en.wikipedia.org/wiki/Foot_(unit)",
            aliases = ["ft", "feet", "'"]
        ],
        YARD => [
            <| |y| y * 0.9144,
            |> |m| m / 0.9144,
            description = "Three feet.",
            link = "https://en.wikipedia.org/wiki/Yard",
            aliases = ["yd"]
        ],
        STATUTE_MILE => [
            <| |mi| mi * 1609.344,
            |> |m| m / 1609.344,
            description = "Based on the older English unit of length equal to 5,280 English feet, or 1,760 yards.",
            link = "https://en.wikipedia.org/wiki/Mile",
            aliases = ["mi", "statute mile", "mile"]
        ],
        // TODO: Is this the english league?
        LEAGUE => [
            <| |l| l * 4828.0417,
            |> |m| m / 4828.0417,
            description = "Was common in Europe and Latin America, but is no longer an official unit in any nation. May have originally represented, roughly, the distance a person could walk in an hour.",
            link = "https://en.wikipedia.org/wiki/League_(unit)"
        ],
        ASTRONMICAL_UNIT => [
            <| |au| au * 1.495978707e11,
            |> |m| m / 1.495978707e11,
            description = "Roughly the distance from Earth to the Sun and approximately equal to 150 million kilometres (93 million miles) or 8.3 light-minutes",
            link = "https://en.wikipedia.org/wiki/Astronomical_unit",
            aliases = ["au", "astronomical unit"]
        ],
        SIRIOMETER => [
            <| |sir| sir * 1.495978707e17,
            |> |m| m / 1.495978707e17,
            description = "Obsolete astronomical unit of length, defined to be equal to one million astronomical units.",
            link = "https://en.wikipedia.org/wiki/Siriometer",
            aliases = ["sir"]
        ],
        LIGHT_YEAR => [
            <| |ly| ly * 9460730472580800.0,
            |> |m| m / 9460730472580800.0,
            description = "Used to express astronomical distances and is equivalent to about 9.46 trillion kilometers. Defined as the distance that light travels in a vacuum in one Julian year (365.25 days).",
            link = "https://en.wikipedia.org/wiki/Light-year",
            aliases = ["ly", "light-year"]
        ],
        PARSEC => [
            <| |pc| pc * 3.0856776e16,
            |> |m| m / 3.0856776e16,
            description = "Used to measure the large distances to astronomical objects outside the Solar System, approximately equal to 3.26 light-years or 206,265 astronomical units.",
            link = "https://en.wikipedia.org/wiki/Parsec",
            aliases = ["pc"]
        ],
        #[cfg(feature = "wacky_units")]
        POTRZEBIE => [
            <| |p| p * 0.0022633484517438173216473,
            |> |m| m / 0.0022633484517438173216473,
            description = "The thickness of Mad issue 26, or 2.2633484517438173216473 mm. How very silly.",
            link = "https://en.wikipedia.org/wiki/List_of_humorous_units_of_measurement#Potrzebie"
        ],
        FURLONG => [
            <| |fur| fur * 201.168,
            |> |m| m / 201.168,
            description = "Equal to one eighth of a mile, equivalent to any of 660 feet, 220 yards, 40 rods, 10 chains or approximately 201 metres",
            link = "https://en.wikipedia.org/wiki/Furlong",
            aliases = ["fur"]
        ],
        PLANCK_LENGTH => [
            <| |lp| lp * 1.616255e-35,
            |> |m| m / 1.616255e-35,
            description = "It is equal to 1.616255(18)*10^{-35} m. Since the 1950s, it has been conjectured that quantum fluctuations of the spacetime metric might make the familiar notion of distance inapplicable below the Planck length.",
            link = "https://en.wikipedia.org/wiki/Planck_units#Planck_length",
            aliases = ["planck length"]
        ],
        ROD => [
            <| |rod| rod * 5.0292,
            |> |m| m / 5.0292,
            description = "Defined as 16+1/2 feet, equal to exactly 1/320 of a mile, or 5+1/2 yards (a quarter of a surveyor's chain), and is exactly 5.0292 meters.",
            link = "https://en.wikipedia.org/wiki/Rod_(unit)",
            aliases = ["perch", "pole", "lug"]
        ],
        NAUTICAL_MILE => [
            <| |nmi| nmi * 1852.0,
            |> |m| m / 1852.0,
            description = "Used in air, marine, and space navigation, and for the definition of territorial waters. Defined as 1,852 metres (about 6,076 ft; 1.151 mi).",
            link = "https://en.wikipedia.org/wiki/Nautical_mile",
            aliases = ["nmi", "nautical mile"]
        ],
        #[cfg(feature = "wacky_units")]
        HAMMER_UNIT => [
            <| |qu| qu * 0.01905,
            |> |m| m / 0.01905,
            description = "Valve's Source game engine uses the Hammer unit as its base unit of length. The exact definition varies from game to game, but a Hammer unit is usually defined as a sixteenth of a foot.",
            link = "https://en.wikipedia.org/wiki/List_of_unusual_units_of_measurement#Hammer_unit",
            aliases = ["qu", "hammer unit"]
        ],
        RACK_UNIT => [
            <| |u| u * 0.04445,
            |> |m| m / 0.04445,
            description = "Equal to 1.75 inches (44.45 mm). Used to measure rack-mountable audiovisual, computing and industrial equipment.",
            link = "https://en.wikipedia.org/wiki/List_of_unusual_units_of_measurement#Rack_unit",
            aliases = ["U", "rack unit"]
        ],
        HAND => [
            <| |hh| hh * 0.1016,
            |> |m| m / 0.1016,
            description = "Equal to exactly 4 inches (101.6 mm). It is normally used to measure the height of horses.",
            link = "https://en.wikipedia.org/wiki/Hand_(unit)",
            aliases = ["hh"]
        ],
        LIGHT_SECOND => [
            <| |ls| ls * 299792458.0,
            |> |m| m / 299792458.0,
            description = "The distance that light travels in free space in one second. Equal to exactly 299792458 m metres.",
            link = "https://en.wikipedia.org/wiki/Light-second",
            aliases = ["ls", "light-second"]
        ],
        EARTH_RADIUS => [
            <| |re| re * 6.3781e6,
            |> |m| m / 6.3781e6,
            description = "The globally-average radius of Earth, generally given as 6,371 kilometres.",
            link = "https://en.wikipedia.org/wiki/List_of_unusual_units_of_measurement#Earth_radius",
            aliases = ["earth radius"]
        ],
        LUNAR_DISTANCE => [
            <| |ld| ld * 3.84399e8,
            |> |m| m / 3.84399e8,
            description = "The distance from the centre of Earth to the centre of the Moon. Approximately 384,400 km (238,900 mi), or 1.28 light-seconds.",
            link = "https://en.wikipedia.org/wiki/List_of_unusual_units_of_measurement#Lunar_distance",
            aliases = ["LD", "lunar distance"]
        ],
        #[cfg(feature = "wacky_units")]
        SMOOT => [
            <| |s| s * 1.7,
            |> |m| m / 1.7,
            description = "",
            link = ""
        ],
        #[cfg(feature = "wacky_units")]
        MEGALITHIC_YARD => [
            <| |megalithic_yard| megalithic_yard * 0.83,
            |> |m| m / 0.83,
            description = "",
            link = "",
            aliases = ["megalithic yard"]
        ],
        #[cfg(feature = "wacky_units")]
        FINGER_BREADTH => [
            <| |fingerbreadth| fingerbreadth * 0.01905,
            |> |m| m / 0.01905,
            description = "",
            link = ""
        ],
        #[cfg(feature = "wacky_units")]
        DOUBLE_DECKER_BUS => [
            <| |ddb| ddb * 18.75,
            |> |m| m / 18.75,
            description = "",
            link = "",
            aliases = ["double-decker bus"]
        ],
        BARLEYCORN => [
            <| |barley| barley * 0.00846667,
            |> |m| m / 0.00846667,
            description = "",
            link = ""
        ],
        #[cfg(feature = "wacky_units")]
        MICKEY => [
            <| |mickey| mickey * 1.27e-4,
            |> |m| m / 1.27e-4,
            description = "",
            link = ""
        ],
        #[cfg(feature = "wacky_units")]
        NAIL => [
            <| |nail| nail * 0.05715,
            |> |m| m / 0.05715,
            description = "",
            link = ""
        ],
        #[cfg(feature = "wacky_units")]
        ALTUVE => [
            <| |altuve| altuve * 1.68,
            |> |m| m / 1.68,
            description = "",
            link = ""
        ],
        METRIC_INCH => [
            <| |metrinch| metrinch * 0.025,
            |> |m| m / 0.025,
            description = "",
            link = "",
            aliases = ["metric inch"]
        ],
        METIC_FOOT => [
            <| |metrifoot| metrifoot * 0.3,
            |> |m| m / 0.3,
            description = "",
            link = "",
            aliases = ["metric foot"]
        ],
        METRIC_CHAIN => [
            <| |metrichain| metrichain * 20.0,
            |> |m| m / 20.0,
            description = "",
            link = "",
            aliases = ["metric chain"]
        ],
        METRIC_LIEUE => [
            <| |metrilieue| metrilieue * 4000.0,
            |> |m| m / 4000.0,
            description = "",
            link = "",
            aliases = ["metric lieue"]
        ],
        SCANDINAVIAN_MILE => [
            <| |scanmile| scanmile * 10000.0,
            |> |m| m / 10000.0,
            description = "",
            link = "",
            aliases = ["scandinavian mile"]
        ],
        FERMI => [
            <| |fm| fm * 1e-15,
            |> |m| m / 1e-15,
            description = "",
            link = "",
            aliases = ["fm"]
        ],
        PICA => [
            <| |pica| pica * 0.0042333,
            |> |m| m / 0.0042333,
            description = "",
            link = ""
        ]
    }
}
