use crate::impl_units;

impl_units! {
    TEMPERATURE => {
        KELVIN => [
            <| |k| k,
            |> |k| k,
            description = "An absolute scale, defined such that 0 K is absolute zero and a change of thermodynamic temperature T by 1 kelvin corresponds to a change of thermal energy kT by 1.380649*10^{-23} J",
            link = "https://en.wikipedia.org/wiki/Kelvin",
            aliases = ["k", "degk"]
        ],
        CELSIUS => [
            <| |c| c + 273.15,
            |> |k| k - 273.15,
            description = "(x + 273.15) K",
            link = "https://en.wikipedia.org/wiki/Celsius",
            aliases = ["c", "degc"]
        ],
        FAHRENHEIT => [
            <| |f| (f - 32.0) * 5.0 / 9.0 + 273.15,
            |> |k| (k - 273.15) * 9.0 / 5.0 + 32.0,
            description = "5/9(x + 459.67) K",
            link = "https://en.wikipedia.org/wiki/Fahrenheit",
            aliases = ["f", "degf"]
        ],
        RANKINE => [
            <| |r| r * 5.0 / 9.0,
            |> |k| k * 9.0 / 5.0,
            description = "5/9 x K",
            link = "https://en.wikipedia.org/wiki/Rankine_scale",
            aliases = ["r", "degr"]
        ]
    }
}
