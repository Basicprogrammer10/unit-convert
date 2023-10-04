use crate::impl_units;

impl_units! {
    Temperature => {
        Kelvin => [
            <| |k| k,
            |> |k| k,
            aliases = ["k", "kelvin", "degk"]
        ],
        Celsius => [
            <| |c| c + 273.15,
            |> |k| k - 273.15,
            aliases = ["c", "celsius", "degc"]
        ],
        Fahrenheit => [
            <| |f| (f - 32.0) * 5.0 / 9.0 + 273.15,
            |> |k| (k - 273.15) * 9.0 / 5.0 + 32.0,
            aliases = ["f", "fahrenheit", "degf"]
        ],
        Rankine => [
            <| |r| r * 5.0 / 9.0,
            |> |k| k * 9.0 / 5.0,
            aliases = ["r", "rankine", "degr"]
        ]
    }
}
