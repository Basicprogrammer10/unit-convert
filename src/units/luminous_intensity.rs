use crate::impl_units;

impl_units! {
    LUMINOUS_INTENSITY => {
        CANDELA => [
            <| |cd| cd,
            |> |cd| cd,
            description = "The unit of luminous intensity in the International System of Units.",
            link = "https://en.wikipedia.org/wiki/Candela",
            aliases = ["cd"],
            metric = true
        ],
        HEFNERKERZE => [
            <| |hk| hk * 0.903,
            |> |cd| cd / 0.903,
            description = "Defined by the luminous flux, which radiates an amyl acetate lamp designed by the engineer Friedrich von Hefner-Alteneck, the Hefner lamp, at 40 mm flame height and 8 mm wick diameter in a horizontal direction.",
            link = "https://de.wikipedia.org/wiki/Hefnerkerze",
            aliases = ["hk"]
        ]
    }
}
