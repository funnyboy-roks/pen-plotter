use std::io;

use scad::prelude::*;

fn main() -> io::Result<()> {
    var! {
        let radius = 40.;
        => vars
    }

    let ext = (Rectangle::with_size((20.1, 20.1))
        - Rectangle::with_size((6.5, 6)).translate((20.1 / 2. - 6.5 / 2., -0.01))
        - Rectangle::with_size((6.5, 6)).translate((20.1 / 2. - 6.5 / 2., 20.1 - 6. + 0.01))
        - Rectangle::with_size((6, 6.5)).translate((-0.01, 20.1 / 2. - 6.5 / 2.))
        - Rectangle::with_size((6, 6.5)).translate((20.1 - 6. + 0.01, 20.1 / 2. - 6.5 / 2.)))
    .linear_extrude(100);
    let cup = (Cube::with_size((40, 40, 80))
        - ext.translate((40. / 2. - 20.1 / 2., 40. / 2. - 20.1 / 2., 40)))
        + (Cube::with_size((40, 40, 80))
            - ext.translate((40. / 2. - 20.1 / 2., 40. / 2. - 20.1 / 2., 40)))
        .rotate((0, 90, 0))
        .translate((0, 0, 40));

    Scad::builder()
        .number_of_segments(200)
        .objects(&cup)
        .variables(vars)
        .build()
        .to_scad(&mut io::stdout().lock())?;

    Ok(())
}
