use std::io;

use scad::prelude::*;

fn main() -> io::Result<()> {
    var! {
        let x = 5;
        => vars
    }

    let cup = {
        Cube::with_size((40 + 20 + 40, 26, 23)).center()
            + Cube::with_size((26, 40 + 20 + 40, 23)).center()
            - Cube::with_size((45 + 20 + 45, 20.5, 20.5))
                .center()
                .translate((0, 0, -(23. - 20.5)))
            - Cube::with_size((20.5, 45 + 20 + 45, 20.5))
                .center()
                .translate((0, 0, -(23. - 20.5)))
            - Cube::with_size((23 + 20 + 23, 23 + 20 + 23, 20.5))
                .center()
                .translate((0, 0, -(23. - 20.5)))
            - Cylinder::with_diameter(5.5, 50).translate((30, 0, 0))
            - Cylinder::with_diameter(5.5, 50).translate((-30, 0, 0))
            - Cylinder::with_diameter(5.5, 50).translate((0, 30, 0))
            - Cylinder::with_diameter(5.5, 50).translate((0, -30, 0))
    };

    Scad::builder()
        .number_of_segments(100)
        .objects(&cup)
        .variables(vars)
        .build()
        .to_scad(&mut io::stdout().lock())?;

    Ok(())
}
