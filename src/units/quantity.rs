use crate::impl_units;

impl_units! {
    QUANTITY => {
        NUMBER => [
            <| |n| n,
            |> |n| n,
            aliases = ["n", "num", "x"],
            metric = true
        ],
        MOLE => [
            <| |m| m * 6.02214076e23,
            |> |n| n / 6.02214076e23,
            description = "One mole contains exactly 6.02214076*10^{23} elementary entities.",
            link = "https://en.wikipedia.org/wiki/Mole_(unit)",
            aliases = ["mol"]
        ]
    }
}
