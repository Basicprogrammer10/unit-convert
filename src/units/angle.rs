use std::f64::consts;

use crate::impl_units;

impl_units! {
    ANGLE => {
        RADIAN => [
            <| |r| r,
            |> |r| r,
            aliases = ["rad"],
            metric = true
        ],
        STERADIAN => [
            <| |s| s,
            |> |s| s,
            aliases = ["sr"],
            metric = true
        ],
        TURN => [
            <| |t| t * 2.0 * consts::PI,
            |> |t| t / (2.0 * consts::PI),
            aliases = ["tr", "pla"],
            metric = true
        ],
        DEGREE =>[
            <| |d| d * consts::PI / 180.0,
            |> |d| d * 180.0 / consts::PI,
            aliases = ["deg"]
        ],
        GRADIAN => [
            <| |g| g * consts::PI / 200.0,
            |> |g| g * 200.0 / consts::PI,
            aliases = ["grad", "gon"],
            metric = true
        ]
    }
}
