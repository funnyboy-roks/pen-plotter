use std::io;

use scad::prelude::*;

fn main() -> io::Result<()> {
    var! {
        let width = 130;
        let gap_len = 11.5;
        let dia = 4;
        => vars
    }

    let sled_width = gap_len + dia + 10;
    let sled = {
        Cube::with_size((4, sled_width, 100))
            - hull![
                Cylinder::with_diameter(dia, 50).rotate((0, 90, 0)),
                Cylinder::with_diameter(dia, 50)
                    .rotate((0, 90, 0))
                    .translate((0, gap_len + dia, 0)),
            ]
            .translate((-5, 5, 50))
    };

    let holder = {
        let wall_thicc = ScadValue::from(2);
        let height = ScadValue::from(80);
        // let width = ScadValue::from(40);
        let width = sled_width + 0.5 + wall_thicc * 2;
        let motor_width = ScadValue::from(23);
        let motor_height = ScadValue::from(12);
        let depth = ScadValue::from(5.75);
        let hold = {
            Cube::with_size((10, sled_width + 0.5 + wall_thicc * 2, 10))
                - Cube::with_size((5, sled_width + 0.5, 10.02)).translate((2.5 + 0.001, 2.5, -0.01))
        }
        .translate((0, (width - (sled_width + 0.5 + wall_thicc * 2)) / 2., 0));
        Cube::with_size((5, width, height))
            + hold.translate((5, 0, 0))
            + hold.translate((5, 0, height - 10))
            - Cylinder::with_diameters(40, 60, 10)
                .rotate((0, 90, 0))
                .translate((2.5, width / 2, height / 2))
            + Cube::with_size((depth, width, wall_thicc + motor_height + wall_thicc)).translate((
                -depth,
                0,
                (height - (wall_thicc + motor_height + wall_thicc)) / 2.,
            ))
            - Cube::with_size((depth, 100, motor_height)).translate((
                -depth - 0.01 - 2,
                -0.01 + width / 2 - 12 / 2,
                wall_thicc + (height - (wall_thicc + motor_height + wall_thicc)) / 2.,
            ))
            - {
                Cylinder::with_diameter(12, 50)
                    + hull![
                        Cylinder::with_diameter(6, 50).translate((0, 6.5, 0)),
                        Cylinder::with_diameter(6, 50),
                    ]
            }
            .rotate((0, 90, 0))
            .translate((-5, width / 2., height / 2.))
    };

    Scad::builder()
        .number_of_segments(100)
        .objects(&sled)
        .objects(&holder.translate((100, 0, 0)))
        .variables(vars)
        .build()
        .to_scad(&mut io::stdout().lock())?;

    Ok(())
}
