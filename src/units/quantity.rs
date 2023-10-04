use crate::impl_units;

impl_units! {
    Quantity => {
        Number => [
            <| |n| n,
            |> |n| n,
            aliases = ["n", "num", "x"],
            metric = true
        ],
        Mole => [
            <| |m| m * 6.02214076e23,
            |> |n| n / 6.02214076e23,
            aliases = ["mol"]
        ]
    }
}
