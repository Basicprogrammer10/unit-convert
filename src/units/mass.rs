use crate::impl_units;

impl_units! {
    MASS => {
        // TODO: Should I change this to kg?
        // It would require updating lots of units and derived units and possibly introducing bugs.
        GRAM => [
            <| |g| g,
            |> |g| g,
            description = "Equal to one one thousandth of a kilogram.",
            link = "https://en.wikipedia.org/wiki/Gram",
            aliases = ["g"],
            metric = true
        ],
        TONNE => [
            <| |t| t * 1000000.0,
            |> |g| g / 100000.0,
            description = "Equal to 1000 kilograms.",
            link = "https://en.wikipedia.org/wiki/Tonne",
            aliases = ["t"]
        ],
        POUND => [
            <| |lb| lb * 453.59237,
            |> |g| g / 453.59237,
            description = "Legally defined as exactly 0.45359237 kilograms.",
            link = "https://en.wikipedia.org/wiki/Pound_(mass)",
            aliases = ["lb"]
        ]
    }
}
