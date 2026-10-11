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
        // + (hull![
        //     Cylinder::with_diameter(8, height + 5),
        //     Cylinder::with_diameter(15, height + 5).translate((-5, 0, 0)),
        // ] - Cylinder::with_diameter(5.5, height * 2).translate((0, 0, 5)))
        // .translate((10 + 5, width / 2, -5))
        - Cube::with_size((50, 30, 12)) // belt cutout
            .translate((-5, width / 2 - 30. / 2., height / 2 - 12. / 2.));

    let cup = rod_clip.translate((0, 0, 45.5))
        + (Cube::with_size((48, 48, 48))
            - Cube::with_size((48, 48 + 5, 48)).translate((10. - 0.01, -0.01, 10)))
        .translate((0, width / 2. - 48. / 2., 0));

    let idler_holder = {
        let screw = |len: u32| {
            let len = ScadValue::from(len);
            Cylinder::with_diameter(7.5, 15).translate((0, 0, len/2 - 3.5 / 2.)) // head + washer
            + Cylinder::with_diameter(3.3, len + 1).center() // shaft
            + Cylinder::with_diameter(6.8, 15) // nut
                // .rotate((0, 0, 30))
                .number_of_segments(6)
                .translate((0, 0, -len / 2 - 15 + 3.5 / 2.))
        };
        Cube::with_size((5, 50, 25)) + Cylinder::with_diameter(5.2 + 4., 25).translate((0, 25, 0))
            - Cylinder::with_diameter(5.2, 50 * 2)
                .center()
                .translate((0, 25, 0))
            - Cube::with_size((50, 30, 10)).translate((
                -5,
                50. / 2. - 30. / 2.,
                25. / 2. - 10. / 2.,
            ))
            - screw(10)
                .rotate((0, 90, 0))
                .translate((3 + 3, (50. / 2. - 30. / 2.) / 2., 25. / 2.))
            - screw(10).rotate((0, 90, 0)).translate((
                3 + 3,
                50. - (50. / 2. - 30. / 2.) / 2.,
                25. / 2.,
            ))
    };

    let idler_holder_cap = {
        Sphere::with_diameter(5) + Cylinder::with_diameter(5, 2) - Cylinder::with_diameter(3, 6)
    };

    Scad::builder()
        .number_of_segments(100)
        .objects(&cup)
        .objects(&idler_holder.translate((100, 0, 0)))
        .objects(&idler_holder_cap.translate((200, 0, 0)))
        .variables(vars)
        .build()
        .to_scad(&mut io::stdout().lock())?;

    Ok(())
}
