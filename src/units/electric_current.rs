use crate::impl_units;

impl_units! {
    ELECTRIC_CURRENT => {
        AMPERE => [
            <| |a| a,
            |> |a| a,
            description = "The unit of electric current in the International System of Units. Equal to 1 coulomb.",
            link = "https://en.wikipedia.org/wiki/Ampere",
            aliases = ["A", "amp"],
            metric = true
        ]
    }
}
