use std::io;

use scad::prelude::*;

fn main() -> io::Result<()> {
    var! {
        let dia = 8.3;
        let height = 21;
        let inset = 15;
        let gap = 75;
        => vars
    }

    let width = inset + gap + inset;

    let screw = Cylinder::with_diameter(7.5, 10).translate((5, 0, height - 5)) // head + washer
        + Cylinder::with_diameter(3.2, height * 2).translate((5, 0, -5)) // shaft
        + Cylinder::with_diameter(6.7, 10) // nut
            .number_of_segments(6)
            .translate((5, 0, -5));

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
        - (&screw).translate((0, 7.5, 0))
        - (&screw).translate((0, width - 7.5, 0))
        - Cube::with_size((25, 30, 12))
            .translate((-5, width / 2 - 30. / 2., height / 2 - 12. / 2.));

    let main = {
        rod_clip.translate((0, 0, 45.5))
            + (Cube::with_size((48, 48, 38. + 5.0)) // x motor shell
            - Cube::with_size((45.5, 42.5, 38.0)).translate((48. - 42.5) / 2.)
            - hull![ // top cutout
                Cylinder::with_diameter(22.5, 50)
                    .center()
                    .translate(48. / 2.),
                Cylinder::with_diameter(22.5, 50)
                    .center()
                    .translate((48., 48. / 2., 48. / 2.)),
            ]
            - {
                let diameter = ScadValue::from(22.5);
                let radius = diameter / 2;
                hull![ // side cutouts
                    Cylinder::with_diameter(diameter, 50)
                        .center()
                        .rotate((90, 0, 0))
                        .translate((radius + 2.5 + 3., 48. / 2., 0)),
                    Cylinder::with_diameter(22.5, 50)
                        .center()
                        .rotate((90, 0, 0))
                        .translate((48. - radius - 2.5 - 3., 48. / 2., 0)),
                ]
                + hull![ // side cutouts
                    Cylinder::with_diameter(diameter, 50)
                        .center()
                        .rotate((90, 0, 0))
                        .translate((radius + 2.5 + 3., 48. / 2., 0)),
                    Cylinder::with_diameter(22.5, 50)
                        .center()
                        .rotate((90, 0, 0))
                        .translate((48. - radius - 2.5 - 3., 48. / 2., 0)),
                ].rotate((0, 0, 90)).translate((10, 0, 0))
            }.translate((0, 0, (38. + 5.) / 2.))
            - Cylinder::with_diameter(3.5, 15).translate((48. / 2. + 31.5 / 2., 48. / 2. + 31.5 / 2., 38. - 5.)) // screw
            - Cylinder::with_diameter(3.5, 15).translate((48. / 2. + 31.5 / 2., 48. / 2. - 31.5 / 2., 38. - 5.)) // screw
            + Cube::with_size((10, 48, 5)).translate((0, 0, 38. + 5.)))
            .translate((0, width / 2. - 48. / 2., 0))
    };

    let z_offset = 80.;
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
            + Cube::with_size((10, btm_width, z_offset)).translate((
                0,
                width / 2. - btm_width / 2.,
                23,
            ))
            + Cube::with_size((48, 2, z_offset)).translate((0, width / 2. - btm_width / 2. + 2, 23))
            + Cube::with_size((48, 2, z_offset)).translate((
                0,
                width / 2. + btm_width / 2. - 2 - 2,
                23,
            ))
    };
    let cup = main.translate((0, 0, 23. + z_offset)) + extrusion_clip;

    Scad::builder()
        .number_of_segments(100)
        .objects(&cup)
        .variables(vars)
        .build()
        .to_scad(&mut io::stdout().lock())?;

    Ok(())
}
