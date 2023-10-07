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
        MILE => [
            <| |mi| mi * 1609.344,
            |> |m| m / 1609.344,
            aliases = ["mi"]
        ],
        LEAGUE => [
            <| |l| l * 4828.0417,
            |> |m| m / 4828.0417
        ]
    }
}
