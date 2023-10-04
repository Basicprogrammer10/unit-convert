use crate::impl_units;

impl_units! {
    ElectricCurrent => {
        Ampere => [
            <| |a| a,
            |> |a| a,
            aliases = ["a", "amp", "ampere"],
            metric = true
        ]
    }
}
