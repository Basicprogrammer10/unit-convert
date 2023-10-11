use std::f64::consts;

use crate::impl_units;

impl_units! {
    ANGLE => {
        RADIAN => [
            <| |r| r,
            |> |r| r,
            description = "The unit of angle in the International System of Units. Defined such that one radian is the angle subtended at the centre of a circle by an arc that is equal in length to the radius",
            link = "https://en.wikipedia.org/wiki/Radian",
            aliases = ["rad"],
            metric = true
        ],
        STERADIAN => [
            <| |s| s,
            |> |s| s,
            description = "Also known as the square radian. The unit of solid angle in the International System of Units. It is used in three dimensional geometry, and is analogous to the radian",
            link = "https://en.wikipedia.org/wiki/Steradian",
            aliases = ["sr"],
            metric = true
        ],
        TURN => [
            <| |t| t * 2.0 * consts::PI,
            |> |t| t / (2.0 * consts::PI),
            description = "Unit of plane angle measurement equal to 2π radians.",
            link = "https://en.wikipedia.org/wiki/Turn_(angle)",
            aliases = ["tr", "pla"],
            metric = true
        ],
        DEGREE =>[
            <| |d| d * consts::PI / 180.0,
            |> |d| d * 180.0 / consts::PI,
            description = "Measurement of a plane angle in which one full rotation is 360 degrees.",
            link = "https://en.wikipedia.org/wiki/Degree_(angle)",
            aliases = ["deg"]
        ],
        GRADIAN => [
            <| |g| g * consts::PI / 200.0,
            |> |g| g * 200.0 / consts::PI,
            description = "Unit of measurement of an angle, defined as one-hundredth of the right angle.",
            link = "https://en.wikipedia.org/wiki/Gradian",
            aliases = ["grad", "gon"],
            metric = true
        ]
    }
}
