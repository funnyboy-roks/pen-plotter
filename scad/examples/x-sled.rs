use std::io;

use scad::{dim::_3D, prelude::*};

fn main() -> io::Result<()> {
    var! {
        let dia = 15.3;
        /// Gap between wall and centre of rod cutout
        let inset = 20;
        let gap = 100;
        let thicc = 24;
        let cut_thicc = 1;
        => vars
    }

    let width = gap + inset * 2;

    let screw = |len: u32| {
        let len = ScadValue::from(len);
        Cylinder::with_diameter(7.5, 15).translate((0, 0, len/2 - 3.5 / 2.)) // head + washer
            + Cylinder::with_diameter(3.3, len + 1).center() // shaft
            + Cylinder::with_diameter(6.8, 15) // nut
                // .rotate((0, 0, 30))
                .number_of_segments(6)
                .translate((0, 0, -len / 2 - 15 + 3.5 / 2.))
    };

    let bush_clip = {
        Cube::with_size((thicc, dia + inset / 2, inset)).translate((0, 0, inset))
        + Cylinder::with_diameter(inset * 2, thicc)
            .rotate((0, 90, 0))
            .translate((0, inset, inset))
        // rod
        - Cylinder::with_diameter(dia, thicc *4)
            .center()
            .rotate((0, 90, 0))
            .translate((-5, inset, inset))
        // clamp
        - Cube::with_size((thicc * 4, cut_thicc, inset))
            .translate((-0.01, inset - cut_thicc / 2, -0.01))
        - screw(15)
            .rotate((90, 0, 0))
            .translate((thicc / 2, inset, (inset - dia / 2) / 2))
    };

    let belt_clip = {
        Cube::with_size((thicc, 8, inset + 6)).translate((0, width / 2 - 8 / 2 + 9, inset - 6))
            - screw(15)
                .rotate((90, 0, 0))
                .translate((thicc / 2, width / 2 - 8 / 2 + 9, inset))
    };

    let cup = {
        (&bush_clip)
            + (&bush_clip)
                .translate((0, -inset, 0))
                .scale((1, -1, 1))
                .translate((0, gap + inset, 0))
            + Cube::with_size((thicc, width, 6)).translate((0, 0, inset * 2 - 6))
            + belt_clip
    };

    Scad::builder()
        .number_of_segments(100)
        .objects(&cup)
        .variables(vars)
        .build()
        .to_scad(&mut io::stdout().lock())?;

    Ok(())
}
