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
            <| |hk| hk * 0.903,
            |> |cd| cd / 0.903,
            aliases = ["hk"]
        ]
    }
}
