use std::io;

use scad::prelude::*;

fn main() -> io::Result<()> {
    var! {
        let dia = 8.3;
        let height = 21;
        let inset = 15;
        let width = 130;
        => vars
    }

    let screw = Cylinder::with_diameter(7.5, 10).translate((5, 0, height - 5)) // head + washer
        + Cylinder::with_diameter(3.25, height * 2).translate((5, 0, -5)) // shaft
        + Cylinder::with_diameter(6.8, 10) // nut
            .number_of_segments(6)
            .translate((5, 0, -5));

    let screw = &screw;

    let slot_len = 10;
    let rod_clip = Cube::with_size((10, width, height))
        - Cylinder::with_diameter(dia, 20)
            .rotate((0, 90, 0))
            .translate((-5, inset, height / 2))
            // rod
        - Cylinder::with_diameter(dia, 20)
            .rotate((0, 90, 0))
            .translate((-5, width - inset, height / 2)) // rod
        - Cube::with_size((20, inset, 1)).translate((0, -0.01, height / 2 - 1)) // clamp
        - Cube::with_size((20, inset, 1)).translate((0, width - inset + 0.01, height / 2 - 1)) // clamp
        - screw.translate((0, 7.5, 0))
        - screw.translate((0, width - 7.5, 0))
        // Idle holder
        + {
            hull![
                Cylinder::with_diameter(8, height + 15).translate((slot_len, 0, 0)),
                Cylinder::with_diameter(15, height + 15).translate((-5, 0, 0)),
            ].translate((0, 0, -5))
            - hull![
                Cylinder::with_diameter(5.5, height * 2),
                Cylinder::with_diameter(5.5, height * 2).translate((slot_len, 0, 0)),
            ]
            - Cube::with_size((50, 30, 22)) // belt cutout
                .translate((-5. + 0.01, -15, height / 2 - 22. / 2. + 5.))
        }
        .translate((10 + 5, width / 2, -5))
        // Wedge control
        + {
            let screw = |len: u32| {
                let len = ScadValue::from(len);
                Cylinder::with_diameter(7.5, 15).translate((0, 0, len/2 - 3.5 / 2.)) // head + washer
                    + Cylinder::with_diameter(3.3, len + 1).center() // shaft
                    + Cylinder::with_diameter(6.8, 15) // nut
                        // .rotate((0, 0, 30))
                        .number_of_segments(6)
                        .translate((0, 0, -len / 2 - 15 + 3.5 / 2.))
            };
            Cube::with_size((20, 5, height)) - screw(10).rotate((-90, 0, 0)).translate((10, 5, height / 2))
        }
        .translate((10, width / 2 + 40, 0))
        - Cube::with_size((50, 30, 12)) // belt cutout
            .translate((-5, width / 2 - 30. / 2., height / 2 - 12. / 2.));

    let main = rod_clip.translate((0, 0, 45.5))
        + (Cube::with_size((48, 48, 48))
            - Cube::with_size((48, 48 + 5, 100)).translate((10. - 0.01, -0.01, -0.01)))
        .translate((0, width / 2. - 48. / 2., 0));

    let extrusion_clip = {
        let btm_width = 35.;
        Cube::with_size((48, btm_width, 23)).translate((0, width / 2. - btm_width / 2., 0))
            - Cube::with_size((50, 20.5, 20.5)).translate((-1, width / 2. - 20.5 / 2., -0.01))
            - Cylinder::with_diameter(5, 100)
                .rotate((-90, 0, 0))
                .translate((48. / 2., 0, 20.5 / 2.))
            - Cylinder::with_diameter(11, 50)
                .rotate((90, 0, 0))
                .translate((48. / 2., width / 2. - 20.5 / 2. - 3, 20.5 / 2.))
            - Cylinder::with_diameter(11, 50)
                .rotate((-90, 0, 0))
                .translate((48. / 2., width / 2. + 20.5 / 2. + 3, 20.5 / 2.))
    };

    let cup = main.translate((0, 0, 23)) + extrusion_clip;

    let idler_wedge = {
        let cutout_height = 10.;
        hull![
            Cube::with_size((0.1, 0.1, height)),
            Cube::with_size((slot_len + 10, 0.1, height)).translate((0, 40, 0)),
        ] - Cube::with_size((50, 40 - 5, cutout_height)).translate((
            -5,
            -0.01,
            height / 2. - cutout_height / 2.,
        ))
    };

    Scad::builder()
        .number_of_segments(100)
        .objects(&cup)
        .objects(&idler_wedge.translate((100, 0, 0) /*(10, 65, 45.5)*/))
        .variables(vars)
        .build()
        .to_scad(&mut io::stdout().lock())?;

    Ok(())
}
