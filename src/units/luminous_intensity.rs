use crate::impl_units;

impl_units! {
    LuminousIntensity => {
        Candela => [
            <| |cd| cd,
            |> |cd| cd,
            aliases = ["cd"],
            metric = true
        ],
        Hefnerkerze => [
            <| |hk| hk * 0.92,
            |> |hk| hk / 0.92,
            aliases = ["hk"]
        ]
    }
}
