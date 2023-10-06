use crate::impl_units;

impl_units! {
    ElectricCurrent => {
        Ampere => [
            <| |a| a,
            |> |a| a,
            aliases = ["A", "amp"],
            metric = true
        ]
    }
}
