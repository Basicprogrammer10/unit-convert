use crate::impl_units;

impl_units! {
    TEMPERATURE => {
        KELVIN => [
            <| |k| k,
            |> |k| k,
            aliases = ["k", "degk"]
        ],
        CELSIUS => [
            <| |c| c + 273.15,
            |> |k| k - 273.15,
            aliases = ["c", "degc"]
        ],
        FAHRENHEIT => [
            <| |f| (f - 32.0) * 5.0 / 9.0 + 273.15,
            |> |k| (k - 273.15) * 9.0 / 5.0 + 32.0,
            aliases = ["f", "degf"]
        ],
        RANKINE => [
            <| |r| r * 5.0 / 9.0,
            |> |k| k * 9.0 / 5.0,
            aliases = ["r", "degr"]
        ]
    }
}
