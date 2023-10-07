use crate::impl_units;

impl_units! {
    LUMINOUS_INTENSITY => {
        CANDELA => [
            <| |cd| cd,
            |> |cd| cd,
            aliases = ["cd"],
            metric = true
        ],
        HEFNERKERZE => [
            <| |hk| hk * 0.92,
            |> |hk| hk / 0.92,
            aliases = ["hk"]
        ]
    }
}
