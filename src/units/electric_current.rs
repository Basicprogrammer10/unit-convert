use crate::impl_units;

impl_units! {
    ELECTRIC_CURRENT => {
        AMPERE => [
            <| |a| a,
            |> |a| a,
            aliases = ["A", "amp"],
            metric = true
        ]
    }
}
